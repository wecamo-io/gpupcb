// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use crate::hyperlynx::parse_hyperlynx;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn load_hyperlynx_with_wasm(content: &str) -> Result<String, JsValue> {
    let board = parse_hyperlynx(content).map_err(|err| JsValue::from_str(&err))?;

    serde_json::to_string(&board).map_err(|err| JsValue::from_str(&err.to_string()))
}
