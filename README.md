# rust-axum-api

A REST API for todos, built with Rust, [Axum](https://github.com/tokio-rs/axum), SQLx and SQLite.

## Run

```sh
cargo run   # listens on 0.0.0.0:3000, creates todos.db and runs migrations
```

Env vars: `DATABASE_URL` (default `sqlite://todos.db`), `ADDR` (default `0.0.0.0:3000`), `RUST_LOG`.

## Endpoints

| Method | Path          | Body                                   | Response          |
|--------|---------------|----------------------------------------|-------------------|
| GET    | `/health`     |                                        | `200 ok`          |
| GET    | `/todos`      |                                        | `200` list        |
| POST   | `/todos`      | `{"title": "..."}`                     | `201` todo        |
| GET    | `/todos/{id}` |                                        | `200` / `404`     |
| PATCH  | `/todos/{id}` | `{"title"?: "...", "completed"?: bool}`| `200` / `404`     |
| DELETE | `/todos/{id}` |                                        | `204` / `404`     |

Errors return `{"error": "..."}`. Titles are trimmed and must be 1-200 characters.

```sh
curl -X POST localhost:3000/todos -H 'content-type: application/json' -d '{"title":"ship it"}'
```

## Test

```sh
cargo test
```
