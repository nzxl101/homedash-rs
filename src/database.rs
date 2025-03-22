use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::config::ConfigFields;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused)]
pub struct Poster {
    pub id: Option<u32>,
    pub title: String,
    pub image: String,
    pub tvdb_id: Option<u32>,
    pub tmdb_id: Option<u32>,
    pub json_data: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused)]
pub struct Ping {
    pub id: Option<u32>,
    pub name: String,
    pub status: u32,
    pub last_check: u32,
    pub url: String,
    pub app_order: i32,
    pub is_favorite: bool,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused)]
pub struct Preference {
    pub id: u32,
    pub is_favorite: bool,
    pub app_order: i32,
}

pub fn connect_db() -> Result<Connection, Box<dyn Error>> {
    let db_path = if std::path::Path::new("data").exists() {
        "data/database.db"
    } else {
        "database.db"
    };

    let conn = match Connection::open(db_path) {
        Ok(conn) => conn,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(conn)
}

pub fn create_schemas(conn: &Connection) -> Result<()> {
    /*
         id = service id
         name = service name
         status = service status
         last_check = last time the service was checked
         expires = time in seconds before the json_data cache expires
         url = service url
         json_data = service data in json format + needs a timestamp appended for cache expiration
    */
    conn.execute(
        "CREATE TABLE IF NOT EXISTS services (
              id          INTEGER PRIMARY KEY,
              name        TEXT NOT NULL,
              status      INTEGER DEFAULT 0,
              last_check  INTEGER DEFAULT 0,
              expires     INTEGER DEFAULT 180,
              url         TEXT NOT NULL,
              json_data   TEXT,
              UNIQUE(name, url)
           )",
        (),
    )?;

    /*
         id = preference id
         service_id = service id
         is_favorite = is the service a favorite
         app_order = order of the service
    */
    conn.execute(
        "CREATE TABLE IF NOT EXISTS preferences (
              id            INTEGER PRIMARY KEY AUTOINCREMENT,
              service_id    INTEGER NOT NULL,
              is_favorite   BOOLEAN DEFAULT FALSE,
              app_order         INTEGER NOT NULL,
              FOREIGN KEY(service_id) REFERENCES services(id)
            )",
        (),
    )?;

    /*
        trigger to insert a new preference when a new service is inserted
    */
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS insert_preferences
              AFTER INSERT ON services
              BEGIN
               INSERT INTO preferences (service_id, app_order) VALUES (NEW.id, (SELECT COALESCE(MAX(app_order), 0) + 1 FROM preferences));
              END;",
        (),
    )?;

    /*
        trigger to delete a preference when a service is deleted
    */
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS delete_preferences
              AFTER DELETE ON services
              BEGIN
               DELETE FROM preferences WHERE service_id = OLD.id;
              END;",
        (),
    )?;

    /*
        trigger to update the order of the preferences when a service is deleted
    */
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS update_order
              AFTER DELETE ON preferences
              BEGIN
               UPDATE preferences SET app_order = app_order - 1 WHERE app_order > OLD.app_order;
              END;",
        (),
    )?;

    /*
        id = poster id
        title = poster title
        image = poster image
        tvdb_id = poster tvdb id
        tmdb_id = poster tmdb id
        json_data = poster api data in json
    */
    conn.execute(
        "CREATE TABLE IF NOT EXISTS posters (
              id          INTEGER PRIMARY KEY,
              title       TEXT NOT NULL,
              image       TEXT NOT NULL,
              tvdb_id     INTEGER DEFAULT 0,
              tmdb_id     INTEGER DEFAULT 0,
              json_data   TEXT NOT NULL
            )",
        (),
    )?;

    /*
       test data
    */
    // conn.execute(
    //     "INSERT INTO services (name, url) VALUES (?1, ?2)",
    //     (String::from("Sonarr"), String::from("URL_ADDRESS")),
    // )?;

    Ok(())
}

pub fn populate_tables(conn: &Connection, config: &ConfigFields) -> Result<()> {
    let mut insert = conn.prepare("INSERT OR IGNORE INTO services (name, url) VALUES (?1, ?2)")?;
    let mut delete = conn.prepare("DELETE FROM services WHERE name = ?1")?;

    // Handle built-in services
    if config.sonarr.enabled {
        insert.execute(params![String::from("Sonarr"), config.sonarr.url])?;
    } else {
        delete.execute(params![String::from("Sonarr")])?;
    }

    if config.radarr.enabled {
        insert.execute(params![String::from("Radarr"), config.radarr.url])?;
    } else {
        delete.execute(params![String::from("Radarr")])?;
    }

    if config.prowlarr.enabled {
        insert.execute(params![String::from("Prowlarr"), config.prowlarr.url])?;
    } else {
        delete.execute(params![String::from("Prowlarr")])?;
    }

    if config.overseerr.enabled {
        insert.execute(params![String::from("Overseerr"), config.overseerr.url])?;
    } else {
        delete.execute(params![String::from("Overseerr")])?;
    }

    if config.qbittorrent.enabled {
        insert.execute(params![String::from("qBittorrent"), config.qbittorrent.url])?;
    } else {
        delete.execute(params![String::from("qBittorrent")])?;
    }

    if config.plex.enabled {
        insert.execute(params![String::from("Plex"), config.plex.url])?;
    } else {
        delete.execute(params![String::from("Plex")])?;
    }

    if config.tautulli.enabled {
        insert.execute(params![String::from("Tautulli"), config.tautulli.url])?;
    } else {
        delete.execute(params![String::from("Tautulli")])?;
    }

    if config.proxmox.enabled {
        insert.execute(params![String::from("Proxmox"), config.proxmox.url])?;
    } else {
        delete.execute(params![String::from("Proxmox")])?;
    }

    if config.adguard.enabled {
        insert.execute(params![String::from("AdGuard"), config.adguard.url])?;
    } else {
        delete.execute(params![String::from("AdGuard")])?;
    }

    if config.dockwatch.enabled {
        insert.execute(params![String::from("Dockwatch"), config.dockwatch.url])?;
    } else {
        delete.execute(params![String::from("Dockwatch")])?;
    }

    // For HTTP URLs, first collect all configured URLs
    let mut config_urls = Vec::new();
    if config.http.enabled {
        for url in &config.http.urls {
            config_urls.push((String::from(&url.name), String::from(&url.url)));
            insert.execute(params![url.name, url.url])?;
        }
    }

    // Only delete HTTP URLs that are not in the current config
    let mut stmt = conn.prepare(
        "SELECT name, url FROM services
         WHERE name NOT IN ('Sonarr', 'Radarr', 'Prowlarr', 'Overseerr',
         'qBittorrent', 'Plex', 'Tautulli', 'Proxmox', 'AdGuard', 'Dockwatch')",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;

    for row in rows {
        let (name, url) = row?;
        if !config_urls.contains(&(name.clone(), url.clone())) {
            delete.execute(params![name])?;
        }
    }

    Ok(())
}

pub fn get_tvdb_id(conn: &Connection, tvdb_id: u32) -> Result<Poster, Box<dyn Error>> {
    Ok(conn.query_row(
        "SELECT * FROM posters WHERE tvdb_id = ?1",
        [tvdb_id],
        |row| {
            Ok(Poster {
                id: row.get(0)?,
                title: row.get(1)?,
                image: row.get(2)?,
                tvdb_id: row.get(3)?,
                tmdb_id: row.get(4)?,
                json_data: row.get(5)?,
            })
        },
    )?)
}

pub fn insert_tvdb_id(conn: &Connection, poster: &Poster) -> Result<usize, Box<dyn Error>> {
    Ok(conn.execute(
        "INSERT INTO posters (title, image, tvdb_id, json_data) VALUES (?1, ?2, ?3, ?4)",
        params![poster.title, poster.image, poster.tvdb_id, poster.json_data],
    )?)
}

pub fn get_tmdb_id(conn: &Connection, tmdb_id: u32) -> Result<Poster, Box<dyn Error>> {
    Ok(conn.query_row(
        "SELECT * FROM posters WHERE tmdb_id = ?1",
        [tmdb_id],
        |row| {
            Ok(Poster {
                id: row.get(0)?,
                title: row.get(1)?,
                image: row.get(2)?,
                tvdb_id: row.get(3)?,
                tmdb_id: row.get(4)?,
                json_data: row.get(5)?,
            })
        },
    )?)
}

pub fn insert_tmdb_id(conn: &Connection, poster: &Poster) -> Result<usize, Box<dyn Error>> {
    Ok(conn.execute(
        "INSERT INTO posters (title, image, tmdb_id, json_data) VALUES (?1, ?2, ?3, ?4)",
        params![poster.title, poster.image, poster.tmdb_id, poster.json_data],
    )?)
}

pub fn get_ping_data(conn: &Connection) -> Result<Vec<Ping>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT s.id, s.name, s.status, s.last_check, s.url, p.app_order, p.is_favorite
         FROM services s
         LEFT JOIN preferences p ON s.id = p.service_id
         ORDER BY p.app_order",
    )?;

    let ping_iter = stmt.query_map([], |row| {
        Ok(Ping {
            id: row.get(0)?,
            name: row.get(1)?,
            status: row.get(2)?,
            last_check: row.get(3)?,
            url: row.get(4)?,
            app_order: row.get(5)?,
            is_favorite: row.get(6)?,
        })
    })?;

    let mut pings = Vec::new();
    for ping in ping_iter {
        pings.push(ping?);
    }
    Ok(pings)
}

pub fn update_app_preferences(
    conn: &mut Connection,
    apps: Vec<Preference>,
) -> Result<(), Box<dyn Error>> {
    let tx = conn.transaction()?;

    for app in apps {
        tx.execute(
            "UPDATE preferences
             SET is_favorite = ?1, app_order = ?2
             WHERE service_id = ?3",
            params![app.is_favorite, app.app_order, app.id],
        )?;
    }

    tx.commit()?;
    Ok(())
}
