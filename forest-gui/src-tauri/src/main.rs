#![allow(dead_code)]

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use forest_lib::forest::Forest;
use serde::Serialize;

const DEFAULT_SEED: u64 = 123123;
const DEFAULT_WIDTH: usize = 120;
const DEFAULT_HEIGHT: usize = 80;

struct AppState {
    forest: Mutex<Forest>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            forest: Mutex::new(Forest::new(
                DEFAULT_SEED,
                DEFAULT_WIDTH,
                DEFAULT_HEIGHT,
            )),
        }
    }
}

#[derive(Serialize)]
struct ForestInfo {
    map: Vec<u16>,
    width: usize,
    height: usize,
    months_elapsed: u32,
    yearly_lumber: u32,
    yearly_mauls: u32,
}

impl ForestInfo {
    fn new(forest: &Forest) -> Self {
        Self {
            map: forest.map.clone(),
            width: forest.width,
            height: forest.height,
            months_elapsed: forest.months_elapsed,
            yearly_lumber: forest.yearly_lumber,
            yearly_mauls: forest.yearly_mauls,
        }
    }
}

#[tauri::command]
fn create_forest(
    seed: Option<u64>,
    width: Option<usize>,
    height: Option<usize>,
    state: tauri::State<AppState>
) -> ForestInfo {
    let forest = Forest::new(
        seed.unwrap_or(DEFAULT_SEED),
        width.unwrap_or(DEFAULT_WIDTH),
        height.unwrap_or(DEFAULT_HEIGHT),
    );
    *state.forest.lock().unwrap() = forest;
    ForestInfo::new(&state.forest.lock().unwrap())
}

#[tauri::command]
fn get_forest(state: tauri::State<AppState>) -> ForestInfo {
    ForestInfo::new(&state.forest.lock().unwrap())
}

#[tauri::command]
fn update_forest(state: tauri::State<AppState>) {
    let mut forest = state.forest.lock().unwrap();
    forest.update();
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            create_forest,
            get_forest,
            update_forest,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
