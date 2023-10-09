// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod game_data;
mod sudoku;

use game_data::DATA;
use sudoku::Sudoku;

use std::sync::Mutex;
use tauri::State;

struct GameState {
    sudoku: Mutex<Sudoku>,
}

#[tauri::command]
fn new_game(game_state: State<GameState>, index: usize) -> Sudoku {
    *game_state.sudoku.lock().unwrap() = DATA[index];
    *game_state.sudoku.lock().unwrap()
}

#[tauri::command]
fn evaluate(game_state: State<GameState>) -> bool {
    game_state.sudoku.lock().unwrap().is_completed()
}

#[tauri::command]
fn input_sync(game_state: State<GameState>, index: (usize, usize), value: u32) {
    game_state.sudoku.lock().unwrap().values[index.0][index.1] = value;
}

fn main() {
    tauri::Builder::default()
        .manage(GameState {
            sudoku: Mutex::new(Sudoku::new()),
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![new_game, evaluate, input_sync])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
