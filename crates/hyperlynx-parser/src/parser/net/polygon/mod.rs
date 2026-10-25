// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::{ModalResult, Parser};
use winnow::combinator::repeat;

use crate::model::Polyvoid;
use crate::parser::net::polygon::line::parse_line_row;
use crate::parser::utils::{ExtractEntryErrorMessages, close_block, extract_float_entry, extract_unsigned_integer_entry, open_block};

mod line;

pub fn parse_polyvoid_block(input: &mut &str) -> ModalResult<Polyvoid> {
    open_block(input, "POLYVOID")?;

    let id = extract_unsigned_integer_entry(
        input,
        "ID",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polyvoid ID key",
            key_error_description: "invalid ID key name",
            value_error_label: "polyvoid ID value",
            value_error_description: "invalid ID attribute value",
        },
    )?;

    let x = extract_float_entry(
        input,
        "X",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polyvoid X key",
            key_error_description: "invalid X key name",
            value_error_label: "polyvoid X value",
            value_error_description: "invalid X attribute value",
        }
    )?;

    let y = extract_float_entry(
        input,
        "Y",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polyvoid Y key",
            key_error_description: "invalid Y key name",
            value_error_label: "polyvoid Y value",
            value_error_description: "invalid Y attribute value",
        }
    )?;

    let outline: Vec<(f64, f64)> = repeat(0.., parse_line_row).parse_next(input)?;

    close_block(input)?;

    Ok(Polyvoid {
        id,
        x,
        y,
        outline,
    })
}

#[cfg(test)]
mod polyvoid_block_parser_tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};

    #[test_case(
        "{POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }",
        Polyvoid {
            id: 1,
            x: 8.1200787402,
            y: -4.6751968504,
            outline: vec![(8.1200787402, -4.6634706693), (8.1200787402, -4.6751968504)],
        }
    )]
    fn parses_arc_row(mut input: &str, expected: Polyvoid) {
        let polyvoid = parse_polyvoid_block(&mut input).unwrap();

        assert_eq!(polyvoid, expected);
    }
}
