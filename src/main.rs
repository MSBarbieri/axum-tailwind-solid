use axum::{
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

async fn index() -> impl IntoResponse {
    let html = tokio::fs::read_to_string("web/index.html").await.unwrap();
    Html(html)
}
async fn css() -> impl IntoResponse {
    let css = tokio::fs::read_to_string("web/main.css").await.unwrap();
    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(css)
        .unwrap()
}

async fn js() -> impl IntoResponse {
    let js = tokio::fs::read_to_string("web/index.js").await.unwrap();
    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(js)
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(index))
        .route("/main.css", get(css))
        .route("/index.js", get(js));
    let listener = "127.0.0.1:3000".parse().unwrap();
    axum::Server::bind(&listener)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
