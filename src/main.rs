use axum::response::Html;
use axum::routing::get;
use axum::Router;



#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/flaschards", get(send_flaschard));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}




//serves index.html
async fn root () -> Html<&'static str> {
    Html(std::include_str!("index.html"))
}

async fn send_flaschard() {

}