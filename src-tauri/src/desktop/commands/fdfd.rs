// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

#[tauri::command]
pub fn set_frequency(value: f64) -> f64 {
    value
}
