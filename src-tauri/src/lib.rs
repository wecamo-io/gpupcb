// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use encoding_rs::{UTF_16BE, UTF_16LE, WINDOWS_1252};
use hyperlynx_parser::model::Board;
use hyperlynx_parser::parser::parse_hyperlynx;
use std::fs;

fn decode_hyperlynx_file(bytes: &[u8]) -> String {
    match bytes {
        // UTF-16
        [0xFF, 0xFE, rest @ ..] => UTF_16LE.decode(rest).0.to_string(),
        [0xFE, 0xFF, rest @ ..] => UTF_16BE.decode(rest).0.to_string(),

        // UTF-8
        _ if std::str::from_utf8(bytes).is_ok() => String::from_utf8(bytes.to_vec()).unwrap(),

        // place WINDOWS_1252 decoding as fallback
        _ => WINDOWS_1252.decode(bytes).0.to_string(),
    }
}

#[tauri::command]
fn load_hyperlynx_file(path: String) -> Result<Board, String> {
    let content: String = match fs::read(path) {
        Ok(bytes) => decode_hyperlynx_file(&bytes),
        Err(error) => panic!(
            "Couldn't read the file.\nMake sure it has the right format (.hyp or .HYP) and that the path is correct\n{error:#?}"
        ),
    };

    parse_hyperlynx(&content).map_err(|err| err.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_hyperlynx_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
