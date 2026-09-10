# AGENTS.md

Rust + React homelab dashboard built with [Tuono](https://github.com/tuono-labs/tuono) v0.19.7. Tuono is the framework: file-based routing in `src/routes/`, Rust SSR handlers + an Axum server, React frontend.

## Commands

```bash
pnpm i --frozen-lockfile        # install (already done; use frozen, matches CI)
cargo install tuono@0.19.7      # tuono CLI (pin to the same version as package.json/Cargo.toml)
pnpm dev                        # dev server on http://localhost:3005 (regenerates .tuono/)
pnpm build                      # production build -> out/ + target/release/tuono binary
pnpm release                    # alias: cargo run --release
```

- No lint, typecheck, or test scripts exist. Verification = `pnpm build` (compiles TS + Rust).
- Tuono version must match across `package.json` (`"tuono"`), `Cargo.toml` (`tuono_lib`), and the installed CLI. Bump all three together.

## Generated/gitignored files — never edit or commit

- `.tuono/` — `main.rs` and `routeTree.gen.ts` auto-generated from `src/routes/` on every `tuono dev`/`tuono build`. `Cargo.toml` `[[bin]]` points at `.tuono/main.rs`, so a fresh clone cannot `cargo build` until a tuono command has generated it.
- `out/`, `target/`, `node_modules/`.
- `config.toml`, `database.db`, `data/` — gitignored local state. Live copies in this checkout hold real API keys/tokens; never commit them, don't echo them into output. Config is auto-created at `config.toml`, or `data/config.toml` when a `data/` dir exists (Docker mounts `/app/data`).

## Adding/routing

- Route files are `src/routes/**/<name>.rs` (server-side). Server-rendered props use `#[tuono_lib::handler]` returning `Response::Props`; API endpoints use `#[tuono_lib::api(GET|POST)]` returning axum responses. Files named `[id].rs` become dynamic routes (`req.params["id"]`, axum path `{id}`); each needs a matching `src/routes/**/<name>.tsx` React page.
- After adding/changing any route, run `pnpm dev` (or `build`) so Tuono rewrites `.tuono/` before checking the Rust code.
- Structs serialized into `tuono/types` (the TS route-props types) need `#[derive(Type)]` from `tuono_lib`.
- `Cargo.toml` relies on a Git fork via `[patch.crates-io] ssr_rs` — network access is required to build.

## Config schema

- Service config lives in `src/config.rs`. On any struct change: bump `LATEST_CONFIG_VERSION` and add a migration arm in `migrate_config` (versions are persisted in `config.toml` and auto-migrated).
- Adding support for a new app: config section + service module under `src/services/` + `populate_tables()` entry in `src/database.rs` + ping entry in `src/ping.rs` + route files under `src/routes/api/<app>/`.
- The DB caches API responses (`services`, `posters`) in SQLite (bundled rusqlite). OAuth tokens (TVDB) auto-refresh via `src/oauth.rs`; `src/cookie.rs` refreshes session cookies (qBittorrent).

## Frontend

- React + TanStack Query + Tailwind v4 (`src/styles/globals.css`); shadcn/ui components in `@/components/ui`, `@` aliases to `src/`. Drag/drop reorder persists via `POST /api/save`. Add shadcn bits with `npx shadcn@latest add`.
- tsconfig is strict with `noUnusedLocals`/`noUnusedParameters` — keep TS clean.

## CI / Docker

- `.github/workflows/ci.yml` only builds on `main`/PRs. The Docker build uncomments `openssl` in `Cargo.toml` via `sed -i 's/# openssl/openssl/'` — keep that commented line intact.
- Commit messages follow Conventional Commits (`feat:`, `fix:`, `chore:`); PRs/branches target `main`; releases are auto-tagged from the CI build on push to `main`.
