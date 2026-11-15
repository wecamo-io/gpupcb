// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

#[cfg(not(target_arch = "wasm32"))]
mod desktop {

    use hyperlynx_parser::model::Board;
    use hyperlynx_parser::parser::{ReturnParseError, parse_hyperlynx};
    use std::fs;

    #[tauri::command]
    pub fn load_hyperlynx_file(path: String) -> Result<Board, ReturnParseError> {
        if !path.to_lowercase().ends_with(".hyp") {
            return Err(ReturnParseError {
                line: 0,
                placement: String::from("Invalid file extension. Expected .hyp"),
                inner: String::from("FileExtensionError"),
                offset: 0,
            });
        }

        let bytes = fs::read(path).map_err(|e| ReturnParseError {
            line: 0,
            placement: format!("File read error: {e}"),
            inner: String::from("FileSystemError"),
            offset: 0,
        })?;

        parse_hyperlynx(&bytes)
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use hyperlynx_parser::parser::{ReturnParseError, parse_hyperlynx};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn load_hyperlynx_with_wasm(filename: &str, content: &str) -> Result<String, JsValue> {
        // validate extension
        if !filename.to_lowercase().ends_with(".hyp") {
            let error = ReturnParseError {
                line: 0,
                placement: String::from("Invalid file extension. Expected .hyp"),
                inner: String::from("FileExtensionError"),
                offset: 0,
            };
        }

        let board = parse_hyperlynx(content).map_err(|e| {
            let error = ReturnParseError::from(e);

            serde_wasm_bindgen::to_value(&error).unwrap_or_else(|_| JsValue::NULL)
        })?;

        serde_json::to_string(&board).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use desktop::load_hyperlynx_file;

#[cfg(target_arch = "wasm32")]
pub use wasm::load_hyperlynx_with_wasm;

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![load_hyperlynx_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
