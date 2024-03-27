use axum::{
    routing::get,
    Router,
};
use printer_server::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/printer-names", get(routes::get_printer_names))
        .route("/print/:printer_name", get(routes::print));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Printer Server running"
}
