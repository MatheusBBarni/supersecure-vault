#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{borrow::Borrow, sync::Arc};

mod api;
pub mod prisma;

#[tokio::main]
async fn main() {
    let router = api::new().build().arced();
    let client = Arc::new(prisma::new_client().await.unwrap());

    tauri::Builder::default()
        .plugin(rspc::integrations::tauri::plugin(router, || client))
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
