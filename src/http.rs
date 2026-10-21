use reqwest::Client;
use std::sync::OnceLock;

/// User-Agent sent by every homedash HTTP request.
///
/// Reverse proxies in front of monitored services (e.g. Anubis on Pangolin)
/// deny or challenge headerless clients. Tagging all machine traffic here
/// lets those middlewares allow the dashboard explicitly.
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Shared HTTP client for all service modules (reuses connection pools).
static CLIENT: OnceLock<Client> = OnceLock::new();

pub fn client() -> Client {
    CLIENT
        .get_or_init(|| {
            Client::builder()
                .user_agent(USER_AGENT)
                .build()
                .expect("failed to build shared http client")
        })
        .clone()
}
