// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::async_runtime::Mutex;

mod config;
mod server;
use server::Extheaders;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
async fn get_headers(
    kw: String,
    state: tauri::State<'_, Mutex<server::Server>>,
) -> Result<Extheaders, String> {
    let binding = state.lock().await;
    let info = binding.get_headers(kw);
    Ok(info)
}
#[tauri::command]
async fn get_ext(
    id: &str,
    state: tauri::State<'_, Mutex<server::Server>>,
) -> Result<config::ExtInfo, String> {
    let binding = state.lock().await;
    binding
        .get_ext(id)
        .map(|e| e.clone())
        .ok_or("not found".to_string())
}

#[derive(serde::Serialize)]
pub struct ExtId {
    pub id: String,
}

#[tauri::command]
async fn set_ext(
    id: &str,
    ext: config::ExtInfo,
    state: tauri::State<'_, Mutex<server::Server>>,
) -> Result<ExtId, String> {
    let mut binding = state.lock().await;
    binding.set_ext(id, ext).map(|id| ExtId { id })
}

#[tauri::command]
async fn del_ext(id: String, state: tauri::State<'_, Mutex<server::Server>>) -> Result<(), String> {
    let mut binding = state.lock().await;
    binding.del_ext(&id)
}
fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(server::Server::new().unwrap()))
        .invoke_handler(tauri::generate_handler![
            get_headers,
            get_ext,
            set_ext,
            del_ext
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
