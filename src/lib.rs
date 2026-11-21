use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::SqlitePool;
use tower_http::trace::TraceLayer;

#[derive(Serialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

pub enum AppError {
    NotFound,
    BadRequest(&'static str),
    Db(sqlx::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Db(e)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "todo not found"),
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            AppError::Db(e) => {
                tracing::error!("database error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error")
            }
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}

type Result<T> = std::result::Result<T, AppError>;

fn valid_title(title: &str) -> Result<&str> {
    let t = title.trim();
    if t.is_empty() || t.len() > 200 {
        return Err(AppError::BadRequest("title must be 1-200 characters"));
    }
    Ok(t)
}

pub fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/todos", get(list).post(create))
        .route("/todos/{id}", get(fetch).patch(update).delete(remove))
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
}

async fn list(State(db): State<SqlitePool>) -> Result<Json<Vec<Todo>>> {
    let todos = sqlx::query_as("SELECT * FROM todos ORDER BY id")
        .fetch_all(&db)
        .await?;
    Ok(Json(todos))
}

async fn fetch(State(db): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Todo>> {
    sqlx::query_as("SELECT * FROM todos WHERE id = ?")
        .bind(id)
        .fetch_optional(&db)
        .await?
        .map(Json)
        .ok_or(AppError::NotFound)
}

async fn create(
    State(db): State<SqlitePool>,
    Json(body): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>)> {
    let todo = sqlx::query_as("INSERT INTO todos (title) VALUES (?) RETURNING *")
        .bind(valid_title(&body.title)?)
        .fetch_one(&db)
        .await?;
    Ok((StatusCode::CREATED, Json(todo)))
}

async fn update(
    State(db): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateTodo>,
) -> Result<Json<Todo>> {
    let title = body.title.as_deref().map(valid_title).transpose()?;
    sqlx::query_as(
        "UPDATE todos SET title = COALESCE(?, title), completed = COALESCE(?, completed)
         WHERE id = ? RETURNING *",
    )
    .bind(title)
    .bind(body.completed)
    .bind(id)
    .fetch_optional(&db)
    .await?
    .map(Json)
    .ok_or(AppError::NotFound)
}

async fn remove(State(db): State<SqlitePool>, Path(id): Path<i64>) -> Result<StatusCode> {
    let res = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&db)
        .await?;
    if res.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}
