#![no_main]

use gpupcb_app_lib::decode_hyperlynx_file;
use hyperlynx_parser::parser::parse_hyperlynx;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut utf16le = vec![0xFF, 0xFE];

    utf16le.extend_from_slice(data);
    let decoded = decode_hyperlynx_file(&utf16le);
    let _ = parse_hyperlynx(&decoded);
});
