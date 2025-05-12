use reqwest::{Client, Response}; // Remove blocking
use serde::{Deserialize, Serialize};
use std::error::Error;
use tuono_lib::Type;

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct TautulliV2Session {
    pub grandparent_title: Option<String>,
    pub title: Option<String>,
    pub media_index: Option<String>,
    pub parent_media_index: Option<String>,
    pub progress_percent: Option<String>,
    pub transcode_decision: Option<String>,
    pub user: Option<String>,
    pub video_full_resolution: Option<String>,
    pub stream_video_full_resolution: Option<String>,
    pub stream_video_codec: Option<String>,
    pub stream_audio_codec: Option<String>,
    pub stream_video_bitrate: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct TautulliV2Data {
    pub stream_count: Option<String>,
    pub sessions: Option<Vec<TautulliV2Session>>,
    pub stream_count_direct_play: Option<u32>,
    pub stream_count_direct_stream: Option<u32>,
    pub stream_count_transcode: Option<u32>,
    pub total_bandwidth: Option<u32>,
    pub lan_bandwidth: Option<u32>,
    pub wan_bandwidth: Option<u32>,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct TautulliV2Response {
    pub result: Option<String>,
    pub message: Option<String>,
    pub data: Option<TautulliV2Data>,
}

#[derive(Debug, Deserialize, Clone, Serialize, Type)]
#[allow(unused, non_snake_case)]
pub struct TautulliV2Sessions {
    pub response: TautulliV2Response,
}

async fn req(
    base_url: String,
    endpoint: String,
    api_key: String,
) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/api/v2", base_url);
    let response = client
        .get(url)
        .query(&[("apikey", api_key), ("cmd", endpoint)])
        .send()
        .await?;

    Ok(response)
}

pub async fn get_stream_sessions(
    base_url: String,
    api_key: String,
) -> Result<TautulliV2Sessions, Box<dyn Error>> {
    let response = req(base_url, String::from("get_activity"), api_key).await?;
    let data = response.json::<TautulliV2Sessions>().await?;

    Ok(data)
}
