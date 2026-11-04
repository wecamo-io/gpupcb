// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::multispace0;
use winnow::combinator::{cut_err, peek};
use winnow::prelude::*;

use crate::model::Curve;
use crate::parser::utils::{
    ExtractEntryErrorMessages, close_row, extract_float_entry, row_open, single_space, skip_ignored,
};

pub fn parse_curve_row(input: &mut &str) -> ModalResult<Curve> {
    row_open(input, "CURVE")?;

    single_space(input)?;

    let x1 = extract_float_entry(
        input,
        "X1",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "curve X1 key",
            key_error_description: "invalid X1 key name",
            value_error_label: "curve X1 value",
            value_error_description: "invalid X1 attribute value",
        },
    )?;

    single_space(input)?;

    let y1 = extract_float_entry(
        input,
        "Y1",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "curve Y1 key",
            key_error_description: "invalid Y1 key name",
            value_error_label: "curve Y1 value",
            value_error_description: "invalid Y1 attribute value",
        },
    )?;

    single_space(input)?;

    let x2 = extract_float_entry(
        input,
        "X2",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "curve X2 key",
            key_error_description: "invalid X2 key name",
            value_error_label: "curve X2 value",
            value_error_description: "invalid X2 attribute value",
        },
    )?;

    single_space(input)?;

    let y2 = extract_float_entry(
        input,
        "Y2",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "curve Y2 key",
            key_error_description: "invalid Y2 key name",
            value_error_label: "curve Y2 value",
            value_error_description: "invalid Y2 attribute value",
        },
    )?;

    single_space(input)?;

    let xc = extract_float_entry(
        input,
        "XC",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "curve XC key",
            key_error_description: "invalid XC key name",
            value_error_label: "curve XC value",
            value_error_description: "invalid XC attribute value",
        },
    )?;

    single_space(input)?;

    let yc = extract_float_entry(
        input,
        "YC",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "curve YC key",
            key_error_description: "invalid YC key name",
            value_error_label: "curve YC value",
            value_error_description: "invalid YC attribute value",
        },
    )?;

    single_space(input)?;

    let radius = extract_float_entry(
        input,
        "R",
        ')',
        ExtractEntryErrorMessages {
            key_error_label: "curve R key",
            key_error_description: "invalid R key name",
            value_error_label: "curve R value",
            value_error_description: "invalid R attribute value",
        },
    )?;

    close_row(input)?;

    skip_ignored(input)?;

    Ok(Curve {
        x1,
        y1,
        x2,
        y2,
        xc,
        yc,
        radius,
    })
}

pub fn parse_curve_row_with_err(input: &mut &str) -> ModalResult<Curve> {
    peek((multispace0, "(CURVE")).parse_next(input)?;

    cut_err(parse_curve_row).parse_next(input)
}

#[cfg(test)]
mod curve_row_parser_tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};

    #[test_case(
          "(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)\n  (...)",
          Curve {
              x1: 8.2480314961,
              y1: -4.8854330709,
              x2: 8.3055118110,
              y2: -4.9429133858,
              xc: 0.5,
              yc: 0.5,
              radius: 1.0,
          }
      )]
    fn parses_curve_row(mut input: &str, expected: Curve) {
        let curve = parse_curve_row(&mut input).unwrap();

        assert_eq!(curve, expected);
    }

    // opening row
    #[test_case(
          r#"xyz(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (
              StrContext::Label("opening row"),
              StrContext::Expected(StrContextValue::Description("("))
          )
      )]
    // row name
    #[test_case(
          r#"(xyzCURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (
              StrContext::Label("row name"),
              StrContext::Expected(StrContextValue::Description("CURVE"))
          )
      )]
    #[test_case(
          r#"(CURVExyz X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (
              StrContext::Label("row name boundary"),
              StrContext::Expected(StrContextValue::Description("valid character after row name"))
          )
      )]
    // X1 entry
    #[test_case(
          r#"(CURVE xyzX1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve X1 key"), StrContext::Expected(StrContextValue::Description("invalid X1 key name")));
          "before X1 key"
      )]
    #[test_case(
          r#"(CURVE X1xyz=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve X1 key"), StrContext::Expected(StrContextValue::Description("invalid X1 key name")));
          "after X1 key"
      )]
    #[test_case(
          r#"(CURVE X1=xyz8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve X1 value"), StrContext::Expected(StrContextValue::Description("invalid X1 attribute value")));
          "before X1 value"
      )]
    #[test_case(
          r#"(CURVE X1=8xyz.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve X1 value"), StrContext::Expected(StrContextValue::Description("invalid X1 attribute value")));
          "inside X1 value"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961xyz Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve X1 value"), StrContext::Expected(StrContextValue::Description("invalid X1 attribute value")));
          "after X1 value"
      )]
    // Y1 entry
    #[test_case(
          r#"(CURVE X1=8.2480314961 xyzY1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve Y1 key"), StrContext::Expected(StrContextValue::Description("invalid Y1 key name")));
          "before Y1 key"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1xyz=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve Y1 key"), StrContext::Expected(StrContextValue::Description("invalid Y1 key name")));
          "after Y1 key"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=xyz-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve Y1 value"), StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value")));
          "before Y1 value"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4xyz.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve Y1 value"), StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value")));
          "inside Y1 value"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709xyz X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve Y1 value"), StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value")));
          "after Y1 value"
      )]
    // X2 entry
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 xyzX2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve X2 key"), StrContext::Expected(StrContextValue::Description("invalid X2 key name")));
          "before X2 key"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2xyz=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve X2 key"), StrContext::Expected(StrContextValue::Description("invalid X2 key name")));
          "after X2 key"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=xyz8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve X2 value"), StrContext::Expected(StrContextValue::Description("invalid X2 attribute value")));
          "before X2 value"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8xyz.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve X2 value"), StrContext::Expected(StrContextValue::Description("invalid X2 attribute value")));
          "inside X2 value"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110xyz Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve X2 value"), StrContext::Expected(StrContextValue::Description("invalid X2 attribute value")));
          "after X2 value"
      )]
    // Y2 entry
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 xyzY2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve Y2 key"), StrContext::Expected(StrContextValue::Description("invalid Y2 key name")));
          "before Y2 key"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2xyz=-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve Y2 key"), StrContext::Expected(StrContextValue::Description("invalid Y2 key name")));
          "after Y2 key"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=xyz-4.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve Y2 value"), StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value")));
          "before Y2 value"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4xyz.9429133858 XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve Y2 value"), StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value")));
          "inside Y2 value"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858xyz XC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve Y2 value"), StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value")));
          "after Y2 value"
      )]
    // XC entry
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 xyzXC=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve XC key"), StrContext::Expected(StrContextValue::Description("invalid XC key name")));
          "before XC key"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XCxyz=0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve XC key"), StrContext::Expected(StrContextValue::Description("invalid XC key name")));
          "after XC key"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=xyz0.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve XC value"), StrContext::Expected(StrContextValue::Description("invalid XC attribute value")));
          "before XC value"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0xyz.5 YC=0.5 R=1.0)"#,
          (StrContext::Label("curve XC value"), StrContext::Expected(StrContextValue::Description("invalid XC attribute value")));
          "inside XC value"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5xyz YC=0.5 R=1.0)"#,
          (StrContext::Label("curve XC value"), StrContext::Expected(StrContextValue::Description("invalid XC attribute value")));
          "after XC value"
      )]
    // YC entry
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 xyzYC=0.5 R=1.0)"#,
          (StrContext::Label("curve YC key"), StrContext::Expected(StrContextValue::Description("invalid YC key name")));
          "before YC key"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YCxyz=0.5 R=1.0)"#,
          (StrContext::Label("curve YC key"), StrContext::Expected(StrContextValue::Description("invalid YC key name")));
          "after YC key"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=xyz0.5 R=1.0)"#,
          (StrContext::Label("curve YC value"), StrContext::Expected(StrContextValue::Description("invalid YC attribute value")));
          "before YC value"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0xyz.5 R=1.0)"#,
          (StrContext::Label("curve YC value"), StrContext::Expected(StrContextValue::Description("invalid YC attribute value")));
          "inside YC value"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5xyz R=1.0)"#,
          (StrContext::Label("curve YC value"), StrContext::Expected(StrContextValue::Description("invalid YC attribute value")));
          "after YC value"
      )]
    // R entry
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 xyzR=1.0)"#,
          (StrContext::Label("curve R key"), StrContext::Expected(StrContextValue::Description("invalid R key name")));
          "before R key"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 Rxyz=1.0)"#,
          (StrContext::Label("curve R key"), StrContext::Expected(StrContextValue::Description("invalid R key name")));
          "after R key"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=xyz1.0)"#,
          (StrContext::Label("curve R value"), StrContext::Expected(StrContextValue::Description("invalid R attribute value")));
          "before R value"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1xyz.0)"#,
          (StrContext::Label("curve R value"), StrContext::Expected(StrContextValue::Description("invalid R attribute value")));
          "inside R value"
      )]
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0xyz)"#,
          (StrContext::Label("curve R value"), StrContext::Expected(StrContextValue::Description("invalid R attribute value")));
          "after R value"
      )]
    // closing row
    #[test_case(
          r#"(CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 xyz)"#,
          (
              StrContext::Label("closing row"),
              StrContext::Expected(StrContextValue::Description(")"))
          ); "after R value"
      )]
    fn rejects_malformed_curve_row(mut input: &str, expected_errors: (StrContext, StrContext)) {
        let err = parse_curve_row(&mut input).unwrap_err();

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
