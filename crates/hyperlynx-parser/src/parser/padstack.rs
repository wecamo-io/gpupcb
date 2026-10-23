// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::digit1;
use winnow::combinator::{backtrack_err, delimited};
use winnow::error::{StrContext, StrContextValue};
use winnow::token::{literal, take_until};
use winnow::{ModalResult, Parser};

use crate::model::{Pad, PadShape};
use crate::parser::utils::{close_row, extract_float_value, row_open, ExtractValueErrorMessages};

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
        let value = digit1.parse_next(input)?;

        match value {
            "0" => PadShape::Oval,
            "1" => PadShape::Roundrect,
            "2" => PadShape::Rectangle,
            _ => todo!(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::ErrMode;

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
    // // layer name value lacks double quotes
    // #[test_case(
    //     r#"(xyz"F.Cu", 2, 0.047244094, 0.055118110, 90.0, M)"#,
    //     (
    //         StrContext::Label("pad layer value"),
    //         StrContext::Expected(StrContextValue::Description("invalid layer name value"))
    //     )
    // )]
    // #[test_case(
    //     r#"(F.Cu", 2, 0.047244094, 0.055118110, 90.0, M)"#,
    //     (
    //         StrContext::Label("pad layer value"),
    //         StrContext::Expected(StrContextValue::Description("invalid layer name value"))
    //     )
    // )]
    // #[test_case(
    //     r#"("F.Cu, 2, 0.047244094, 0.055118110, 90.0, M)"#,
    //     (
    //         StrContext::Label("pad layer value"),
    //         StrContext::Expected(StrContextValue::Description("invalid layer name value"))
    //     )
    // )]
    // #[test_case(
    //     r#"(F.Cu, 2, 0.047244094, 0.055118110, 90.0, M)"#,
    //     (
    //         StrContext::Label("pad layer value"),
    //         StrContext::Expected(StrContextValue::Description("invalid layer name value"))
    //     )
    // )]
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
        r#"("F.Cu",2z, 0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu",2,z 0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu",2,0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu",2 ,0.047244094, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    // separator malformed between size_x and size_y
    #[test_case(
        r#"("F.Cu",2, 0.047244094z, 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu",2, 0.047244094,z 0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu",2, 0.047244094,0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu",2, 0.047244094 ,0.055118110, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu",2, 0.047244094, 0.055118110z, 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    // separator malformed between size_y and angle
    #[test_case(
        r#"("F.Cu",2, 0.047244094, 0.055118110,z 90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu",2, 0.047244094, 0.055118110,90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu",2, 0.047244094, 0.055118110 ,90.0, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    // separator malformed between angle and M
    #[test_case(
        r#"("F.Cu",2, 0.047244094, 0.055118110, 90.0z, M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu",2, 0.047244094, 0.055118110, 90.0,z M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu",2, 0.047244094, 0.055118110, 90.0,M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    #[test_case(
        r#"("F.Cu",2, 0.047244094, 0.055118110, 90.0 ,M)"#,
        (
            StrContext::Label("pad ', ' separator"),
            StrContext::Expected(StrContextValue::Description("invalid ', ' separator"))
        )
    )]
    fn rejects_malformed_pad_row_separators_and_parenthesis(
        mut input: &str,
        expected_errors: (StrContext, StrContext)
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
}
