#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::db::*;
use std::sync::Arc;

pub mod db;

mod api;

#[tokio::main]
async fn main() {
    let router = api::new().build().arced();
    let client = new_client().await.unwrap();

    #[cfg(debug_assertions)]
    client._db_push().await.unwrap();

    tauri::Builder::default()
        .plugin(rspc::integrations::tauri::plugin(router, || {
            Arc::new(client)
        }))
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
