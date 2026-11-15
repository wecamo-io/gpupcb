#![no_main]

use gpupcb_app_lib::decode_hyperlynx_file;
use hyperlynx_parser::parser::parse_hyperlynx;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut utf16be = vec![0xFE, 0xFC];

    utf16be.extend_from_slice(data);
    let decoded = decode_hyperlynx_file(&utf16be);
    let _ = parse_hyperlynx(&decoded);
});
