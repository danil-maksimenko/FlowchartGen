#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod parser;
mod macros;
mod error;
mod ast;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parser::parse_cpp_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
