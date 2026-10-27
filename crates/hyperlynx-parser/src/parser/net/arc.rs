// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::multispace0;
use winnow::combinator::{cut_err, peek};
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

pub fn parse_arc_row_with_err(input: &mut &str) -> ModalResult<Arc> {
    peek((multispace0, "(ARC")).parse_next(input)?;

    cut_err(parse_arc_row).parse_next(input)
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
        let arc = parse_arc_row(&mut input).unwrap();

        assert_eq!(arc, expected);
    }

    // opening row
    #[test_case(
        r#"xyz(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("opening row"),
            StrContext::Expected(StrContextValue::Description("("))
        )
    )]
    // row name
    #[test_case(
        r#"(xyzARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("row name"),
            StrContext::Expected(StrContextValue::Description("ARC"))
        )
    )]
    #[test_case(
        r#"(ARCxyz X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("row name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after row name"))
        )
    )]
    // X1 entry
    #[test_case(
        r#"(ARC xyzX1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc X1 key"),
            StrContext::Expected(StrContextValue::Description("invalid X1 key name"))
        ); "before X1 key"
    )]
    #[test_case(
        r#"(ARC X1xyz=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc X1 key"),
            StrContext::Expected(StrContextValue::Description("invalid X1 key name"))
        ); "after X1 key"
    )]
    #[test_case(
        r#"(ARC X1=xyz8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc X1 value"),
            StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
        ); "before X1 value"
    )]
    #[test_case(
        r#"(ARC X1=8xyz.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc X1 value"),
            StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
        ); "inside X1 value"
    )]
    #[test_case(
        r#"(ARC X1=8.xyz2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc X1 value"),
            StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
        ); "inside X1 value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961xyz Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc X1 value"),
            StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
        ); "after X1 value"
    )]
    // Y1 entry
    #[test_case(
        r#"(ARC X1=8.2480314961 xyzY1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc Y1 key"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 key name"))
        ); "before Y1 key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1xyz=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc Y1 key"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 key name"))
        ); "after Y1 key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=xyz-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc Y1 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
        ); "before Y1 value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4xyz.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc Y1 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
        ); "inside Y1 value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.xyz8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc Y1 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
        ); "inside Y1 value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709xyz X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc Y1 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
        ); "after Y1 value"
    )]
    // X2 entry
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 xyzX2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc X2 key"),
            StrContext::Expected(StrContextValue::Description("invalid X2 key name"))
        ); "before X2 key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2xyz=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc X2 key"),
            StrContext::Expected(StrContextValue::Description("invalid X2 key name"))
        ); "after X2 key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=xyz8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc X2 value"),
            StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
        ); "before X2 value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8xyz.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc X2 value"),
            StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
        ); "inside X2 value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.xyz3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc X2 value"),
            StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
        ); "inside X2 value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110xyz Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc X2 value"),
            StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
        ); "after X2 value"
    )]
    // Y2 entry
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 xyzY2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc Y2 key"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 key name"))
        ); "before Y2 key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2xyz=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc Y2 key"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 key name"))
        ); "after Y2 key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=xyz-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc Y2 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
        ); "before Y2 value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4xyz.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc Y2 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
        ); "inside Y2 value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.xyz9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc Y2 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
        ); "inside Y2 value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858xyz XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc Y2 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
        ); "after Y2 value"
    )]
    // XC entry
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 xyzXC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc XC key"),
            StrContext::Expected(StrContextValue::Description("invalid XC key name"))
        ); "before XC key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XCxyz=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc XC key"),
            StrContext::Expected(StrContextValue::Description("invalid XC key name"))
        ); "after XC key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=xyz0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc XC value"),
            StrContext::Expected(StrContextValue::Description("invalid XC attribute value"))
        ); "before XC value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0xyz.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc XC value"),
            StrContext::Expected(StrContextValue::Description("invalid XC attribute value"))
        ); "inside XC value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.xyz5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc XC value"),
            StrContext::Expected(StrContextValue::Description("invalid XC attribute value"))
        ); "inside XC value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5xyz YC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc XC value"),
            StrContext::Expected(StrContextValue::Description("invalid XC attribute value"))
        ); "after XC value"
    )]
    // YC entry
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 xyzYC=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc YC key"),
            StrContext::Expected(StrContextValue::Description("invalid YC key name"))
        ); "before YC key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YCxyz=0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc YC key"),
            StrContext::Expected(StrContextValue::Description("invalid YC key name"))
        ); "after YC key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=xyz0.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc YC value"),
            StrContext::Expected(StrContextValue::Description("invalid YC attribute value"))
        ); "before YC value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0xyz.5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc YC value"),
            StrContext::Expected(StrContextValue::Description("invalid YC attribute value"))
        ); "inside YC value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.xyz5 R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc YC value"),
            StrContext::Expected(StrContextValue::Description("invalid YC attribute value"))
        ); "inside YC value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5xyz R=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc YC value"),
            StrContext::Expected(StrContextValue::Description("invalid YC attribute value"))
        ); "after YC value"
    )]
    // R entry
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 xyzR=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc R key"),
            StrContext::Expected(StrContextValue::Description("invalid R key name"))
        ); "before R key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 Rxyz=1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc R key"),
            StrContext::Expected(StrContextValue::Description("invalid R key name"))
        ); "after R key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=xyz1.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc R value"),
            StrContext::Expected(StrContextValue::Description("invalid R attribute value"))
        ); "before R value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1xyz.0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc R value"),
            StrContext::Expected(StrContextValue::Description("invalid R attribute value"))
        ); "inside R value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.xyz0 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc R value"),
            StrContext::Expected(StrContextValue::Description("invalid R attribute value"))
        ); "inside R value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0xyz W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc R value"),
            StrContext::Expected(StrContextValue::Description("invalid R attribute value"))
        ); "after R value"
    )]
    // W entry
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 xyzW=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc W key"),
            StrContext::Expected(StrContextValue::Description("invalid W key name"))
        ); "before W key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 Wxyz=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc W key"),
            StrContext::Expected(StrContextValue::Description("invalid W key name"))
        ); "after W key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=xyz0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc W value"),
            StrContext::Expected(StrContextValue::Description("invalid W attribute value"))
        ); "before W value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0xyz.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc W value"),
            StrContext::Expected(StrContextValue::Description("invalid W attribute value"))
        ); "inside W value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.xyz0118110236 L="F.Cu")"#,
        (
            StrContext::Label("arc W value"),
            StrContext::Expected(StrContextValue::Description("invalid W attribute value"))
        ); "inside W value"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236xyz L="F.Cu")"#,
        (
            StrContext::Label("arc W value"),
            StrContext::Expected(StrContextValue::Description("invalid W attribute value"))
        ); "after W value"
    )]
    // L entry
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 xyzL="F.Cu")"#,
        (
            StrContext::Label("arc L key"),
            StrContext::Expected(StrContextValue::Description("invalid L key name"))
        ); "before L key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 Lxyz="F.Cu")"#,
        (
            StrContext::Label("arc L key"),
            StrContext::Expected(StrContextValue::Description("invalid L key name"))
        ); "after L key"
    )]
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=xyz"F.Cu")"#,
        (
            StrContext::Label("arc L value"),
            StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
        ); "before L value"
    )]
    // closing row
    #[test_case(
        r#"(ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L="F.Cu"xyz)"#,
        (
            StrContext::Label("closing row"),
            StrContext::Expected(StrContextValue::Description(")"))
        ); "after L value"
    )]
    fn rejects_malformed_arc_row(
        mut input: &str,
        expected_errors: (StrContext, StrContext)
    ) {
        let err = parse_arc_row(&mut input).unwrap_err();

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
