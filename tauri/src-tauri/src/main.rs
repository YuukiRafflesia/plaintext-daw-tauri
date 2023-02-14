#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::render::render_project;

mod render;
mod project;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn syscall_test() {
    println!("hello tauri");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, syscall_test, render_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}