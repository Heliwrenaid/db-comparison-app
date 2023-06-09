// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menu;
mod commands;
mod database;
mod models;

fn main() {
    tauri::Builder::default()
        .menu(menu::build())
        .on_menu_event(|event| menu::handle_menu_event(event))
        .invoke_handler(tauri::generate_handler![
            commands::run_query,
            commands::sort_pkgs_by_field_with_limit,
            commands::get_query_time
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
