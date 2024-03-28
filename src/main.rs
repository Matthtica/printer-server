use axum::{
    http::header::CONTENT_TYPE,
    http::Method,
    routing::{get, post},
    Router,
};
use local_ip_address::local_ip;
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
        .route("/testprint", post(routes::test_print))
        .layer(cors);

    let ipaddress = local_ip().unwrap();
    let port = "4590";

    let access_point = format!("{:?}:{}", ipaddress, port);

    let listener = tokio::net::TcpListener::bind(&access_point).await.unwrap();
    println!("Listening on:  {}", &access_point);
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Printer Server running"
}
