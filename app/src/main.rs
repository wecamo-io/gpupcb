// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use hyperlynx_parser::model::Board;
use hyperlynx_parser::parser::parse_hyperlynx;

fn main() {
    let content_kicad =
        std::fs::read_to_string("../crates/hyperlynx-parser/tests/fixtures/kicad.hyp").unwrap();
    let content_altium =
        std::fs::read_to_string("../crates/hyperlynx-parser/tests/fixtures/altium designer.HYP")
            .unwrap();

    {
        let board = match parse_hyperlynx(&content_kicad) {
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

        println!("BOARD INFO");
        println!("VERSION: {}", version);
        println!("UNITS: {:?}", units);
        println!("OUTLINE COUNT: {:?}", outline.len());
        println!("STACKUP COUNT: {:?}", stackup.len());
        println!("NETS COUNT: {:?}", nets.len());
        println!("DEVICES COUNT: {:?}", devices.len());
        println!("PADSTACKS COUNT: {:?}", padstacks.len());
    }

    println!("-------------------------------------------------");

    {
        let board = match parse_hyperlynx(&content_altium) {
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

        println!("BOARD INFO");
        println!("VERSION: {}", version);
        println!("UNITS: {:?}", units);
        println!("OUTLINE COUNT: {:?}", outline.len());
        println!("STACKUP COUNT: {:?}", stackup.len());
        println!("NETS COUNT: {:?}", nets.len());
        println!("DEVICES COUNT: {:?}", devices.len());
        println!("PADSTACKS COUNT: {:?}", padstacks.len());
    }
}
