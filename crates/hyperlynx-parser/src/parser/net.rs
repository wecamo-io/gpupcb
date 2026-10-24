// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::{ModalResult, Parser};

use crate::model::{Pin, Segment, Via};
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

fn parse_via_row(input: &mut &str) -> ModalResult<Via> {
    row_open(input, "VIA")?;

    let x = extract_float_entry(
        input,
        "X",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "via X key",
            key_error_description: "invalid X key name",
            value_error_label: "via X value",
            value_error_description: "invalid X attribute value",
        }
    )?;

    let y = extract_float_entry(
        input,
        "Y",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "via Y key",
            key_error_description: "invalid Y key name",
            value_error_label: "via Y value",
            value_error_description: "invalid Y attribute value",
        }
    )?;

    let padstack_id = extract_unsigned_integer_entry(
        input,
        "P",
        ')',
        ExtractEntryErrorMessages {
            key_error_label: "via P key",
            key_error_description: "invalid P key name",
            value_error_label: "via P value",
            value_error_description: "invalid P attribute value",
        }
    )?;

    Ok(Via {
        x,
        y,
        padstack_id
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

    #[test_case(
        r#"(VIA X=8.1917322835 Y=-5.0476377953 P=18)"#,
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
    fn rejects_malformed_pin_row(
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
    fn rejects_malformed_via_row(
        mut input: &str,
        expected_errors: (StrContext, StrContext)
    ) {
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
