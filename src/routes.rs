use axum::{extract::Path, http::StatusCode, Json};
use base64::prelude::*;
use printers;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

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

#[derive(Deserialize)]
pub struct PrintRequestBody {
    name: String,
    encoded_data: String,
}

pub async fn print(
    Path(Params { printer_name }): Path<Params>,
    Json(payload): Json<PrintRequestBody>,
) -> StatusCode {
    println!("Attempting to print with {} printer", printer_name);

    let decoded_data = match BASE64_STANDARD.decode(payload.encoded_data) {
        Ok(data) => data,
        Err(_) => {
            println!("Decoding error");
            return StatusCode::BAD_REQUEST;
        }
    };
    let mut filename = payload.name.clone();
    filename.push_str(".pdf");

    let mut file = match File::create(&filename).await {
        Ok(file) => file,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    if let Err(_) = file.write_all(&decoded_data).await {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    let status = printers::print_file(
        printer_name.as_ref(),
        &filename,
        Some("Test printer server"),
    );
    if status.is_ok() {
        println!("Currently printing...");
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
