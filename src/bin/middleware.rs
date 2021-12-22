use axum::{
    body::Body,
    extract::{Extension, Form, Path},
    http::{Request, StatusCode},
    response::IntoResponse,
    response::Response,
    routing::{get, post},
    AddExtensionLayer, Json, Router,
};
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    str::FromStr,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::runtime::Builder;
use tower::{layer::layer_fn, Service};

struct State {
    data: String,
}

fn main() {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    rt.block_on(async {
        let state = AddExtensionLayer::new(Arc::new(State {
            data: "asdasdsa".into(),
        }));

        let app = Router::new()
            .route("/", get(root))
            .route("/json", post(create_user_json))
            .route("/url/:name/:age", get(create_user_get))
            .route("/from", post(create_user_from))
            .layer(state)
            .layer(layer_fn(|inner| MyMiddleware { inner }));

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

async fn create_user_json(
    Json(payload): Json<CreateUser>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    println!("{}", state.data);
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

#[derive(Clone)]
struct MyMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for MyMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        println!("`MyMiddleware` called!");

        // best practice is to clone the inner service like this
        // see https://github.com/tower-rs/tower/issues/547 for details
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            let res: Response = inner.call(req).await?;

            println!("`MyMiddleware` received the response");

            Ok(res)
        })
    }
}
