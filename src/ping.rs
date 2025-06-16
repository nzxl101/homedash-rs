use std::error::Error;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time;
use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;

use rusqlite::Connection;

use crate::config::ConfigFields;

pub fn ping_all_urls(config: &ConfigFields, conn: &Connection) -> Result<(), Box<dyn Error>> {
    let current_time = SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Check if last ping was within 5 minutes
    let last_check: u64 = conn
        .query_row("SELECT MAX(last_check) FROM services", [], |row| row.get(0))
        .unwrap_or(0);

    if current_time - last_check < 300 {
        // 5 minutes = 300 seconds
        println!("Skipping ping check - last check was less than 5 minutes ago");
        return Ok(());
    }

    let last_check = current_time;
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut urls = vec![];

    if config.sonarr.enabled {
        urls.push(&config.sonarr.url);
    }
    if config.radarr.enabled {
        urls.push(&config.radarr.url);
    }
    if config.prowlarr.enabled {
        urls.push(&config.prowlarr.url);
    }
    if config.overseerr.enabled {
        urls.push(&config.overseerr.url);
    }
    if config.qbittorrent.enabled {
        urls.push(&config.qbittorrent.url);
    }
    if config.plex.enabled {
        urls.push(&config.plex.url);
    }
    if config.tautulli.enabled {
        urls.push(&config.tautulli.url);
    }
    if config.proxmox.enabled {
        urls.push(&config.proxmox.url);
    }
    if config.adguard.enabled {
        urls.push(&config.adguard.url);
    }
    if config.dockwatch.enabled {
        urls.push(&config.dockwatch.url);
    }
    if config.http.enabled {
        for url in &config.http.urls {
            urls.push(&url.url);
        }
    }

    let mut handles = vec![];

    let start_time = Instant::now();

    for url in urls {
        let results_clone = Arc::clone(&results);
        let url_clone = url.clone();
        let handle = thread::spawn(move || match ping_url(&url_clone) {
            Ok(status) => {
                results_clone
                    .lock()
                    .unwrap()
                    .push((url_clone.clone(), status));
                println!("Ping Success for {}: Status Code {}", url_clone, status)
            }
            Err(e) => {
                results_clone.lock().unwrap().push((url_clone.clone(), 503));
                println!("Ping Error for {}: {}", url_clone, e)
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for result in results.lock().unwrap().clone().into_iter() {
        let (url, status) = result;
        println!(
            "Updating database with ping results for {} ({})",
            url, status
        );
        conn.execute(
            "UPDATE services SET status = ?1, last_check = ?2 WHERE url = ?3",
            (status, last_check, url),
        )?;
    }

    let duration = start_time.elapsed();
    println!("Total time to ping all URLs: {:?}", duration);

    Ok(())
}

fn ping_url(url: &str) -> Result<u16, Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();
    let url = String::from(url); // Clone the URL into a String

    thread::spawn(move || {
        let client = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true) // Accept invalid certificates
            .timeout(Duration::from_secs(2)) // 2s timeout
            .build()
            .unwrap();

        match client.get(&url).send() {
            Ok(response) => tx.send(Ok(response)).unwrap(),
            Err(e) => tx.send(Err(Box::new(e))).unwrap(),
        }
    });

    let response = rx.recv_timeout(Duration::from_secs(5))??;
    Ok(response.status().as_u16())
}
