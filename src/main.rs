use axum::{routing::get, Router, Json};
use serde_json::{Value, json};

async fn home() -> Json<Value> {
    Json(json!({ "msg": "hello" }))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
