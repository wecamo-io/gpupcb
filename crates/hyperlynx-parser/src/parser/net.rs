// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::{ModalResult, Parser};

use crate::model::{Pin, Segment};
use crate::parser::utils::{ExtractEntryErrorMessages, close_row, extract_float_entry, extract_quoted_string_entry, extract_unsigned_integer_entry, row_open};

fn parse_pin_row(input: &mut &str) -> ModalResult<Pin> {
    row_open(input, "PIN")?;

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

    let padstack_id = extract_unsigned_integer_entry(
        input,
        "P",
        ')',
        ExtractEntryErrorMessages {
            key_error_label: "pin P key",
            key_error_description: "invalid P key name",
            value_error_label: "pin P value",
            value_error_description: "invalid P attribute value",
        },
    )?;

    close_row(input)?;

    Ok(Pin {
        x,
        y,
        component_reference: String::from(component_reference),
        padstack_id,
    })
}

fn parse_seg_row(input: &mut &str) -> ModalResult<Segment> {
    row_open(input, "SEG")?;

    let x1 = extract_float_entry(
        input,
        "X1",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "perimeter segment X1 key",
            key_error_description: "invalid X1 key name",
            value_error_label: "perimeter segment X1 value",
            value_error_description: "invalid X1 attribute value",
        },
    )?;

    let y1 = extract_float_entry(
        input,
        "Y1",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "perimeter segment Y1 key",
            key_error_description: "invalid Y1 key name",
            value_error_label: "perimeter segment Y1 value",
            value_error_description: "invalid Y1 attribute value",
        },
    )?;

    let x2 = extract_float_entry(
        input,
        "X2",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "perimeter segment X2 key",
            key_error_description: "invalid X2 key name",
            value_error_label: "perimeter segment X2 value",
            value_error_description: "invalid X2 attribute value",
        },
    )?;

    let y2 = extract_float_entry(
        input,
        "Y2",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "perimeter segment Y2 key",
            key_error_description: "invalid Y2 key name",
            value_error_label: "perimeter segment Y2 value",
            value_error_description: "invalid Y2 attribute value",
        },
    )?;

    let width = extract_float_entry(
        input,
        "W",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "perimeter segment Y2 key",
            key_error_description: "invalid Y2 key name",
            value_error_label: "perimeter segment Y2 value",
            value_error_description: "invalid Y2 attribute value",
        },
    )?;

    let layer = extract_quoted_string_entry(
        input,
        "L",
        ExtractEntryErrorMessages {
            key_error_label: "perimeter segment L key",
            key_error_description: "invalid L key name",
            value_error_label: "perimeter segment L value",
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
mod tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};

    #[test_case(
        r#"(PIN X=8.2480314961 Y=-4.8622047244 R="R3.1" P=0)"#,
        Pin {
            x: 8.2480314961,
            y: -4.8622047244,
            component_reference: String::from("R3.1"),
            padstack_id: 0,
        }
    )]
    #[test_case(
        r#"(PIN X=8.3572834646 Y=-4.5978346457 R="U2.2" P=12)"#,
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
    fn rejects_malformed_input_row(
        mut input: &str,
        expected_errors: (StrContext, StrContext),
    ) {
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
