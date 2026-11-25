// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

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
