// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::multispace0;
use winnow::combinator::{cut_err, peek};
use winnow::prelude::*;

use crate::model::Pin;
use crate::parser::utils::{
    ExtractEntryErrorMessages, close_row, extract_float_entry, extract_quoted_string_entry,
    extract_unsigned_integer_entry, line_ending_with_two_spaces, parse_either_line_ending,
    row_open, single_line_ending, single_space,
};

pub fn parse_pin_row(input: &mut &str) -> ModalResult<Pin> {
    row_open(input, "PIN")?;

    single_space(input)?;

    let x = extract_float_entry(
        input,
        "X",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "pin X key",
            key_error_description: "invalid X key name",
            value_error_label: "pin X value",
            value_error_description: "invalid X attribute value",
        },
    )?;

    single_space(input)?;

    let y = extract_float_entry(
        input,
        "Y",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "pin Y key",
            key_error_description: "invalid Y key name",
            value_error_label: "pin Y value",
            value_error_description: "invalid Y attribute value",
        },
    )?;

    single_space(input)?;

    let component_reference = extract_quoted_string_entry(
        input,
        "R",
        ExtractEntryErrorMessages {
            key_error_label: "pin R key",
            key_error_description: "invalid R key name",
            value_error_label: "pin R value",
            value_error_description: "invalid R attribute value",
        },
    )?;

    single_space(input)?;

    let padstack_id = extract_unsigned_integer_entry(
        input,
        "P",
        ExtractEntryErrorMessages {
            key_error_label: "pin P key",
            key_error_description: "invalid P key name",
            value_error_label: "pin P value",
            value_error_description: "invalid P attribute value",
        },
    )?;

    close_row(input)?;

    parse_either_line_ending(
        input,
        line_ending_with_two_spaces,
        single_line_ending,
        (
            "line ending",
            "expected single line ending or new line with 2 spaces after",
        ),
    )?;

    Ok(Pin {
        x,
        y,
        component_reference: String::from(component_reference),
        padstack_id,
    })
}

pub fn parse_pin_row_with_err(input: &mut &str) -> ModalResult<Pin> {
    peek((multispace0, "(PIN")).parse_next(input)?;

    cut_err(parse_pin_row).parse_next(input)
}

#[cfg(test)]
mod pin_row_parser_tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};

    #[test_case(
        "(PIN X=8.2480314961 Y=-4.8622047244 R=\"R3.1\" P=0)\n  (...)",
        Pin {
            x: 8.2480314961,
            y: -4.8622047244,
            component_reference: String::from("R3.1"),
            padstack_id: 0,
        }
    )]
    #[test_case(
        "(PIN X=8.3572834646 Y=-4.5978346457 R=\"U2.2\" P=12)\n  (...)",
        Pin {
            x: 8.3572834646,
            y: -4.5978346457,
            component_reference: String::from("U2.2"),
            padstack_id: 12,
        }
    )]
    fn parses_pin_row(mut input: &str, expected: Pin) {
        let pin = parse_pin_row(&mut input).unwrap();

        assert_eq!(pin, expected);
    }

    // malformed opening row
    #[test_case(
        r#"xyz(PIN X=8.3572834646 Y=-4.5978346457 R="U2.2" P=12)"#,
        (
            StrContext::Label("opening row"),
            StrContext::Expected(StrContextValue::Description("("))
        )
    )]
    // row name
    #[test_case(
        r#"(xyzPIN X=8.3572834646 Y=-4.5978346457 R="U2.2" P=12)"#,
        (
            StrContext::Label("row name"),
            StrContext::Expected(StrContextValue::Description("PIN"))
        )
    )]
    #[test_case(
        r#"(PINxyz X=8.3572834646 Y=-4.5978346457 R="U2.2" P=12)"#,
        (
            StrContext::Label("row name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after row name"))
        )
    )]
    // X entry
    #[test_case(
        r#"(PIN xyzX=8.3572834646 Y=-4.5978346457 R="U2.2" P=12)"#,
        (
            StrContext::Label("pin X key"),
            StrContext::Expected(StrContextValue::Description("invalid X key name"))
        )
    )]
    #[test_case(
        r#"(PIN Xxyz=8.3572834646 Y=-4.5978346457 R="U2.2" P=12)"#,
        (
            StrContext::Label("pin X key"),
            StrContext::Expected(StrContextValue::Description("invalid X key name"))
        )
    )]
    #[test_case(
        r#"(PIN X=xyz8.3572834646 Y=-4.5978346457 R="U2.2" P=12)"#,
        (
            StrContext::Label("pin X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        )
    )]
    #[test_case(
        r#"(PIN X=8xyz.3572834646 Y=-4.5978346457 R="U2.2" P=12)"#,
        (
            StrContext::Label("pin X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        )
    )]
    #[test_case(
        r#"(PIN X=8.xyz3572834646 Y=-4.5978346457 R="U2.2" P=12)"#,
        (
            StrContext::Label("pin X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        )
    )]
    #[test_case(
        r#"(PIN X=8.3572834646xyz Y=-4.5978346457 R="U2.2" P=12)"#,
        (
            StrContext::Label("pin X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        )
    )]
    // Y entry
    #[test_case(
        r#"(PIN X=8.3572834646 xyzY=-4.5978346457 R="U2.2" P=12)"#,
        (
            StrContext::Label("pin Y key"),
            StrContext::Expected(StrContextValue::Description("invalid Y key name"))
        )
    )]
    #[test_case(
        r#"(PIN X=8.3572834646 Yxyz=-4.5978346457 R="U2.2" P=12)"#,
        (
            StrContext::Label("pin Y key"),
            StrContext::Expected(StrContextValue::Description("invalid Y key name"))
        )
    )]
    #[test_case(
        r#"(PIN X=8.3572834646 Y=xyz-4.5978346457 R="U2.2" P=12)"#,
        (
            StrContext::Label("pin Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        )
    )]
    #[test_case(
        r#"(PIN X=8.3572834646 Y=-4xyz.5978346457 R="U2.2" P=12)"#,
        (
            StrContext::Label("pin Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        )
    )]
    #[test_case(
        r#"(PIN X=8.3572834646 Y=-4.xyz5978346457 R="U2.2" P=12)"#,
        (
            StrContext::Label("pin Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        )
    )]
    #[test_case(
        r#"(PIN X=8.3572834646 Y=-4.5978346457xyz R="U2.2" P=12)"#,
        (
            StrContext::Label("pin Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        )
    )]
    // R entry
    #[test_case(
        r#"(PIN X=8.3572834646 Y=-4.5978346457 xyzR="U2.2" P=12)"#,
        (
            StrContext::Label("pin R key"),
            StrContext::Expected(StrContextValue::Description("invalid R key name"))
        )
    )]
    #[test_case(
        r#"(PIN X=8.3572834646 Y=-4.5978346457 Rxyz="U2.2" P=12)"#,
        (
            StrContext::Label("pin R key"),
            StrContext::Expected(StrContextValue::Description("invalid R key name"))
        )
    )]
    #[test_case(
        r#"(PIN X=8.3572834646 Y=-4.5978346457 R=xyz"U2.2" P=12)"#,
        (
            StrContext::Label("pin R value"),
            StrContext::Expected(StrContextValue::Description("invalid R attribute value"))
        )
    )]
    #[test_case(
        r#"(PIN X=8.3572834646 Y=-4.5978346457 R=U2.2" P=12)"#,
        (
            StrContext::Label("pin R value"),
            StrContext::Expected(StrContextValue::Description("invalid R attribute value"))
        )
    )]
    #[test_case(
        r#"(PIN X=8.3572834646 Y=-4.5978346457 R="U2.2 P=12)"#,
        (
            StrContext::Label("pin R value"),
            StrContext::Expected(StrContextValue::Description("invalid R attribute value"))
        )
    )]
    // P entry
    #[test_case(
        r#"(PIN X=8.3572834646 Y=-4.5978346457 R="U2.2" xyzP=12)"#,
        (
            StrContext::Label("pin P key"),
            StrContext::Expected(StrContextValue::Description("invalid P key name"))
        )
    )]
    #[test_case(
        r#"(PIN X=8.3572834646 Y=-4.5978346457 R="U2.2" Pxyz=12)"#,
        (
            StrContext::Label("pin P key"),
            StrContext::Expected(StrContextValue::Description("invalid P key name"))
        )
    )]
    #[test_case(
        r#"(PIN X=8.3572834646 Y=-4.5978346457 R="U2.2" P=xyz12)"#,
        (
            StrContext::Label("pin P value"),
            StrContext::Expected(StrContextValue::Description("invalid P attribute value"))
        )
    )]
    // closing row
    #[test_case(
        r#"(PIN X=8.3572834646 Y=-4.5978346457 R="U2.2" P=12xyz)"#,
        (
            StrContext::Label("closing row"),
            StrContext::Expected(StrContextValue::Description(")"))
        )
    )]
    fn rejects_malformed_pin_row(mut input: &str, expected_errors: (StrContext, StrContext)) {
        let err = parse_pin_row(&mut input).unwrap_err();

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
