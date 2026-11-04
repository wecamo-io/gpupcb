// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

fn main() {
    let target = std::env::var("TARGET").unwrap();

    if target.starts_with("wasm32") {
        return;
    }

    tauri_build::build();
}
