#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod alkan_parser;
mod writer;

use alkan_parser::*;
use anyhow::Result;
use std::fs::{self, File};
use std::io::prelude::*;
use tectonic;
use writer::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![name_to_chemfig])
        .invoke_handler(tauri::generate_handler![name_to_pdf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn name_to_chemfig(name: String) -> String {
    match name_to_chemfig_internal(name) {
        Ok(v) => v,
        Err(err) => format!("Err: {}", err),
    }
}

fn name_to_chemfig_internal(alkan_name: String) -> Result<String> {
    let mut alkan_builder = AlkanBuilder::new(alkan_name)?;
    alkan_builder.trim_base();
    alkan_builder.sort_sides()?;
    let mut alkan_writer = AlkanWriter::new(&alkan_builder);
    alkan_writer.add_sides(&alkan_builder);
    Ok(alkan_writer.render())
}

#[tauri::command]
fn name_to_pdf(name: String) -> String {
    match name_to_pdf_internal(&name) {
        Ok(v) => v,
        Err(err) => format!("Err: {}", err),
    }
}

fn name_to_pdf_internal(alkan_name: &String) -> Result<String> {
    let file_path = format!("../public/{}.pdf", alkan_name);
    match fs::metadata(file_path) {
        Ok(_) => return Ok(format!("{alkan_name}.pdf")),
        Err(_) => (),
    }
    let mut alkan_builder = AlkanBuilder::new(alkan_name.clone())?;
    alkan_builder.trim_base();
    alkan_builder.sort_sides()?;
    let mut alkan_writer = AlkanWriter::new(&alkan_builder);
    alkan_writer.add_sides(&alkan_builder);
    let mut latex = r#"
        \documentclass{article}
        \usepackage{chemfig}
        \title{$1}
        \author{Денис}
        \begin{document}
        \chemfig{$2}
        \end{document}
        "#
    .to_string();
    latex = latex.replace("$1", &alkan_name);
    latex = latex.replace("$2", &alkan_writer.render()[..]);

    let pdf_data: Vec<u8> = tectonic::latex_to_pdf(latex).expect("processing failed");
    let mut file = File::create(format!("../public/{}.pdf", alkan_name))?;
    file.write_all(&pdf_data)?;
    Ok(format!("{alkan_name}.pdf"))
}
