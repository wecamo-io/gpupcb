// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::{ModalResult, Parser};

use crate::model::Arc;
use crate::parser::utils::{ExtractEntryErrorMessages, close_row, extract_float_entry, extract_quoted_string_entry, row_open};

pub fn parse_arc_row(input: &mut &str) -> ModalResult<Arc> {
    row_open(input, "ARC")?;

    let x1 = extract_float_entry(
        input,
        "X1",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "arc X1 key",
            key_error_description: "invalid X1 key name",
            value_error_label: "arc X1 value",
            value_error_description: "invalid X1 attribute value",
        },
    )?;

    let y1 = extract_float_entry(
        input,
        "Y1",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "arc Y1 key",
            key_error_description: "invalid Y1 key name",
            value_error_label: "arc Y1 value",
            value_error_description: "invalid Y1 attribute value",
        },
    )?;

    let x2 = extract_float_entry(
        input,
        "X2",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "arc X2 key",
            key_error_description: "invalid X2 key name",
            value_error_label: "arc X2 value",
            value_error_description: "invalid X2 attribute value",
        },
    )?;

    let y2 = extract_float_entry(
        input,
        "Y2",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "arc Y2 key",
            key_error_description: "invalid Y2 key name",
            value_error_label: "arc Y2 value",
            value_error_description: "invalid Y2 attribute value",
        },
    )?;

    let xc = extract_float_entry(
        input,
        "XC",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "arc XC key",
            key_error_description: "invalid XC key name",
            value_error_label: "arc XC value",
            value_error_description: "invalid XC attribute value",
        },
    )?;

    let yc = extract_float_entry(
        input,
        "YC",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "arc YC key",
            key_error_description: "invalid YC key name",
            value_error_label: "arc YC value",
            value_error_description: "invalid YC attribute value",
        },
    )?;

    let radius = extract_float_entry(
        input,
        "R",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "arc R key",
            key_error_description: "invalid R key name",
            value_error_label: "arc R value",
            value_error_description: "invalid R attribute value",
        },
    )?;

    let width = extract_float_entry(
        input,
        "W",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "arc W key",
            key_error_description: "invalid W key name",
            value_error_label: "arc W value",
            value_error_description: "invalid W attribute value",
        },
    )?;

    let layer = extract_quoted_string_entry(
        input,
        "L",
        ExtractEntryErrorMessages {
            key_error_label: "arc L key",
            key_error_description: "invalid L key name",
            value_error_label: "arc L value",
            value_error_description: "invalid L attribute value",
        },
    )?;

    close_row(input)?;

    Ok(Arc {
        x1,
        y1,
        x2,
        y2,
        xc,
        yc,
        radius,
        width,
        layer: String::from(layer),
    })
}

#[cfg(test)]
mod arc_row_parser_tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};

    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        Arc {
            x1: 8.2480314961,
            y1: -4.8854330709,
            x2: 8.3055118110,
            y2: -4.9429133858,
            xc: 0.5,
            yc: 0.5,
            radius: 1.0,
            width: 0.0118110236,
            layer: String::from("F.Cu"),
        }
    )]
    fn parses_arc_row(mut input: &str, expected: Arc) {
        let via = parse_arc_row(&mut input).unwrap();

        assert_eq!(via, expected);
    }
}
