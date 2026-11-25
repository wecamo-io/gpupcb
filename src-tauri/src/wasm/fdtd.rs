// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn set_timestep_with_wasm(value: f64) -> f64 {
    value
}
