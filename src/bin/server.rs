use std::{collections::HashMap, net::SocketAddr, str::FromStr};

use axum::{
    extract::{Form, Path},
    handler::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::runtime::Builder;
fn main() {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    rt.block_on(async {
        let app = Router::new()
            .route("/", get(root))
            .route("/json", post(create_user_json))
            .route("/url/:name/:age", get(create_user_get))
            .route("/from", post(create_user_from));
        let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    })
}

async fn root() -> &'static str {
    "hello"
}

async fn create_user_json(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1000,
        name: payload.name,
        age: payload.age,
    };
    (StatusCode::CREATED, Json(user))
}

async fn create_user_get(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    let name = params.get("name").unwrap().clone();
    let age = params.get("age").unwrap().parse().unwrap();
    let user = User {
        id: 1000,
        name,
        age,
    };
    (StatusCode::CREATED, Json(user))
}

async fn create_user_from(Form(from): Form<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1000,
        name: from.name,
        age: from.age,
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    age: i32,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    age: i32,
}
