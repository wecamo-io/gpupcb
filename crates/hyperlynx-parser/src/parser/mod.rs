// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use version::parse_version_block;

mod utils;
mod version;

pub fn parse_version(input: &mut &str) -> String {
    parse_version_block(input).unwrap_or_else(|err| err.to_string())
}
