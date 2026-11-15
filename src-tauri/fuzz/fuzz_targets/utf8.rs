#![no_main]

use gpupcb_app_lib::decode_hyperlynx_file;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = decode_hyperlynx_file(data);
});
