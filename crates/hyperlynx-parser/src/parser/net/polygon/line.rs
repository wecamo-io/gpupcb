// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::{ModalResult, Parser};

use crate::parser::utils::{ExtractEntryErrorMessages, close_row, extract_float_entry, row_open};

pub fn parse_line_row(input: &mut &str) -> ModalResult<(f64, f64)> {
    row_open(input, "LINE")?;

    let x = extract_float_entry(
        input,
        "X",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "line X key",
            key_error_description: "invalid X key name",
            value_error_label: "line X value",
            value_error_description: "invalid X attribute value",
        }
    )?;

    let y = extract_float_entry(
        input,
        "Y",
        ')',
        ExtractEntryErrorMessages {
            key_error_label: "line Y key",
            key_error_description: "invalid Y key name",
            value_error_label: "line Y value",
            value_error_description: "invalid Y attribute value",
        }
    )?;

    close_row(input)?;

    Ok((x, y))
}

#[cfg(test)]
mod line_row_parser_tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};

    #[test_case(
        r#"(LINE X=7.9166411024 Y=-4.5833994094)"#,
        (7.9166411024, -4.5833994094)
    )]
    fn parses_line_row(mut input: &str, expected: (f64, f64)) {
        let line = parse_line_row(&mut input).unwrap();

        assert_eq!(line, expected);
    }
}
