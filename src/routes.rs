use axum::{http::StatusCode, Json};
use printers;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::process::Command;

pub async fn get_printer_names() -> (StatusCode, Json<Vec<String>>) {
    println!("Getting printer names");
    let printers = printers::get_printers();
    if printers.is_empty() {
        (StatusCode::NOT_FOUND, Json(vec![]))
    } else {
        let printer_names = printers.iter().map(|p| p.name.clone()).collect();
        (StatusCode::OK, Json(printer_names))
    }
}

#[derive(Deserialize)]
pub struct PrintRequestBody {
    printer_name: String,
    name: String,
    content: String,
}

pub async fn print(
    Json(payload): Json<PrintRequestBody>,
) -> StatusCode {
    println!("Attempting to print with {} printer", payload.printer_name.as_str());
    let filename = payload.name.clone();
    
    // put the content into the html body
    let mut content_str = "<!DOCTYPE html><html><head><title>Page Title</title></head><body>".to_owned();
    content_str.push_str(&payload.content);
    content_str.push_str("</body></html>");

    // write the html string to a temp html file
    let temp_html_path = format!("{}.html", filename);
    let temp_pdf_path = format!("{}.pdf", filename);
    let mut file = File::create(&temp_html_path).await.expect("Failed to create file");
    file.write_all(content_str.as_bytes()).await.expect("Failed to write to file");
    drop(file);

    // convert the html file to a pdf file
    let output = Command::new("html2pdf")
        .arg(&temp_html_path)
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        println!("Successfully converted to pdf file")
    } else {
        println!("Failed to convert to pdf file");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    // print the pdf file to the printer
    if printers::print_file(&payload.printer_name, &temp_pdf_path, Some("Printing from the server")).is_ok() {
        println!("Printing info: \nPrinter: {}\nFile: {}\n", &payload.printer_name, &temp_pdf_path);
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
