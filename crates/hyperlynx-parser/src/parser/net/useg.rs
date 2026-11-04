// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::multispace0;
use winnow::combinator::{cut_err, peek};
use winnow::prelude::*;

use crate::model::UnroutedSegment;
use crate::parser::utils::{
    ExtractEntryErrorMessages, close_row, extract_float_entry, extract_string_entry, row_open,
    single_space, skip_ignored,
};

pub fn parse_useg_row(input: &mut &str) -> ModalResult<UnroutedSegment> {
    row_open(input, "USEG")?;

    single_space(input)?;

    let x1 = extract_float_entry(
        input,
        "X1",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "useg X1 key",
            key_error_description: "invalid X1 key name",
            value_error_label: "useg X1 value",
            value_error_description: "invalid X1 attribute value",
        },
    )?;

    single_space(input)?;

    let y1 = extract_float_entry(
        input,
        "Y1",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "useg Y1 key",
            key_error_description: "invalid Y1 key name",
            value_error_label: "useg Y1 value",
            value_error_description: "invalid Y1 attribute value",
        },
    )?;

    single_space(input)?;

    let layer1 = extract_string_entry(
        input,
        "L1",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "useg L1 key",
            key_error_description: "invalid L1 key name",
            value_error_label: "useg L1 value",
            value_error_description: "invalid L1 attribute value",
        },
    )?;

    single_space(input)?;

    let x2 = extract_float_entry(
        input,
        "X2",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "useg X2 key",
            key_error_description: "invalid X2 key name",
            value_error_label: "useg X2 value",
            value_error_description: "invalid X2 attribute value",
        },
    )?;

    single_space(input)?;

    let y2 = extract_float_entry(
        input,
        "Y2",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "useg Y2 key",
            key_error_description: "invalid Y2 key name",
            value_error_label: "useg Y2 value",
            value_error_description: "invalid Y2 attribute value",
        },
    )?;

    single_space(input)?;

    let layer2 = extract_string_entry(
        input,
        "L2",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "useg L2 key",
            key_error_description: "invalid L2 key name",
            value_error_label: "useg L2 value",
            value_error_description: "invalid L2 attribute value",
        },
    )?;

    single_space(input)?;

    let z = extract_float_entry(
        input,
        "Z",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "useg Z key",
            key_error_description: "invalid Z key name",
            value_error_label: "useg Z value",
            value_error_description: "invalid Z attribute value",
        },
    )?;

    single_space(input)?;

    let delay = extract_float_entry(
        input,
        "D",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "useg D key",
            key_error_description: "invalid D key name",
            value_error_label: "useg D value",
            value_error_description: "invalid D attribute value",
        },
    )?;

    single_space(input)?;

    let resistance = extract_float_entry(
        input,
        "R",
        ')',
        ExtractEntryErrorMessages {
            key_error_label: "useg R key",
            key_error_description: "invalid R key name",
            value_error_label: "useg R value",
            value_error_description: "invalid R attribute value",
        },
    )?;

    close_row(input)?;

    skip_ignored(input)?;

    Ok(UnroutedSegment {
        x1,
        y1,
        layer1,
        x2,
        y2,
        layer2,
        z,
        delay,
        resistance,
    })
}

pub fn parse_useg_row_with_err(input: &mut &str) -> ModalResult<UnroutedSegment> {
    peek((multispace0, "(USEG")).parse_next(input)?;

    cut_err(parse_useg_row).parse_next(input)
}

#[cfg(test)]
mod useg_row_parser_tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};

    #[test_case(
        "(USEG X1=8.2024606299 Y1=-5.0408464567 L1=\"F.Cu\" X2=8.2495078740 Y2=-5.0408464567 L2=\"B.Cu\" Z=50.0 D=0.25 R=0.5)\n  (...)",
        UnroutedSegment {
            x1: 8.2024606299,
            y1: -5.0408464567,
            layer1: String::from("F.Cu"),
            x2: 8.2495078740,
            y2: -5.0408464567,
            layer2: String::from("B.Cu"),
            z: 50.0,
            delay: 0.25,
            resistance: 0.5,
        }
    )]
    fn parses_useg_row(mut input: &str, expected: UnroutedSegment) {
        let useg = parse_useg_row(&mut input).unwrap();

        assert_eq!(useg, expected);
    }

    // opening row
    #[test_case(
        r#"xyz(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("opening row"),
            StrContext::Expected(StrContextValue::Description("("))
        )
    )]
    // row name
    #[test_case(
        r#"(xyzUSEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("row name"),
            StrContext::Expected(StrContextValue::Description("USEG"))
        )
    )]
    // X1 entry
    #[test_case(
        r#"(USEG xyzX1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg X1 key"),
            StrContext::Expected(StrContextValue::Description("invalid X1 key name"))
        ); "before X1"
    )]
    #[test_case(
        r#"(USEG X1xyz=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg X1 key"),
            StrContext::Expected(StrContextValue::Description("invalid X1 key name"))
        ); "after X1"
    )]
    #[test_case(
        r#"(USEG X1=xyz1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg X1 value"),
            StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
        ); "before X1 value"
    )]
    #[test_case(
        r#"(USEG X1=1xyz Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg X1 value"),
            StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
        ); "after X1 value"
    )]
    // Y1 entry
    #[test_case(
        r#"(USEG X1=1 xyzY1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg Y1 key"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 key name"))
        ); "before Y1 key"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1xyz=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg Y1 key"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 key name"))
        ); "after Y1 key"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=xyz2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg Y1 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
        ); "before Y1 value"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2xyz L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg Y1 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
        ); "after Y1 value"
    )]
    // L1 entry
    #[test_case(
        r#"(USEG X1=1 Y1=2 xyzL1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg L1 key"),
            StrContext::Expected(StrContextValue::Description("invalid L1 key name"))
        ); "before L1 key"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1xyz="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg L1 key"),
            StrContext::Expected(StrContextValue::Description("invalid L1 key name"))
        ); "after L1 key"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("single space"),
            StrContext::Expected(StrContextValue::Description("invalid indentation, needs single space"))
        ); "missing second double quote L1"
    )]
    // X2 entry
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" xyzX2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg X2 key"),
            StrContext::Expected(StrContextValue::Description("invalid X2 key name"))
        ); "before X2"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2xyz=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg X2 key"),
            StrContext::Expected(StrContextValue::Description("invalid X2 key name"))
        ); "after X2"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=xyz3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg X2 value"),
            StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
        ); "before X2 value"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3xyz Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg X2 value"),
            StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
        ); "after X2 value"
    )]
    // Y2 entry
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 xyzY2=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg Y2 key"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 key name"))
        ); "before Y2 key"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2xyz=4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg Y2 key"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 key name"))
        ); "after Y2 key"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=xyz4 L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg Y2 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
        ); "before Y2 value"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4xyz L2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg Y2 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
        ); "after Y2 value"
    )]
    // L2 entry
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 xyzL2="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg L2 key"),
            StrContext::Expected(StrContextValue::Description("invalid L2 key name"))
        ); "before L2 key"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2xyz="B.Cu" Z=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg L2 key"),
            StrContext::Expected(StrContextValue::Description("invalid L2 key name"))
        ); "after L2 key"
    )]
    // Z entry
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" xyzZ=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg Z key"),
            StrContext::Expected(StrContextValue::Description("invalid Z key name"))
        ); "before Z"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Zxyz=50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg Z key"),
            StrContext::Expected(StrContextValue::Description("invalid Z key name"))
        ); "after Z"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=xyz50 D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg Z value"),
            StrContext::Expected(StrContextValue::Description("invalid Z attribute value"))
        ); "before Z value"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50xyz D=0.25 R=0.5)"#,
        (
            StrContext::Label("useg Z value"),
            StrContext::Expected(StrContextValue::Description("invalid Z attribute value"))
        ); "after Z value"
    )]
    // D entry
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 xyzD=0.25 R=0.5)"#,
        (
            StrContext::Label("useg D key"),
            StrContext::Expected(StrContextValue::Description("invalid D key name"))
        ); "before D"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 Dxyz=0.25 R=0.5)"#,
        (
            StrContext::Label("useg D key"),
            StrContext::Expected(StrContextValue::Description("invalid D key name"))
        ); "after D"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=xyz0.25 R=0.5)"#,
        (
            StrContext::Label("useg D value"),
            StrContext::Expected(StrContextValue::Description("invalid D attribute value"))
        ); "before D value"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25xyz R=0.5)"#,
        (
            StrContext::Label("useg D value"),
            StrContext::Expected(StrContextValue::Description("invalid D attribute value"))
        ); "after D value"
    )]
    // R entry
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 xyzR=0.5)"#,
        (
            StrContext::Label("useg R key"),
            StrContext::Expected(StrContextValue::Description("invalid R key name"))
        ); "before R"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 Rxyz=0.5)"#,
        (
            StrContext::Label("useg R key"),
            StrContext::Expected(StrContextValue::Description("invalid R key name"))
        ); "after R"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=xyz0.5)"#,
        (
            StrContext::Label("useg R value"),
            StrContext::Expected(StrContextValue::Description("invalid R attribute value"))
        ); "before R value"
    )]
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5xyz)"#,
        (
            StrContext::Label("useg R value"),
            StrContext::Expected(StrContextValue::Description("invalid R attribute value"))
        ); "after R value"
    )]
    // closing row
    #[test_case(
        r#"(USEG X1=1 Y1=2 L1="F.Cu" X2=3 Y2=4 L2="B.Cu" Z=50 D=0.25 R=0.5 xyz)"#,
        (
            StrContext::Label("closing row"),
            StrContext::Expected(StrContextValue::Description(")"))
        )
    )]
    fn rejects_malformed_useg_row(mut input: &str, expected_errors: (StrContext, StrContext)) {
        let err = parse_useg_row(&mut input).unwrap_err();

        let errors = match err {
            ErrMode::Backtrack(ref err) | ErrMode::Cut(ref err) => {
                let mut iter = err.context();

                (iter.next().unwrap(), iter.next().unwrap())
            }
            ErrMode::Incomplete(_) => todo!(),
        };

        assert_eq!(*errors.0, expected_errors.0);
        assert_eq!(*errors.1, expected_errors.1);
    }
}
