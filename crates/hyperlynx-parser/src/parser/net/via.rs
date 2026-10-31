// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::multispace0;
use winnow::combinator::{cut_err, peek};
use winnow::{ModalResult, Parser};

use crate::model::Via;
use crate::parser::utils::{
    ExtractEntryErrorMessages, close_row, extract_float_entry, extract_unsigned_integer_entry,
    row_open, single_space, skip_ignored,
};

pub fn parse_via_row(input: &mut &str) -> ModalResult<Via> {
    row_open(input, "VIA")?;

    single_space(input)?;

    let x = extract_float_entry(
        input,
        "X",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "via X key",
            key_error_description: "invalid X key name",
            value_error_label: "via X value",
            value_error_description: "invalid X attribute value",
        },
    )?;

    single_space(input)?;

    let y = extract_float_entry(
        input,
        "Y",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "via Y key",
            key_error_description: "invalid Y key name",
            value_error_label: "via Y value",
            value_error_description: "invalid Y attribute value",
        },
    )?;

    single_space(input)?;

    let padstack_id = extract_unsigned_integer_entry(
        input,
        "P",
        ExtractEntryErrorMessages {
            key_error_label: "via P key",
            key_error_description: "invalid P key name",
            value_error_label: "via P value",
            value_error_description: "invalid P attribute value",
        },
    )?;

    close_row(input)?;

    skip_ignored(input)?;

    Ok(Via { x, y, padstack_id })
}

pub fn parse_via_row_with_err(input: &mut &str) -> ModalResult<Via> {
    peek((multispace0, "(VIA")).parse_next(input)?;

    cut_err(parse_via_row).parse_next(input)
}

#[cfg(test)]
mod via_row_parser_tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};

    #[test_case(
        "(VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (...)",
        Via {
            x: 8.1917322835,
            y: -5.0476377953,
            padstack_id: 18,
        }
    )]
    fn parses_via_row(mut input: &str, expected: Via) {
        let via = parse_via_row(&mut input).unwrap();

        assert_eq!(via, expected);
    }

    // opening row
    #[test_case(
        r#"xyz(VIA X=8.1917322835 Y=-5.0476377953 P=18)"#,
        (
            StrContext::Label("opening row"),
            StrContext::Expected(StrContextValue::Description("("))
        )
    )]
    // row name
    #[test_case(
        r#"(xyzVIA X=8.1917322835 Y=-5.0476377953 P=18)"#,
        (
            StrContext::Label("row name"),
            StrContext::Expected(StrContextValue::Description("VIA"))
        )
    )]
    #[test_case(
        r#"(VIAxyz X=8.1917322835 Y=-5.0476377953 P=18)"#,
        (
            StrContext::Label("row name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after row name"))
        )
    )]
    // X entry
    #[test_case(
        r#"(VIA xyzX=8.1917322835 Y=-5.0476377953 P=18)"#,
        (
            StrContext::Label("via X key"),
            StrContext::Expected(StrContextValue::Description("invalid X key name"))
        ); "before X key"
    )]
    #[test_case(
        r#"(VIA Xxyz=8.1917322835 Y=-5.0476377953 P=18)"#,
        (
            StrContext::Label("via X key"),
            StrContext::Expected(StrContextValue::Description("invalid X key name"))
        ); "after X key"
    )]
    #[test_case(
        r#"(VIA X=xyz8.1917322835 Y=-5.0476377953 P=18)"#,
        (
            StrContext::Label("via X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "before X value"
    )]
    #[test_case(
        r#"(VIA X=8xyz.1917322835 Y=-5.0476377953 P=18)"#,
        (
            StrContext::Label("via X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "inside X value"
    )]
    #[test_case(
        r#"(VIA X=8.xyz1917322835 Y=-5.0476377953 P=18)"#,
        (
            StrContext::Label("via X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "inside X value"
    )]
    #[test_case(
        r#"(VIA X=8.1917322835xyz Y=-5.0476377953 P=18)"#,
        (
            StrContext::Label("via X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "after X value"
    )]
    // Y entry
    #[test_case(
        r#"(VIA X=8.1917322835 xyzY=-5.0476377953 P=18)"#,
        (
            StrContext::Label("via Y key"),
            StrContext::Expected(StrContextValue::Description("invalid Y key name"))
        ); "before Y key"
    )]
    #[test_case(
        r#"(VIA X=8.1917322835 Yxyz=-5.0476377953 P=18)"#,
        (
            StrContext::Label("via Y key"),
            StrContext::Expected(StrContextValue::Description("invalid Y key name"))
        ); "after Y key"
    )]
    #[test_case(
        r#"(VIA X=8.1917322835 Y=xyz-5.0476377953 P=18)"#,
        (
            StrContext::Label("via Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "before Y value"
    )]
    #[test_case(
        r#"(VIA X=8.1917322835 Y=-5xyz.0476377953 P=18)"#,
        (
            StrContext::Label("via Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "inside Y value"
    )]
    #[test_case(
        r#"(VIA X=8.1917322835 Y=-5.xyz0476377953 P=18)"#,
        (
            StrContext::Label("via Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "inside Y value"
    )]
    #[test_case(
        r#"(VIA X=8.1917322835 Y=-5.0476377953xyz P=18)"#,
        (
            StrContext::Label("via Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "after Y value"
    )]
    // P entry
    #[test_case(
        r#"(VIA X=8.1917322835 Y=-5.0476377953 xyzP=18)"#,
        (
            StrContext::Label("via P key"),
            StrContext::Expected(StrContextValue::Description("invalid P key name"))
        ); "before P key"
    )]
    #[test_case(
        r#"(VIA X=8.1917322835 Y=-5.0476377953 Pxyz=18)"#,
        (
            StrContext::Label("via P key"),
            StrContext::Expected(StrContextValue::Description("invalid P key name"))
        ); "after P key"
    )]
    #[test_case(
        r#"(VIA X=8.1917322835 Y=-5.0476377953 P=xyz18)"#,
        (
            StrContext::Label("via P value"),
            StrContext::Expected(StrContextValue::Description("invalid P attribute value"))
        ); "before P value"
    )]
    #[test_case(
        r#"(VIA X=8.1917322835 Y=-5.0476377953 P=xyz18)"#,
        (
            StrContext::Label("via P value"),
            StrContext::Expected(StrContextValue::Description("invalid P attribute value"))
        ); "inside P value"
    )]
    fn rejects_malformed_via_row(mut input: &str, expected_errors: (StrContext, StrContext)) {
        let err = parse_via_row(&mut input).unwrap_err();

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
