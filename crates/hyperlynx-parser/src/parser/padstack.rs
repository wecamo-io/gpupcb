// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::{dec_uint, digit1};
use winnow::combinator::{backtrack_err, delimited, repeat};
use winnow::error::{ContextError, ErrMode, StrContext, StrContextValue};
use winnow::token::{literal, take_until};
use winnow::{ModalResult, Parser};

use crate::model::{Pad, PadShape, Padstack};
use crate::parser::utils::{
    close_block, close_row, extract_float_value, extract_unsigned_integer_entry, open_block,
    row_open, ExtractEntryErrorMessages, ExtractValueErrorMessages,
};

fn parse_pad_closing_separator(input: &mut &str, closing: bool) -> ModalResult<()> {
    let (separator, label, desc) = if closing {
        (", M", "pad ', M' separator", "invalid ', M' separator")
    } else {
        (", ", "pad ', ' separator", "invalid ', ' separator")
    };

    backtrack_err(literal(separator))
        .context(StrContext::Label(label))
        .context(StrContext::Expected(StrContextValue::Description(desc)))
        .parse_next(input)?;

    Ok(())
}

fn parse_pad_arguments_separator(input: &mut &str) -> ModalResult<()> {
    parse_pad_closing_separator(input, false)?;

    Ok(())
}

fn parse_pad_row(input: &mut &str) -> ModalResult<Pad> {
    row_open(input, "")?;

    let layer = backtrack_err(delimited('"', take_until(0.., '"'), '"'))
        .context(StrContext::Label("pad layer value"))
        .context(StrContext::Expected(StrContextValue::Description(
            "invalid layer name value",
        )))
        .parse_next(input)?;

    parse_pad_arguments_separator(input)?;

    let shape = {
        let value = backtrack_err(digit1)
            .context(StrContext::Label("pad shape value"))
            .context(StrContext::Expected(StrContextValue::Description(
                "invalid shape value",
            )))
            .parse_next(input)?;

        match value {
            "0" => PadShape::Oval,
            "1" => PadShape::Roundrect,
            "2" => PadShape::Rectangle,
            _ => {
                let mut err = ContextError::new();
                err.push(StrContext::Label("pad shape value"));
                err.push(StrContext::Expected(StrContextValue::Description(
                    "invalid pad shape value, should be 0-2",
                )));

                Err(ErrMode::Backtrack(err))
            }?,
        }
    };

    parse_pad_arguments_separator(input)?;

    let size_x = extract_float_value(
        input,
        ',',
        ExtractValueErrorMessages {
            value_error_label: "pad size_x value",
            value_error_description: "invalid size_x value",
        },
    )?;

    parse_pad_arguments_separator(input)?;

    let size_y = extract_float_value(
        input,
        ',',
        ExtractValueErrorMessages {
            value_error_label: "pad size_y value",
            value_error_description: "invalid size_y value",
        },
    )?;

    parse_pad_arguments_separator(input)?;

    let angle = extract_float_value(
        input,
        ',',
        ExtractValueErrorMessages {
            value_error_label: "pad angle value",
            value_error_description: "invalid angle value",
        },
    )?;

    parse_pad_closing_separator(input, true)?;

    close_row(input)?;

    Ok(Pad {
        layer: String::from(layer),
        shape,
        size_x,
        size_y,
        angle,
    })
}

fn parse_padstack_block(input: &mut &str) -> ModalResult<Padstack> {
    open_block(input, "")?;

    let id = extract_unsigned_integer_entry(
        input,
        "PADSTACK",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "padstack PADSTACK key",
            key_error_description: "invalid PADSTACK key name",
            value_error_label: "padstack ID value",
            value_error_description: "invalid ID attribute value",
        },
    )?;

    parse_pad_arguments_separator(input)?;

    let drill = extract_float_value(
        input,
        ' ',
        ExtractValueErrorMessages {
            value_error_label: "padstack Drill value",
            value_error_description: "invalid Drill attribute value",
        },
    )?;

    backtrack_err(literal("\n  "))
        .context(StrContext::Label("padstack new line"))
        .context(StrContext::Expected(StrContextValue::Description(
            "invalid padstack new line",
        )))
        .parse_next(input)?;

    let pads: Vec<Pad> = repeat(0.., parse_pad_row).parse_next(input)?;

    close_block(input)?;

    Ok(Padstack { id, drill, pads })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0.055118110, 90.0, M)"#,
        Pad {
            layer: String::from("F.Cu"),
            shape: PadShape::Rectangle,
            size_x: 0.047244094,
            size_y: 0.055118110,
            angle: 90.0,
        }
    )]
    #[test_case(
        r#"("B.Cu", 0, 0.047244094, 0.055118110, 180.0, M)"#,
        Pad {
            layer: String::from("B.Cu"),
            shape: PadShape::Oval,
            size_x: 0.047244094,
            size_y: 0.055118110,
            angle: 180.0,
        }
    )]
    #[test_case(
        r#"("F.Cu", 1, 0.047244094, 0.055118110, 270.0, M)"#,
        Pad {
            layer: String::from("F.Cu"),
            shape: PadShape::Roundrect,
            size_x: 0.047244094,
            size_y: 0.055118110,
            angle: 270.0,
        }
    )]
    fn parses_pad_row(mut input: &str, expected: Pad) {
        let pad = parse_pad_row.parse_next(&mut input).unwrap();

        assert_eq!(pad, expected);
    }

    #[test_case(
        r#"xyz("F.Cu", 2, 0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("opening row"),
            StrContext::Expected(StrContextValue::Description("("))
        )
    )]
    // separator malformed between layer and shape
    #[test_case(
        r#"("F.Cu"z, 2, 0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu",z 2, 0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu",2, 0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu" ,2, 0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    // separator malformed between shape and size_x
    #[test_case(
        r#"("F.Cu", 2z, 0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2,z 0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2,0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2 ,0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    // separator malformed between size_x and size_y
    #[test_case(
        r#"("F.Cu", 2, 0.047244094,z 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0.047244094,0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0.047244094 ,0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    // separator malformed between size_y and angle
    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0.055118110,z 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0.055118110,90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0.055118110 ,90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    // separator malformed between angle and M
    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0.055118110, 90.0,z M)"#,
        (
            StrContext::Label("pad ', M' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', M' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0.055118110, 90.0,M)"#,
        (
            StrContext::Label("pad ', M' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', M' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0.055118110, 90.0 ,M)"#,
        (
            StrContext::Label("pad ', M' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', M' separator"))
        )
    )]
    fn rejects_malformed_pad_row_separators_and_parenthesis(
        mut input: &str,
        expected_errors: (StrContext, StrContext),
    ) {
        let err = parse_pad_row(&mut input).unwrap_err();

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

    // layer name value lacks double quotes
    #[test_case(
        r#"(xyz"F.Cu", 2, 0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad layer value"),
            StrContext::Expected(StrContextValue::Description("invalid layer name value"))
        )
    )]
    #[test_case(
        r#"(F.Cu", 2, 0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad layer value"),
            StrContext::Expected(StrContextValue::Description("invalid layer name value"))
        )
    )]
    #[test_case(
        r#"("F.Cu, 2, 0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad layer value"),
            StrContext::Expected(StrContextValue::Description("invalid layer name value"))
        )
    )]
    #[test_case(
        r#"(F.Cu, 2, 0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad layer value"),
            StrContext::Expected(StrContextValue::Description("invalid layer name value"))
        )
    )]
    // shape value malversation
    #[test_case(
        r#"("F.Cu", xyz2, 0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad shape value"),
            StrContext::Expected(StrContextValue::Description("invalid shape value"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 3, 0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad shape value"),
            StrContext::Expected(StrContextValue::Description("invalid pad shape value, should be 0-2"))
        )
    )]
    // size_x value malversation
    #[test_case(
        r#"("F.Cu", 2, xyz0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad size_x value"),
            StrContext::Expected(StrContextValue::Description("invalid size_x value"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0xyz.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad size_x value"),
            StrContext::Expected(StrContextValue::Description("invalid size_x value"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0.xyz047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad size_x value"),
            StrContext::Expected(StrContextValue::Description("invalid size_x value"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0.047244094xyz, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad size_x value"),
            StrContext::Expected(StrContextValue::Description("invalid size_x value"))
        )
    )]
    // size_y value malversation
    #[test_case(
        r#"("F.Cu", 2, 0.047244094, xyz0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad size_y value"),
            StrContext::Expected(StrContextValue::Description("invalid size_y value"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0xyz.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad size_y value"),
            StrContext::Expected(StrContextValue::Description("invalid size_y value"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0.xyz055118110, 90.0, M)"#,
        (
            StrContext::Label("pad size_y value"),
            StrContext::Expected(StrContextValue::Description("invalid size_y value"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0.055118110xyz, 90.0, M)"#,
        (
            StrContext::Label("pad size_y value"),
            StrContext::Expected(StrContextValue::Description("invalid size_y value"))
        )
    )]
    // angle value malversation
    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0.055118110, xyz90.0, M)"#,
        (
            StrContext::Label("pad angle value"),
            StrContext::Expected(StrContextValue::Description("invalid angle value"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0.055118110, 9xyz0.0, M)"#,
        (
            StrContext::Label("pad angle value"),
            StrContext::Expected(StrContextValue::Description("invalid angle value"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0.055118110, 90xyz.0, M)"#,
        (
            StrContext::Label("pad angle value"),
            StrContext::Expected(StrContextValue::Description("invalid angle value"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0.055118110, 90.xyz0, M)"#,
        (
            StrContext::Label("pad angle value"),
            StrContext::Expected(StrContextValue::Description("invalid angle value"))
        )
    )]
    #[test_case(
        r#"("F.Cu", 2, 0.047244094, 0.055118110, 90.0xyz, M)"#,
        (
            StrContext::Label("pad angle value"),
            StrContext::Expected(StrContextValue::Description("invalid angle value"))
        )
    )]
    fn rejects_malformed_pad_row_arguments(
        mut input: &str,
        expected_errors: (StrContext, StrContext),
    ) {
        let err = parse_pad_row(&mut input).unwrap_err();

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

    #[test]
    fn parses_padstack_block() {
        let mut input =
            "{PADSTACK=15, 0.007874016\n  (\"MDEF\", 0, 0.023622047, 0.023622047, 0.0, M)\n}";
        let expected = Padstack {
            id: 15,
            drill: 0.007874016,
            pads: vec![Pad {
                layer: String::from("MDEF"),
                shape: PadShape::Oval,
                size_x: 0.023622047,
                size_y: 0.023622047,
                angle: 0.0,
            }],
        };

        let padstack = parse_padstack_block.parse_next(&mut input).unwrap();

        assert_eq!(padstack, expected);
    }

    // opening brace malversations
    #[test_case(
        "xyz{PADSTACK=15, 0.007874016\n  (\"MDEF\", 0, 0.023622047, 0.023622047, 0.0, M)\n}",
        (
            StrContext::Label("opening block"),
            StrContext::Expected(StrContextValue::Description("{"))
        )
    )]
    // PADSTACK entry malversations
    #[test_case(
        "{xyzPADSTACK=15, 0.007874016\n  (\"MDEF\", 0, 0.023622047, 0.023622047, 0.0, M)\n}",
        (
            StrContext::Label("padstack PADSTACK key"),
            StrContext::Expected(StrContextValue::Description("invalid PADSTACK key name"))
        )
    )]
    #[test_case(
        "{PADSTACK=xyz15, 0.007874016\n  (\"MDEF\", 0, 0.023622047, 0.023622047, 0.0, M)\n}",
        (
            StrContext::Label("padstack ID value"),
            StrContext::Expected(StrContextValue::Description("invalid ID attribute value"))
        )
    )]
    // ', ' separator malversation
    #[test_case(
        "{PADSTACK=15xyz, 0.007874016\n  (\"MDEF\", 0, 0.023622047, 0.023622047, 0.0, M)\n}",
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        "{PADSTACK=15,xyz 0.007874016\n  (\"MDEF\", 0, 0.023622047, 0.023622047, 0.0, M)\n}",
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    // Drill value malversation
    #[test_case(
        "{PADSTACK=15, xyz0.007874016\n  (\"MDEF\", 0, 0.023622047, 0.023622047, 0.0, M)\n}",
        (
            StrContext::Label("padstack Drill value"),
            StrContext::Expected(StrContextValue::Description("invalid Drill attribute value"))
        )
    )]
    #[test_case(
        "{PADSTACK=15, 0xyz.007874016\n  (\"MDEF\", 0, 0.023622047, 0.023622047, 0.0, M)\n}",
        (
            StrContext::Label("padstack Drill value"),
            StrContext::Expected(StrContextValue::Description("invalid Drill attribute value"))
        )
    )]
    #[test_case(
        "{PADSTACK=15, 0.xyz007874016\n  (\"MDEF\", 0, 0.023622047, 0.023622047, 0.0, M)\n}",
        (
            StrContext::Label("padstack Drill value"),
            StrContext::Expected(StrContextValue::Description("invalid Drill attribute value"))
        )
    )]
    #[test_case(
        "{PADSTACK=15, 0.007874016xyz\n  (\"MDEF\", 0, 0.023622047, 0.023622047, 0.0, M)\n}",
        (
            StrContext::Label("padstack Drill value"),
            StrContext::Expected(StrContextValue::Description("invalid Drill attribute value"))
        )
    )]
    // closing brace malversations
    #[test_case(
        "{PADSTACK=15, 0.007874016\n  (\"MDEF\", 0, 0.023622047, 0.023622047, 0.0, M)\nxyz}",
        (
            StrContext::Label("closing block"),
            StrContext::Expected(StrContextValue::Description("}"))
        )
    )]
    fn rejects_malformed_padstack_block(
        mut input: &str,
        expected_errors: (StrContext, StrContext),
    ) {
        let mut input = input;
        let err = parse_padstack_block.parse_next(&mut input).unwrap_err();

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

    #[test]
    fn leaves_further_input_after_block_parsing_finish() {
        let mut input ="{PADSTACK=15, 0.007874016\n  (\"MDEF\", 0, 0.023622047, 0.023622047, 0.0, M)\n}further input";
        let expected = "further input";

        let _padstack = parse_padstack_block(&mut input).unwrap();

        assert_eq!(input, expected);
    }
}
