use quick_xml::{events::Event, Reader};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tuono_lib::Type;

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct PlexLibraryCount {
    pub movies: u32,
    pub shows: u32,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct PlexLibrary {
    pub library_type: String,
    pub name: String,
    pub key: u32,
}

async fn req(
    base_url: String,
    endpoint: String,
    api_key: String,
) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/{}", base_url, endpoint);

    let response = client
        .get(url)
        .query(&[("X-Plex-Token", &api_key)])
        .header("X-Plex-Token", &api_key)
        .send()
        .await?;

    Ok(response)
}

fn get_attribute(e: &quick_xml::events::BytesStart, attr_name: &[u8]) -> Option<Vec<u8>> {
    e.attributes()
        .find(|a| a.as_ref().unwrap().key.as_ref() == attr_name)
        .map(|a| a.unwrap().value.into_owned())
}

async fn get_libraries_xml(
    base_url: String,
    api_key: String,
) -> Result<Vec<PlexLibrary>, Box<dyn Error>> {
    let response = req(base_url, String::from("library/sections"), api_key).await?;
    let xml = response.text().await?;

    let mut reader = Reader::from_str(&xml);
    let mut buf = Vec::new();
    let mut library: Vec<PlexLibrary> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(e)) | Ok(Event::Start(e)) => match e.name().as_ref() {
                b"Directory" => {
                    let section_type = get_attribute(&e, b"type");
                    let section_key = get_attribute(&e, b"key");
                    let section_title = get_attribute(&e, b"title");

                    if let (Some(library_type), Some(name), Some(key)) =
                        (section_type, section_title, section_key)
                    {
                        library.push(PlexLibrary {
                            library_type: String::from_utf8(library_type).unwrap(),
                            name: String::from_utf8(name).unwrap(),
                            key: String::from_utf8(key).unwrap().parse::<u32>()?,
                        });
                    }
                }
                _ => (),
            },
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => (),
        }
    }

    Ok(library)
}

async fn get_library_items(
    base_url: String,
    api_key: String,
    section_key: u32,
) -> Result<u32, Box<dyn Error>> {
    let response = req(
        base_url,
        format!("library/sections/{}/all", section_key),
        api_key,
    )
    .await?;
    let xml = response.text().await?;

    let mut reader = Reader::from_str(&xml);
    let mut buf = Vec::new();
    let mut counts: u32 = 0;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(e)) | Ok(Event::Start(e)) => match e.name().as_ref() {
                b"Directory" => counts += 1,
                b"Video" => counts += 1,
                _ => (),
            },
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => (),
        }
    }

    Ok(counts)
}

pub async fn get_library_media_count(
    base_url: String,
    api_key: String,
) -> Result<PlexLibraryCount, Box<dyn Error>> {
    let mut data: PlexLibraryCount = PlexLibraryCount {
        movies: 0,
        shows: 0,
    };
    let libraries = get_libraries_xml(base_url.clone(), api_key.clone()).await?;
    for library in libraries {
        let count = get_library_items(base_url.clone(), api_key.clone(), library.key).await?;
        match library.library_type.as_str() {
            "movie" => data.movies += count,
            "show" => data.shows += count,
            _ => (),
        }
    }

    Ok(data)
}
