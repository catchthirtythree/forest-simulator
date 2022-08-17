#![allow(dead_code)]

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use forest_lib::forest::Forest;

struct AppState {
    forest: Mutex<Forest>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            forest: Mutex::new(Forest::new(123456, 120, 80)),
        }
    }
}

#[tauri::command]
fn get_map(state: tauri::State<AppState>) -> Vec<u32> {
    state.forest.lock().unwrap().map.clone()
}

#[tauri::command]
fn update(state: tauri::State<AppState>) {
    state.forest.lock().unwrap().update();
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_map,
            update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
