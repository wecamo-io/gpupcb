// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use hyperlynx_parser::model::Board;
use hyperlynx_parser::parser::parse_hyperlynx;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (file_path, message): (&str, &str) = match args.get(1) {
        Some(path) => (path, "Parsing received file..."),
        None => (
            "../crates/hyperlynx-parser/tests/fixtures/kicad.hyp",
            "Parsing default file...",
        ), // fallback to test data
    };

    let content = match fs::read_to_string(file_path) {
        Ok(file) => file,
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
