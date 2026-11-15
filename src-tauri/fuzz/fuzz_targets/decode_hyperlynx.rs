// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

#![no_main]

use gpupcb_app_lib::decode_hyperlynx_file;
use hyperlynx_parser::parser::parse_hyperlynx;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let decoded = decode_hyperlynx_file(data);

    // The decoder is supposed to return UTF-8
    assert!(std::str::from_utf8(decoded.as_bytes()).is_ok());

    // check if decode fn returned valid UTF-8
    let _ = hyperlynx_parser::parser::parse_hyperlynx(&decoded);
});
