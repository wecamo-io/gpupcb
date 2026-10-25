// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::{ModalResult, Parser};

use crate::model::Segment;
use crate::parser::utils::{ExtractEntryErrorMessages, close_row, extract_float_entry, extract_quoted_string_entry, row_open};

pub fn parse_seg_row(input: &mut &str) -> ModalResult<Segment> {
    row_open(input, "SEG")?;

    let x1 = extract_float_entry(
        input,
        "X1",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "seg X1 key",
            key_error_description: "invalid X1 key name",
            value_error_label: "seg X1 value",
            value_error_description: "invalid X1 attribute value",
        },
    )?;

    let y1 = extract_float_entry(
        input,
        "Y1",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "seg Y1 key",
            key_error_description: "invalid Y1 key name",
            value_error_label: "seg Y1 value",
            value_error_description: "invalid Y1 attribute value",
        },
    )?;

    let x2 = extract_float_entry(
        input,
        "X2",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "seg X2 key",
            key_error_description: "invalid X2 key name",
            value_error_label: "seg X2 value",
            value_error_description: "invalid X2 attribute value",
        },
    )?;

    let y2 = extract_float_entry(
        input,
        "Y2",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "seg Y2 key",
            key_error_description: "invalid Y2 key name",
            value_error_label: "seg Y2 value",
            value_error_description: "invalid Y2 attribute value",
        },
    )?;

    let width = extract_float_entry(
        input,
        "W",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "seg W key",
            key_error_description: "invalid W key name",
            value_error_label: "seg W value",
            value_error_description: "invalid W attribute value",
        },
    )?;

    let layer = extract_quoted_string_entry(
        input,
        "L",
        ExtractEntryErrorMessages {
            key_error_label: "seg L key",
            key_error_description: "invalid L key name",
            value_error_label: "seg L value",
            value_error_description: "invalid L attribute value",
        },
    )?;

    close_row(input)?;

    Ok(Segment {
        x1,
        y1,
        x2,
        y2,
        width,
        layer: String::from(layer),
    })
}

#[cfg(test)]
mod seg_row_parser_tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};

    #[test_case(
        r#"(SEG X1=8.2024606299 Y1=-5.0408464567 X2=8.2495078740 Y2=-5.0408464567 W=0.0196850394 L="F.Cu")"#,
        Segment {
            x1: 8.2024606299,
            y1: -5.0408464567,
            x2: 8.2495078740,
            y2: -5.0408464567,
            width: 0.0196850394,
            layer: String::from("F.Cu"),
        }
    )]
    fn parses_seg_row(mut input: &str, expected: Segment) {
        let segment = parse_seg_row(&mut input).unwrap();

        assert_eq!(segment, expected);
    }

    // opening row
    #[test_case(
        r#"xyz(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("opening row"),
            StrContext::Expected(StrContextValue::Description("("))
        )
    )]
    // row name
    #[test_case(
        r#"(xyzSEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("row name"),
            StrContext::Expected(StrContextValue::Description("SEG"))
        )
    )]
    // X1 entry
    #[test_case(
        r#"(SEG xyzX1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg X1 key"),
            StrContext::Expected(StrContextValue::Description("invalid X1 key name"))
        ); "before X1"
    )]
    #[test_case(
        r#"(SEG X1xyz=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg X1 key"),
            StrContext::Expected(StrContextValue::Description("invalid X1 key name"))
        ); "after X1"
    )]
    #[test_case(
        r#"(SEG X1=xyz8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg X1 value"),
            StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
        ); "before X1 value"
    )]
    #[test_case(
        r#"(SEG X1=8xyz.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg X1 value"),
            StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
        ); "inside X1 value"
    )]
    #[test_case(
        r#"(SEG X1=8.xyz2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg X1 value"),
            StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
        ); "inside X1 value"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961xyz Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg X1 value"),
            StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
        ); "after X1 value"
    )]
    // Y1 entry
    #[test_case(
        r#"(SEG X1=8.2480314961 xyzY1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg Y1 key"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 key name"))
        ); "before Y1 key"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1xyz=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg Y1 key"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 key name"))
        ); "after Y1 key"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=xyz-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg Y1 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
        ); "before Y1 value"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4xyz.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg Y1 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
        ); "inside Y1 value"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.xyz8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg Y1 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
        ); "inside Y1 value"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709xyz X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg Y1 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
        ); "after Y1 value"
    )]
    // X2 entry
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 xyzX2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg X2 key"),
            StrContext::Expected(StrContextValue::Description("invalid X2 key name"))
        ); "before X2"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2xyz=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg X2 key"),
            StrContext::Expected(StrContextValue::Description("invalid X2 key name"))
        ); "after X2"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=xyz8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg X2 value"),
            StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
        ); "before X2 value"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8xyz.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg X2 value"),
            StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
        ); "inside X2 value"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.xyz3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg X2 value"),
            StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
        ); "inside X2 value"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110xyz Y2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg X2 value"),
            StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
        ); "after X2 value"
    )]
    // Y2 entry
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 xyzY2=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg Y2 key"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 key name"))
        ); "before Y2 key"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2xyz=-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg Y2 key"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 key name"))
        ); "after Y2 key"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=xyz-4.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg Y2 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
        ); "before Y2 value"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4xyz.9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg Y2 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
        ); "inside Y2 value"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.xyz9429133858 W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg Y2 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
        ); "inside Y2 value"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858xyz W=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg Y2 value"),
            StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
        ); "after Y2 value"
    )]
    // W entry
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 xyzW=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg W key"),
            StrContext::Expected(StrContextValue::Description("invalid W key name"))
        ); "before W key"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 Wxyz=0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg W key"),
            StrContext::Expected(StrContextValue::Description("invalid W key name"))
        ); "after W key"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=xyz0.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg W value"),
            StrContext::Expected(StrContextValue::Description("invalid W attribute value"))
        ); "before W value"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0xyz.0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg W value"),
            StrContext::Expected(StrContextValue::Description("invalid W attribute value"))
        ); "inside W value"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.xyz0118110236 L="F.Cu")"#,
        (
            StrContext::Label("seg W value"),
            StrContext::Expected(StrContextValue::Description("invalid W attribute value"))
        ); "inside W value"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236xyz L="F.Cu")"#,
        (
            StrContext::Label("seg W value"),
            StrContext::Expected(StrContextValue::Description("invalid W attribute value"))
        ); "after W value"
    )]
    // L entry
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 xyzL="F.Cu")"#,
        (
            StrContext::Label("seg L key"),
            StrContext::Expected(StrContextValue::Description("invalid L key name"))
        ); "before L key"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 Lxyz="F.Cu")"#,
        (
            StrContext::Label("seg L key"),
            StrContext::Expected(StrContextValue::Description("invalid L key name"))
        ); "after L key"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L=xyz"F.Cu")"#,
        (
            StrContext::Label("seg L value"),
            StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
        ); "before L value"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L=F.Cu")"#,
        (
            StrContext::Label("seg L value"),
            StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
        ); "missing first double quote"
    )]
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu)"#,
        (
            StrContext::Label("seg L value"),
            StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
        ); "missing second double quote"
    )]
    // closing row
    #[test_case(
        r#"(SEG X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 W=0.0118110236 L="F.Cu"xyz)"#,
        (
            StrContext::Label("closing row"),
            StrContext::Expected(StrContextValue::Description(")"))
        )
    )]
    fn rejects_malformed_seg_row(
        mut input: &str,
        expected_errors: (StrContext, StrContext)
    ) {
        let err = parse_seg_row(&mut input).unwrap_err();

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
