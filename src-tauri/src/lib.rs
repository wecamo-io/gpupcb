// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

// BROWSER

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::{load_hyperlynx_with_wasm, set_frequency_with_wasm, set_timestep_with_wasm};

// DESKTOP

#[cfg(not(target_arch = "wasm32"))]
mod desktop;

#[cfg(not(target_arch = "wasm32"))]
use desktop::commands::{load_hyperlynx_file, set_frequency, set_time_step};

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            load_hyperlynx_file,
            set_time_step,
            set_frequency
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
