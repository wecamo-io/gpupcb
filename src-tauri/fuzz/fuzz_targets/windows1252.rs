#![no_main]

use gpupcb_app_lib::decode_hyperlynx_file;
use hyperlynx_parser::parser::parse_hyperlynx;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // because windows1252 encoding is currently fallback
    // just start the sequence with BOM that doesnt match
    // UTF-8 nor UTF-16 both versions
    let mut windows1252 = vec![0xFF];

    windows1252.extend_from_slice(data);
    let decoded = decode_hyperlynx_file(&windows1252);
    let _ = parse_hyperlynx(&decoded);
});
