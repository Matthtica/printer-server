use axum::{
    http::header::CONTENT_TYPE,
    http::Method,
    routing::{get, post},
    Router,
};
use printer_server::*;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(root))
        .route("/printers", get(routes::get_printer_names))
        .route("/print", post(routes::print))
        .layer(cors);

    let port = "0.0.0.0:4590";
    let listener = tokio::net::TcpListener::bind(port).await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Printer Server running"
}
