// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::multispace0;
use winnow::combinator::{cut_err, peek};
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

pub fn parse_line_row_with_err(input: &mut &str) -> ModalResult<(f64, f64)> {
    peek((multispace0, "(LINE")).parse_next(input)?;

    cut_err(parse_line_row).parse_next(input)
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

    // opening row
    #[test_case(
        r#"xyz(LINE X=7.9166411024 Y=-4.5833994094)"#,
        (
            StrContext::Label("opening row"),
            StrContext::Expected(StrContextValue::Description("("))
        )
    )]
    // row name
    #[test_case(
        r#"(xyzLINE X=7.9166411024 Y=-4.5833994094)"#,
        (
            StrContext::Label("row name"),
            StrContext::Expected(StrContextValue::Description("LINE"))
        )
    )]
    #[test_case(
        r#"(LINExyz X=7.9166411024 Y=-4.5833994094)"#,
        (
            StrContext::Label("row name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after row name"))
        )
    )]
    // X entry
    #[test_case(
        r#"(LINE xyzX=7.9166411024 Y=-4.5833994094)"#,
        (
            StrContext::Label("line X key"),
            StrContext::Expected(StrContextValue::Description("invalid X key name"))
        ); "before X key"
    )]
    #[test_case(
        r#"(LINE Xxyz=7.9166411024 Y=-4.5833994094)"#,
        (
            StrContext::Label("line X key"),
            StrContext::Expected(StrContextValue::Description("invalid X key name"))
        ); "after X key"
    )]
    #[test_case(
        r#"(LINE X=xyz7.9166411024 Y=-4.5833994094)"#,
        (
            StrContext::Label("line X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "before X value"
    )]
    #[test_case(
        r#"(LINE X=7xyz.9166411024 Y=-4.5833994094)"#,
        (
            StrContext::Label("line X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "inside X value"
    )]
    #[test_case(
        r#"(LINE X=7.xyz9166411024 Y=-4.5833994094)"#,
        (
            StrContext::Label("line X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "inside X value"
    )]
    #[test_case(
        r#"(LINE X=7.916xyz6411024 Y=-4.5833994094)"#,
        (
            StrContext::Label("line X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "inside X value"
    )]
    #[test_case(
        r#"(LINE X=7.9166411024xyz Y=-4.5833994094)"#,
        (
            StrContext::Label("line X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "after X value"
    )]
    // Y entry
    #[test_case(
        r#"(LINE X=7.9166411024 xyzY=-4.5833994094)"#,
        (
            StrContext::Label("line Y key"),
            StrContext::Expected(StrContextValue::Description("invalid Y key name"))
        ); "before Y key"
    )]
    #[test_case(
        r#"(LINE X=7.9166411024 Yxyz=-4.5833994094)"#,
        (
            StrContext::Label("line Y key"),
            StrContext::Expected(StrContextValue::Description("invalid Y key name"))
        ); "after Y key"
    )]
    #[test_case(
        r#"(LINE X=7.9166411024 Y=xyz-4.5833994094)"#,
        (
            StrContext::Label("line Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "before Y value"
    )]
    #[test_case(
        r#"(LINE X=7.9166411024 Y=-4xyz.5833994094)"#,
        (
            StrContext::Label("line Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "inside Y value"
    )]
    #[test_case(
        r#"(LINE X=7.9166411024 Y=-4.xyz5833994094)"#,
        (
            StrContext::Label("line Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "inside Y value"
    )]
    #[test_case(
        r#"(LINE X=7.9166411024 Y=-4.583xyz3994094)"#,
        (
            StrContext::Label("line Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "inside Y value"
    )]
    #[test_case(
        r#"(LINE X=7.9166411024 Y=-4.5833994094xyz)"#,
        (
            StrContext::Label("line Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "after Y value"
    )]
    // closing row
    #[test_case(
        r#"(LINE X=7.9166411024 Y=-4.5833994094 xyz)"#,
        (
            StrContext::Label("closing row"),
            StrContext::Expected(StrContextValue::Description(")"))
        )
    )]
    fn rejects_malformed_line_row(
        mut input: &str,
        expected_errors: (StrContext, StrContext)
    ) {
        let err = parse_line_row(&mut input).unwrap_err();

        let errors = match err {
            ErrMode::Backtrack(ref err) | ErrMode::Cut(ref err) => {
                let mut iter = err.context().into_iter();
                let errors = (iter.next().unwrap(), iter.next().unwrap());

                errors
            }
            ErrMode::Incomplete(_) => todo!(),
        };

        assert_eq!(*errors.0, expected_errors.0);
        assert_eq!(*errors.1, expected_errors.1);
    }
}
