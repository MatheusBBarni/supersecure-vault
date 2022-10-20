#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::api::Ctx;

mod api;

#[tokio::main]
async fn main() {
    let router = api::new().build().arced();

    tauri::Builder::default()
        .plugin(rspc::integrations::tauri::plugin(router, || Ctx {}))
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
