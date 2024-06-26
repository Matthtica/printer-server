use axum::{http::StatusCode, Json};
use printers;
use serde::Deserialize;
use std::process::Command;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::path::PathBuf;
use std::env;

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
pub struct TestPrintReqBody {
    printer_name: String,
}

pub async fn test_print(Json(payload): Json<TestPrintReqBody>) -> StatusCode {
    println!("Attempting to test print");
    let status = printers::print(&payload.printer_name, "Hello World".as_bytes(), Some("Test Print"));
    if status.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Deserialize)]
pub struct PrintRequestBody {
    printer_name: String,
    name: String,
    content: String,
}

pub async fn print(Json(payload): Json<PrintRequestBody>) -> StatusCode {
    println!(
        "Attempting to print with {} printer",
        payload.printer_name.as_str()
    );
    let filename = payload.name.clone();

    // put the content into the html body
    let mut content_str =
        "<!DOCTYPE html><html><head><title>Page Title</title></head><body>".to_owned();
    content_str.push_str(&payload.content);
    content_str.push_str("</body></html>");

    // TODO: os independent path
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let mut html_path = PathBuf::from(&current_dir);
    html_path.push(format!("{}.html", filename));
    let mut pdf_path = PathBuf::from(current_dir);
    pdf_path.push(format!("{}.pdf", filename));

    // write the html string to a temp html file
    let mut file = File::create(&html_path)
        .await
        .expect("Failed to create file");
    file.write_all(content_str.as_bytes())
        .await
        .expect("Failed to write to file");
    drop(file);

    // convert the html file to a pdf file
    let output = Command::new("html2pdf")
        .arg(&html_path)
        .arg("-o")
        .arg(&pdf_path)
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        println!("Successfully converted to pdf file")
    } else {
        println!("Failed to convert to pdf file");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    // print the pdf file to the printer
    let status = printers::print_file(
        &payload.printer_name,
        pdf_path.to_str().expect("Failed to convert path to string: pdf_path"),
        Some("Printing from the server"),
    );
    if status.is_ok() {
        println!(
            "Printing info... \nPrinter: {}\nFile: {}\n",
            &payload.printer_name, pdf_path.to_str().expect("Failed to convert path to string: pdf_path")
        );
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
