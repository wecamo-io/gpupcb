// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::{ModalResult, Parser};
use winnow::ascii::digit1;
use winnow::combinator::delimited;
use winnow::token::{literal, take_until};

use crate::model::{Pad, PadShape};
use crate::parser::utils::{ExtractValueErrorMessages, close_row, extract_float_value, row_open};

fn parse_pad_row(input: &mut &str) -> ModalResult<Pad> {
    row_open(input, "")?;

    let layer = delimited('"', take_until(0.., '"'), '"').parse_next(input)?;

    literal(", ").parse_next(input)?;

    let shape = {
        let value = digit1.parse_next(input)?;

        match value {
            "0" => PadShape::Oval,
            "1" => PadShape::Roundrect,
            "2" => PadShape::Rectangle,
            _ => todo!(),
        }
    };

    literal(", ").parse_next(input)?;

    let size_x = extract_float_value(
        input,
        ',',
        ExtractValueErrorMessages {
            value_error_label: "pad size_x value",
            value_error_description: "invalid size_x value",
        }
    )?;

    literal(", ").parse_next(input)?;

    let size_y = extract_float_value(
        input,
        ',',
        ExtractValueErrorMessages {
            value_error_label: "pad size_y value",
            value_error_description: "invalid size_y value",
        }
    )?;

    literal(", ").parse_next(input)?;

    let angle = extract_float_value(
        input,
        ',',
        ExtractValueErrorMessages {
            value_error_label: "pad angle value",
            value_error_description: "invalid angle value",
        }
    )?;

    literal(", M").parse_next(input)?;

    close_row(input)?;

    Ok(Pad {
        layer: String::from(layer),
        shape,
        size_x,
        size_y,
        angle,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0.055118110, 90.0, M)"#,
        Pad {
            layer: String::from("F.Cu"),
            shape: PadShape::Rectangle,
            size_x: 0.047244094,
            size_y: 0.055118110,
            angle: 90.0,
        }
    )]
    #[test_case(
        r#"("B.Cu", 0, 0.047244094, 0.055118110, 180.0, M)"#,
        Pad {
            layer: String::from("B.Cu"),
            shape: PadShape::Oval,
            size_x: 0.047244094,
            size_y: 0.055118110,
            angle: 180.0,
        }
    )]
    #[test_case(
        r#"("F.Cu", 1, 0.047244094, 0.055118110, 270.0, M)"#,
        Pad {
            layer: String::from("F.Cu"),
            shape: PadShape::Roundrect,
            size_x: 0.047244094,
            size_y: 0.055118110,
            angle: 270.0,
        }
    )]
    fn parses_pad_row(mut input: &str, expected: Pad) {
        let pad = parse_pad_row.parse_next(&mut input).unwrap();

        assert_eq!(pad, expected);
    }
}
