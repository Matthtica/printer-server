use axum::{extract::Path, http::StatusCode, Json};
use printers;
use serde::{Deserialize, Serialize};

pub async fn get_printer_names() -> (StatusCode, Json<Vec<String>>) {
    let printers = printers::get_printers();
    if printers.is_empty() {
        (StatusCode::NOT_FOUND, Json(vec![]))
    } else {
        let printer_names = printers.iter().map(|p| p.name.clone()).collect();
        (StatusCode::OK, Json(printer_names))
    }
}

#[derive(Deserialize, Serialize)]
pub struct Params {
    printer_name: String,
}

pub async fn print(Path(Params { printer_name }): Path<Params>) -> StatusCode {
    println!("Printing to {}", printer_name);

    let status = printers::print(
        printer_name.as_ref(),
        "Hello world!".as_bytes(),
        Some("Test printer server"),
    );
    if status.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
