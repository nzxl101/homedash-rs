use config::{Config, ConfigError, File};
use serde::Serialize;
use serde_derive::Deserialize;
use std::io::Write;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused)]
pub struct APIOauthConfig {
    pub enabled: bool,
    pub api_key: String,
    pub token: String,
    pub expires_in: u64,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused)]
pub struct APIKeyConfig {
    pub enabled: bool,
    pub url: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused)]
pub struct APICredsConfig {
    pub enabled: bool,
    pub url: String,
    pub username: String,
    pub password: String,
    pub cookie: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused)]
pub struct HttpUrl {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused)]
pub struct HttpConfig {
    pub enabled: bool,
    pub urls: Vec<HttpUrl>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused)]
pub struct ConfigFields {
    pub version: u8,
    pub base_url: String,
    pub username: String,
    pub weather_location: Vec<f64>,
    pub background_url: Option<String>,
    pub tvdb: APIOauthConfig,
    pub tmdb: APIOauthConfig,
    pub sonarr: APIKeyConfig,
    pub radarr: APIKeyConfig,
    pub prowlarr: APIKeyConfig,
    pub overseerr: APIKeyConfig,
    pub qbittorrent: APICredsConfig,
    pub plex: APIKeyConfig,
    pub tautulli: APIKeyConfig,
    pub proxmox: APICredsConfig,
    pub adguard: APICredsConfig,
    pub dockwatch: APIKeyConfig,
    pub http: HttpConfig,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused)]
pub struct OauthValues {
    pub service: String,
    pub token: String,
    pub expires_in: u64,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[allow(unused)]
pub struct CookieValues {
    pub service: String,
    pub cookie: String,
}

const LATEST_CONFIG_VERSION: u8 = 2; // Update on config structure changes

fn get_config_path() -> String {
    let path = if std::path::Path::new("data").exists() {
        "data/config.toml"
    } else {
        "config.toml"
    };

    return String::from(path);
}

fn create_default_config() -> ConfigFields {
    ConfigFields {
        version: LATEST_CONFIG_VERSION,
        base_url: String::from("http://localhost:3000"),
        username: String::from("user"),
        weather_location: vec![0.0, 0.0],
        background_url: Some(String::from("")),
        tvdb: APIOauthConfig {
            enabled: false,
            api_key: String::new(),
            token: String::new(),
            expires_in: 0,
        },
        tmdb: APIOauthConfig {
            enabled: false,
            api_key: String::new(),
            token: String::new(),
            expires_in: 0,
        },
        sonarr: APIKeyConfig {
            enabled: false,
            url: String::new(),
            api_key: String::new(),
        },
        radarr: APIKeyConfig {
            enabled: false,
            url: String::new(),
            api_key: String::new(),
        },
        prowlarr: APIKeyConfig {
            enabled: false,
            url: String::new(),
            api_key: String::new(),
        },
        overseerr: APIKeyConfig {
            enabled: false,
            url: String::new(),
            api_key: String::new(),
        },
        qbittorrent: APICredsConfig {
            enabled: false,
            url: String::new(),
            username: String::new(),
            password: String::new(),
            cookie: None,
        },
        plex: APIKeyConfig {
            enabled: false,
            url: String::new(),
            api_key: String::new(),
        },
        tautulli: APIKeyConfig {
            enabled: false,
            url: String::new(),
            api_key: String::new(),
        },
        proxmox: APICredsConfig {
            enabled: false,
            url: String::new(),
            username: String::new(),
            password: String::new(),
            cookie: None,
        },
        adguard: APICredsConfig {
            enabled: false,
            url: String::new(),
            username: String::new(),
            password: String::new(),
            cookie: None,
        },
        dockwatch: APIKeyConfig {
            enabled: false,
            url: String::new(),
            api_key: String::new(),
        },
        http: HttpConfig {
            enabled: false,
            urls: Vec::new(),
        },
    }
}

pub fn migrate_config(mut config: ConfigFields) -> ConfigFields {
    let mut migrated = false;

    while config.version < LATEST_CONFIG_VERSION {
        match config.version {
            1 => {
                if config.background_url.is_none() {
                    config.background_url = Some(String::from(""));
                }
                config.version = 2;
                migrated = true;
            }
            _ => {
                config.version += 1;
                migrated = true;
            }
        }
    }

    if migrated {
        if let Ok(toml_string) = toml::to_string(&config) {
            if let Ok(mut file) = std::fs::File::create(get_config_path()) {
                let _ = file.write_all(toml_string.as_bytes());
            }
        }
    }

    config
}

pub fn get_config() -> Result<ConfigFields, ConfigError> {
    let config_path = get_config_path();

    if !std::path::Path::new(&config_path).exists() {
        let default_config = create_default_config();
        let toml_string = toml::to_string(&default_config).unwrap();
        let mut file = std::fs::File::create(&config_path).unwrap();
        file.write_all(toml_string.as_bytes()).unwrap();
    }

    let config = Config::builder()
        .add_source(File::with_name(&config_path.trim_end_matches(".toml")).required(true))
        .build()?
        .try_deserialize()?;
    Ok(migrate_config(config))
}

pub fn write_oauth_config(fields: &OauthValues) -> Result<ConfigFields, ConfigError> {
    let mut config = get_config()?;

    if fields.service == "tvdb" {
        config.tvdb.token = String::from(&fields.token);
        config.tvdb.expires_in = fields.expires_in;
    }
    // Can add more oauth service logins here in the future

    let toml_string = toml::to_string(&config).unwrap();
    let mut file = std::fs::File::create(get_config_path()).unwrap();
    file.write_all(toml_string.as_bytes()).unwrap();

    Ok(config)
}

pub fn write_cookie_config(fields: &CookieValues) -> Result<ConfigFields, ConfigError> {
    let mut config = get_config()?;

    if fields.service == "qbittorrent" {
        config.qbittorrent.cookie = Some(String::from(&fields.cookie));
    }
    // Can add more cookie service logins here in the future

    let toml_string = toml::to_string(&config).unwrap();
    let mut file = std::fs::File::create(get_config_path()).unwrap();
    file.write_all(toml_string.as_bytes()).unwrap();

    Ok(config)
}
