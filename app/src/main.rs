// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use hyperlynx_parser::parser::parse;
use hyperlynx_parser::model::Board;

fn main() {
    // let content = std::fs::read_to_string("/path/to/file/example.hyp").unwrap();
    
    // let board = match parse(&content) {
    //     Ok(board) => board,
    //     Err(err) => {
    //         eprintln!("Failed to parse HYP file:\n{err:#?}");
    //         return;
    //     }
    // };

    // let Board {
    //     version,
    //     units,
    //     outline,
    //     stackup,
    //     nets,
    //     devices,
    //     padstacks
    // } = board;

    // println!("BOARD INFO");
    // println!("VERSION: {}", version);
    // println!("UNITS: {:?}", units);
    // println!("OUTLINE COUNT: {:?}", outline.len());
    // println!("STACKUP COUNT: {:?}", stackup.len());
    // println!("NETS COUNT: {:?}", nets.len());
    // println!("DEVICES COUNT: {:?}", devices.len());
    // println!("PADSTACKS COUNT: {:?}", padstacks.len());
}
