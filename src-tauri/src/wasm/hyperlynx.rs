// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

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

        return Err(serde_wasm_bindgen::to_value(&error)?);
    }

    let board = parse_hyperlynx(content.as_bytes()).map_err(|e| {
        let error = ReturnParseError::from(e);

        serde_wasm_bindgen::to_value(&error).unwrap()
    })?;

    serde_json::to_string(&board).map_err(|e| JsValue::from_str(&e.to_string()))
}
