use axum::body::Body;
use axum::response::Html;
use axum::response::Response;
use axum::routing::get;
use axum::Router;



#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/index.js", get(js));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}




//serves index.html
async fn handler() -> Html<&'static str> {
    Html(std::include_str!("index.html"))
}