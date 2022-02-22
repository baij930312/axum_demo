use axum::{
    body::Body,
    extract::{ContentLengthLimit, Extension, Form, Multipart, Path},
    http::{Request, StatusCode},
    response::IntoResponse,
    response::{Html, Response},
    routing::{get, get_service, post},
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
use tower::{layer::layer_fn, Service, ServiceBuilder};
use tower_http::services::ServeDir;

struct State {
    data: String,
}

fn main() {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    rt.block_on(async {
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", "example_multipart_form=debug,tower_http=debug")
        }
        tracing_subscriber::fmt::init();
        let state = AddExtensionLayer::new(Arc::new(State {
            data: "asdasdsa".into(),
        }));

        let middleware_stack = ServiceBuilder::new()
            // .layer(layer_fn(|inner| MyMiddleware { inner }))
            .layer(state)
            .layer(tower_http::trace::TraceLayer::new_for_http());

        let app = Router::new()
            .route("/", get(root))
            .route("/upload", get(upload_page).post(accept_form))
            .route("/json", post(create_user_json))
            .route("/url/:name/:age", get(create_user_get))
            .route("/from", post(create_user_from))
            .nest(
                "/static",
                get_service(ServeDir::new("./static")).handle_error(
                    |error: std::io::Error| async move {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        )
                    },
                ),
            )
            .layer(middleware_stack);

        let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    })
}

async fn upload_page() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/upload" method="post" enctype="multipart/form-data">
                    <label>
                        Upload file:
                        <input type="file" name="file" multiple>
                    </label>
                    <input type="submit" value="Upload files">
                </form>
            </body>
        </html>
        "#,
    )
}

async fn accept_form(
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            250 * 1024 * 1024 /* 250mb */
        },
    >,
) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }
}

async fn root(Extension(state): Extension<Arc<State>>) -> &'static str {
    println!("{}", state.data);
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
