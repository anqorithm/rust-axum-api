<p align="center">
  <img src="docs/banner.svg" alt="rust-axum-api: a REST API in Rust with Axum, SQLx and SQLite" width="100%">
</p>

<p align="center">
  <a href="https://github.com/anqorithm/rust-axum-api/actions/workflows/ci.yml"><img src="https://github.com/anqorithm/rust-axum-api/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <img src="https://img.shields.io/badge/rust-2024_edition-f74c00?logo=rust" alt="Rust 2024">
  <img src="https://img.shields.io/badge/axum-0.8-blue" alt="Axum 0.8">
  <img src="https://img.shields.io/badge/sqlx-0.9-informational" alt="SQLx 0.9">
  <img src="https://img.shields.io/badge/database-SQLite-003b57?logo=sqlite" alt="SQLite">
</p>

A REST API for todos, built with Rust, [Axum](https://github.com/tokio-rs/axum), SQLx and SQLite.

## Features

- Full CRUD for `/todos` plus a `/health` check
- SQLite storage with migrations that run on startup
- Input validation and consistent JSON errors
- Request tracing with `tower-http`
- Integration tests and CI (fmt, clippy, tests)

## Architecture

```mermaid
flowchart LR
    C[Client<br/>curl / app] -->|HTTP JSON| R[Axum Router]
    R --> T[TraceLayer<br/>tower-http]
    T --> H[Handlers<br/>list · fetch · create · update · remove]
    H -->|validate| V{Valid?}
    V -->|no| E[400 JSON error]
    V -->|yes| S[(SQLite<br/>via SQLx pool)]
    S --> H
    H -->|JSON| C
```

## Request flow

```mermaid
sequenceDiagram
    participant C as Client
    participant A as Axum
    participant D as SQLite
    C->>A: POST /todos {"title":"ship it"}
    A->>A: trim + validate title
    A->>D: INSERT ... RETURNING *
    D-->>A: new row
    A-->>C: 201 Created {"id":1,...}
    C->>A: PATCH /todos/1 {"completed":true}
    A->>D: UPDATE ... RETURNING *
    D-->>A: updated row (or none)
    A-->>C: 200 OK / 404 Not Found
```

## Run

```sh
cargo run   # listens on 0.0.0.0:3000, creates todos.db and runs migrations
```

| Env var        | Default              |
|----------------|----------------------|
| `DATABASE_URL` | `sqlite://todos.db`  |
| `ADDR`         | `0.0.0.0:3000`       |
| `RUST_LOG`     | unset                |

## Endpoints

| Method | Path          | Body                                    | Response      |
|--------|---------------|-----------------------------------------|---------------|
| GET    | `/health`     |                                         | `200 ok`      |
| GET    | `/todos`      |                                         | `200` list    |
| POST   | `/todos`      | `{"title": "..."}`                      | `201` todo    |
| GET    | `/todos/{id}` |                                         | `200` / `404` |
| PATCH  | `/todos/{id}` | `{"title"?: "...", "completed"?: bool}` | `200` / `404` |
| DELETE | `/todos/{id}` |                                         | `204` / `404` |

Titles are trimmed and must be 1-200 characters.

## Examples

```console
$ curl -X POST localhost:3000/todos -H 'content-type: application/json' -d '{"title":"ship it"}'
{"id":1,"title":"ship it","completed":false,"created_at":"2026-11-21T15:26:31Z"}

$ curl -X PATCH localhost:3000/todos/1 -H 'content-type: application/json' -d '{"completed":true}'
{"id":1,"title":"ship it","completed":true,"created_at":"2026-11-21T15:26:31Z"}

$ curl localhost:3000/todos/42
{"error":"todo not found"}
```

## Project layout

```text
.
├── migrations/0001_create_todos.sql   # schema
├── src/lib.rs                         # router, handlers, errors
├── src/main.rs                        # config, DB pool, server
└── tests/api.rs                       # integration tests
```

## Test

```sh
cargo test
```
