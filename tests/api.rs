use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use sqlx::SqlitePool;
use tower::ServiceExt;

async fn call(
    pool: &SqlitePool,
    method: &str,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let req = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(body.map_or(Body::empty(), |b| Body::from(b.to_string())))
        .unwrap();
    let res = rust_axum_api::app(pool.clone()).oneshot(req).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[sqlx::test]
async fn todo_crud(pool: SqlitePool) {
    let (s, todo) = call(
        &pool,
        "POST",
        "/todos",
        Some(json!({"title": "  write tests "})),
    )
    .await;
    assert_eq!(s, StatusCode::CREATED);
    assert_eq!(todo["title"], "write tests");
    assert_eq!(todo["completed"], false);
    let id = todo["id"].as_i64().unwrap();

    let (s, list) = call(&pool, "GET", "/todos", None).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);

    let (s, todo) = call(
        &pool,
        "PATCH",
        &format!("/todos/{id}"),
        Some(json!({"completed": true})),
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(todo["completed"], true);
    assert_eq!(todo["title"], "write tests");

    let (s, _) = call(&pool, "DELETE", &format!("/todos/{id}"), None).await;
    assert_eq!(s, StatusCode::NO_CONTENT);

    let (s, err) = call(&pool, "GET", &format!("/todos/{id}"), None).await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    assert_eq!(err["error"], "todo not found");
}

#[sqlx::test]
async fn rejects_bad_input(pool: SqlitePool) {
    let (s, _) = call(&pool, "POST", "/todos", Some(json!({"title": "   "}))).await;
    assert_eq!(s, StatusCode::BAD_REQUEST);

    let (s, _) = call(
        &pool,
        "PATCH",
        "/todos/999",
        Some(json!({"completed": true})),
    )
    .await;
    assert_eq!(s, StatusCode::NOT_FOUND);

    let (s, _) = call(&pool, "DELETE", "/todos/999", None).await;
    assert_eq!(s, StatusCode::NOT_FOUND);
}
