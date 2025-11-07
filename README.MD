# 🦀 Rust LLM Proxy

A lightweight, production-ready gateway for routing client requests to multiple large language model (LLM) providers. The service is written in Rust on top of Axum and Tokio, uses SeaORM for persistence, and embeds HTML templates with Askama for lightweight admin views.

## Key Capabilities
- **Unified AI access layer** – Proxy requests to commercial providers (OpenAI, Anthropic) or self-hosted inference services through a single REST interface.
- **Granular access control** – Authenticate users with signed tokens, manage API keys, and gate admin-only operations with middleware.
- **Job tracking** – Persist service jobs in SQLite (or any SeaORM-compatible database) to provide deterministic auditing of model usage.
- **Operational tooling** – Automatic database migrations and seeders, structured configuration, and reusable middleware for logging, authentication, and authorization.

## Roadmap
- ✅ Gemini (implemented)
- 🔜 OpenAI
- 🔜 Grok
- 🔜 Claude

## Project Layout
```
├── src
│   ├── app.rs             # Bootstraps configuration, database, routes, and HTTP server
│   ├── core               # Cross-cutting concerns: config, HTTP bootstrap, router helpers, global state
│   ├── features           # Bounded contexts (users, services, jobs, home)
│   ├── middleware         # Authentication, authorization, and request guards
│   ├── routes.rs          # Aggregates feature routers into the final Axum router
│   ├── seed               # Database seeders (e.g., default admin)
│   └── utility            # Shared helpers such as application state accessors
├── migration              # SeaORM migration crate
├── templates              # Askama templates rendered by feature controllers
└── .env.example           # Reference environment configuration
```

## Getting Started
### Prerequisites
- Rust toolchain (1.80+ recommended)
- SQLite 3 (default) or another database supported by SeaORM

### Installation
1. **Clone the repository**
   ```bash
   git clone https://github.com/habibi-dev/rust-llm-proxy.git
   cd rust-llm-proxy
   ```
2. **Create your environment file**
   ```bash
   cp .env.example .env
   ```
   Adjust host, port, database URL, and secrets as needed. The application automatically loads the file at startup.
3. **Run the service**
   ```bash
   cargo run
   ```
   The bootstrapper establishes the database connection, runs migrations, seeds an admin user, and exposes the HTTP API (default `http://127.0.0.1:8080`).

### Production Build
```bash
cargo build --release
```
The release profile enables link-time optimization (LTO) and binary stripping for minimal footprint deployments.

## Configuration
| Variable | Description | Default |
| --- | --- | --- |
| `APP_DOMAIN` | Internal domain used when generating URLs. | `localhost` |
| `APP_FINAL_DOMAIN` | Public-facing domain for canonical links. | `localhost` |
| `APP_HOST` | Interface bound by the HTTP listener. | `127.0.0.1` |
| `APP_PORT` | Port exposed by the listener. | `8080` |
| `APP_HTTPS` | Enables HTTPS-specific behavior (e.g., secure cookies). | `false` |
| `DATABASE_URL` | SeaORM connection string. | `sqlite://database.db?mode=rwc` |
| `HMAC_KEY` | Secret used for signing authentication tokens. | _(empty)_ |

## Database & Seeds
- **Migrations** are executed automatically during startup via `migration::Migrator`.
- **Seed data** (`src/seed`) inserts an initial administrator so that you can authenticate immediately after a fresh install.

For manual administration you can run the migration crate directly:
```bash
cargo run -p migration
```

## API Overview
The proxy exposes versioned routes under `/api/v1`.

| Domain | Base Path | Selected Endpoints |
| --- | --- | --- |
| Users | `/api/v1/users` | `GET /` list users (admin), `POST /` create user (admin), `GET /me` authenticated profile, `PUT /{user_id}` update (admin), `DELETE /{user_id}` remove (admin) |
| API Keys | `/api/v1/api-keys` | `GET /` list (admin), `POST /` issue new key (admin), `PUT /{api_key}` rotate (admin), `DELETE /{api_key}` revoke (admin) |
| Services | `/api/v1/services` | `GET /` list providers (admin), `POST /` register provider (admin), `PUT /{service_id}` update (admin), `DELETE /{service_id}` deactivate (admin), `POST /{service_id}/chat` send a chat request (admin) |
| Jobs | `/api/v1/jobs` | `GET /{job_id}` inspect a job created by proxy interactions |

All `/api/v1/**` routes require authentication middleware. Administrative actions are additionally protected by the `is_admin` guard.

## Development Tips
- **Routing composition** – Feature routers are assembled through `core::router::Router`, allowing each bounded context to remain focused on its responsibility.
- **State management** – `core::state` keeps application state behind a thread-safe `OnceCell`, while `utility::state::app_state()` exposes a typed accessor to avoid tight coupling.
- **Extensibility** – Add new providers by creating a feature module with dedicated models, repositories, services, and controllers. Register the router in `routes.rs` to expose the API.

## License
MIT © Habibi-Dev
