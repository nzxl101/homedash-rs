# <img style="height: 200px; border-radius: 36px;" src="./public/icon.png">

# homedash-rs

A fast, focused home dashboard.

## Supported apps

- [x] AdGuard Home
- [x] Dockwatch
- [x] Overseerr
- [x] Plex
- [x] Prowlarr
- [x] Proxmox
- [x] qBittorrent
- [x] Radarr
- [x] Sonarr
- [x] Tautulli
- [x] Gluetun
- [x] TMDB
- [x] TVDB
- [x] HTTP Status
- [x] Unduck

# Screenshots

<img style="height: 200px" src="./public/dashboard-1.png">
<img style="height: 200px" src="./public/dashboard-2.png">

# Installation

Simplest way to deploy homedash-rs is to use Docker.  
Example `docker-compose.yml`:

```yaml
services:
    homedash-rs:
        container_name: homedash-rs
        network_mode: host
        environment:
            - PUID=1000 # Replace with your user ID
            - PGID=1000 # Replace with your group ID
            - TZ=Europe/Berlin # Replace with your timezone
        volumes:
            - ./data:/app/data # Replace with your data directory
        restart: unless-stopped
        image: ghcr.io/nzxl101/homedash-rs:latest
```

It's recommended to run homedash-rs on the same machine as your apps and use `network_mode: host`.

# Contributing

Feel free to open an issue or a PR if you'd like to contribute.
I'm happy to integrate any new features or bug fixes.

# Development

## Prerequisites

- cargo 1.98.1 or newer
- nodejs v24.19.0 or newer

## Run development server

```bash
git clone https://github.com/nzxl101/homedash-rs.git
cargo install tuono@0.19.7
pnpm i --frozen-lockfile
pnpm build
pnpm dev
```

# License

MIT License

Copyright (c) 2026 nzxl101
