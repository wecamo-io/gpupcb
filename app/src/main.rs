// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use encoding_rs::{UTF_16BE, UTF_16LE, WINDOWS_1252};
use hyperlynx_parser::model::Board;
use hyperlynx_parser::parser::parse_hyperlynx;
use std::{env, fs};

fn decode_hyperlynx_file(bytes: &[u8]) -> String {
    match bytes {
        // UTF-16
        [0xFF, 0xFE, rest @ ..] => UTF_16LE.decode(rest).0.to_string(),
        [0xFE, 0xFF, rest @ ..] => UTF_16BE.decode(rest).0.to_string(),

        // UTF-8
        _ if std::str::from_utf8(bytes).is_ok() => String::from_utf8(bytes.to_vec()).unwrap(),

        // place WINDOWS_1252 decoding as fallback
        _ => WINDOWS_1252.decode(bytes).0.to_string(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (file_path, message): (&str, &str) = match args.get(1) {
        Some(path) => (path, "Parsing received file..."),
        None => (
            "../crates/hyperlynx-parser/tests/fixtures/kicad.hyp",
            "Parsing default file...",
        ), // fallback to test data
    };

    let content: String = match fs::read(file_path) {
        Ok(bytes) => decode_hyperlynx_file(&bytes),
        Err(error) => panic!(
            "Couldn't read the file.\nMake sure it has the right format (.hyp or .HYP) and that the path is correct\n{error:#?}"
        ),
    };

    println!("{}", message);

    let board = match parse_hyperlynx(&content) {
        Ok(board) => board,
        Err(err) => {
            eprintln!("Failed to parse HYP file:\n{err:#?}");
            return;
        }
    };

    let Board {
        version,
        units,
        outline,
        stackup,
        nets,
        devices,
        padstacks,
    } = board;

    println!("----------");
    println!("BOARD INFO");
    println!("VERSION: {}", version);
    println!("UNITS: {:?}", units);
    println!("OUTLINE COUNT: {:?}", outline.len());
    println!("STACKUP COUNT: {:?}", stackup.len());
    println!("NETS COUNT: {:?}", nets.len());
    println!("DEVICES COUNT: {:?}", devices.len());
    println!("PADSTACKS COUNT: {:?}", padstacks.len());
}
