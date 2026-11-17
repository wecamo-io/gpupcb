// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

#![no_main]

use hyperlynx_parser::parser::decode_hyperlynx_file;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut utf16be = vec![0xFE, 0xFC];

    utf16be.extend_from_slice(data);
    let _ = decode_hyperlynx_file(&utf16be);
});
