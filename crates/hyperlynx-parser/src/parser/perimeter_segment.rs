// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::multispace0;
use winnow::combinator::{alt, cut_err, peek, repeat};
use winnow::prelude::*;
use winnow::token::literal;

use crate::model::PerimeterType;
use crate::parser::utils::{
    ExtractEntryErrorMessages, ExtractValueErrorMessages, build_parser_cut_error, close_block,
    close_row, extract_float_entry, extract_string_value, open_block, single_space, skip_ignored,
};
use crate::{model::Perimeter, parser::utils::row_open};

fn perimeter_segment(input: &mut &str) -> ModalResult<Perimeter> {
    match row_open(input, "PERIMETER_SEGMENT") {
        Ok(_) => {}
        Err(_) => {
            return Err(build_parser_cut_error((
                "perimeter row name",
                "perimeter_segment/perimeter_arc key name",
            )));
        }
    };

    single_space(input)?;

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

    single_space(input)?;

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

    single_space(input)?;

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

    single_space(input)?;

    let y2 = extract_float_entry(
        input,
        "Y2",
        ')',
        ExtractEntryErrorMessages {
            key_error_label: "perimeter segment Y2 key",
            key_error_description: "invalid Y2 key name",
            value_error_label: "perimeter segment Y2 value",
            value_error_description: "invalid Y2 attribute value",
        },
    )?;

    close_row(input)?;

    skip_ignored(input)?;

    Ok(Perimeter {
        x1,
        y1,
        x2,
        y2,
        r#type: PerimeterType::Segment,
    })
}

fn perimeter_arc(input: &mut &str) -> ModalResult<Perimeter> {
    match row_open(input, "PERIMETER_ARC") {
        Ok(_) => {}
        Err(_) => {
            return Err(build_parser_cut_error((
                "perimeter row name",
                "perimeter_segment/perimeter_arc key name",
            )));
        }
    };

    single_space(input)?;

    let x1 = extract_float_entry(
        input,
        "X1",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "perimeter arc X1 key",
            key_error_description: "invalid X1 key name",
            value_error_label: "perimeter arc X1 value",
            value_error_description: "invalid X1 attribute value",
        },
    )?;

    single_space(input)?;

    let y1 = extract_float_entry(
        input,
        "Y1",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "perimeter arc Y1 key",
            key_error_description: "invalid Y1 key name",
            value_error_label: "perimeter arc Y1 value",
            value_error_description: "invalid Y1 attribute value",
        },
    )?;

    single_space(input)?;

    let x2 = extract_float_entry(
        input,
        "X2",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "perimeter arc X2 key",
            key_error_description: "invalid X2 key name",
            value_error_label: "perimeter arc X2 value",
            value_error_description: "invalid X2 attribute value",
        },
    )?;

    single_space(input)?;

    let y2 = extract_float_entry(
        input,
        "Y2",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "perimeter arc Y2 key",
            key_error_description: "invalid Y2 key name",
            value_error_label: "perimeter arc Y2 value",
            value_error_description: "invalid Y2 attribute value",
        },
    )?;

    single_space(input)?;

    let xc = extract_float_entry(
        input,
        "XC",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "perimeter arc XC key",
            key_error_description: "invalid XC key name",
            value_error_label: "perimeter arc XC value",
            value_error_description: "invalid XC attribute value",
        },
    )?;

    single_space(input)?;

    let yc = extract_float_entry(
        input,
        "YC",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "perimeter arc YC key",
            key_error_description: "invalid YC key name",
            value_error_label: "perimeter arc YC value",
            value_error_description: "invalid YC attribute value",
        },
    )?;

    single_space(input)?;

    let radius = extract_float_entry(
        input,
        "R",
        ')',
        ExtractEntryErrorMessages {
            key_error_label: "perimeter arc R key",
            key_error_description: "invalid R key name",
            value_error_label: "perimeter arc R value",
            value_error_description: "invalid R attribute value",
        },
    )?;

    close_row(input)?;

    skip_ignored(input)?;

    Ok(Perimeter {
        x1,
        y1,
        x2,
        y2,
        r#type: PerimeterType::Arc { xc, yc, radius },
    })
}

fn parse_perimeter(input: &mut &str) -> ModalResult<Perimeter> {
    let is_segment: ModalResult<&str> = peek(literal("(PERIMETER_SEGMENT")).parse_next(input);

    if is_segment.is_ok() {
        perimeter_segment.parse_next(input)
    } else {
        let is_arc: ModalResult<&str> = peek(literal("(PERIMETER_ARC")).parse_next(input);

        if is_arc.is_ok() {
            perimeter_arc.parse_next(input)
        } else {
            alt((perimeter_segment, perimeter_arc)).parse_next(input)
        }
    }
}

fn parse_perimeter_row_with_err(input: &mut &str) -> ModalResult<Perimeter> {
    peek((multispace0, "(")).parse_next(input)?;

    cut_err(parse_perimeter).parse_next(input)
}

pub fn parse_board_block(input: &mut &str) -> ModalResult<Vec<Perimeter>> {
    open_block(input, "BOARD")?;

    single_space(input)?;
    let _path = extract_string_value(
        input,
        '\r',
        ExtractValueErrorMessages {
            value_error_label: "board path value",
            value_error_description: "invalid path attribute value",
        },
    )?;

    skip_ignored(input)?;

    let perimeter_segments: Vec<Perimeter> =
        repeat(0.., parse_perimeter_row_with_err).parse_next(input)?;

    close_block(input)?;

    Ok(perimeter_segments)
}

#[cfg(test)]
mod board_block_parser_tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};

    #[test_case(
        "(PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  (...)",
        Perimeter {
            x1: 8.415354331,
            y1: -5.334321614,
            x2: 8.251181102,
            y2: -5.334321614,
            r#type: PerimeterType::Segment
        }
    )]
    fn parses_single_perimeter_segment(mut input: &str, expected: Perimeter) {
        let perimeter_segment = perimeter_segment(&mut input).unwrap();

        assert_eq!(perimeter_segment, expected);
    }

    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n  (...)",
        Perimeter {
            x1: 8.415354331,
            y1: -5.334321614,
            x2: 8.251181102,
            y2: -5.334321614,
            r#type: PerimeterType::Arc {
                xc: 5.334321614,
                yc: 8.251181102,
                radius: 0.5
            }
        }
    )]
    fn parses_single_perimeter_arc(mut input: &str, expected: Perimeter) {
        let perimeter_segment = perimeter_arc(&mut input).unwrap();

        assert_eq!(perimeter_segment, expected);
    }

    // X1 malformed cases
    #[test_case(
        "(PERIMETER_SEGMENT X1=xyz8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  ",
        StrContext::Label("perimeter segment X1 value"),
        StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.4153xyz54331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  ",
        StrContext::Label("perimeter segment X1 value"),
        StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.415354331xyz Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  ",
        StrContext::Label("perimeter segment X1 value"),
        StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
    )]
    // Y1 malformed cases
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.415354331 Y1=xyz-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  ",
        StrContext::Label("perimeter segment Y1 value"),
        StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.415354331 Y1=-5.3343xyz21614 X2=8.251181102 Y2=-5.334321614)\n  ",
        StrContext::Label("perimeter segment Y1 value"),
        StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614xyz X2=8.251181102 Y2=-5.334321614)\n  ",
        StrContext::Label("perimeter segment Y1 value"),
        StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
    )]
    // X2 malformed cases
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=xyz8.251181102 Y2=-5.334321614)\n  ",
        StrContext::Label("perimeter segment X2 value"),
        StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.2511xyz81102 Y2=-5.334321614)\n  ",
        StrContext::Label("perimeter segment X2 value"),
        StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102xyz Y2=-5.334321614)\n  ",
        StrContext::Label("perimeter segment X2 value"),
        StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
    )]
    // Y2 malformed cases
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=xyz-5.334321614)\n  ",
        StrContext::Label("perimeter segment Y2 value"),
        StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.3343xyz21614)\n  ",
        StrContext::Label("perimeter segment Y2 value"),
        StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614xyz)\n  ",
        StrContext::Label("perimeter segment Y2 value"),
        StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
    )]
    fn rejects_malformed_perimeter_segment(
        mut input: &str,
        expected_label: StrContext,
        expected_description: StrContext,
    ) {
        let err = perimeter_segment(&mut input).unwrap_err();

        let errors = match err {
            ErrMode::Backtrack(ref err) | ErrMode::Cut(ref err) => {
                let mut iter = err.context();

                (iter.next().unwrap(), iter.next().unwrap())
            }
            ErrMode::Incomplete(_) => todo!(),
        };

        assert_eq!(*errors.0, expected_label);
        assert_eq!(*errors.1, expected_description);
    }

    // row name
    #[test_case(
        "(xyzPERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n  (...)",
        StrContext::Label("perimeter row name"),
        StrContext::Expected(StrContextValue::Description(
            "perimeter_segment/perimeter_arc key name"
        ))
    )]
    #[test_case(
        "(PERIMETxyzER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n  (...)",
        StrContext::Label("perimeter row name"),
        StrContext::Expected(StrContextValue::Description(
            "perimeter_segment/perimeter_arc key name"
        ))
    )]
    #[test_case(
        "(PERIMETER_ARCxyz X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n  (...)",
        StrContext::Label("perimeter row name"),
        StrContext::Expected(StrContextValue::Description(
            "perimeter_segment/perimeter_arc key name"
        ))
    )]
    // X1 malformed cases
    #[test_case(
        "(PERIMETER_ARC X1=xyz8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc X1 value"),
        StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_ARC X1=8.4153xyz54331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc X1 value"),
        StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331xyz Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc X1 value"),
        StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
    )]
    // Y1 malformed cases
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=xyz-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc Y1 value"),
        StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.3343xyz21614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc Y1 value"),
        StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614xyz X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc Y1 value"),
        StrContext::Expected(StrContextValue::Description("invalid Y1 attribute value"))
    )]
    // X2 malformed cases
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=xyz8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc X2 value"),
        StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.2511xyz81102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc X2 value"),
        StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102xyz Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc X2 value"),
        StrContext::Expected(StrContextValue::Description("invalid X2 attribute value"))
    )]
    // Y2 malformed cases
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=xyz-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc Y2 value"),
        StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.3343xyz21614 XC=5.334321614 YC=8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc Y2 value"),
        StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614xyz XC=5.334321614 YC=8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc Y2 value"),
        StrContext::Expected(StrContextValue::Description("invalid Y2 attribute value"))
    )]
    // XC malformed cases
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=xyz5.334321614 YC=8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc XC value"),
        StrContext::Expected(StrContextValue::Description("invalid XC attribute value"))
    )]
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.3343xyz21614 YC=8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc XC value"),
        StrContext::Expected(StrContextValue::Description("invalid XC attribute value"))
    )]
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614xyz YC=8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc XC value"),
        StrContext::Expected(StrContextValue::Description("invalid XC attribute value"))
    )]
    // YC malformed cases
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=xyz8.251181102 R=0.5)\n  ",
        StrContext::Label("perimeter arc YC value"),
        StrContext::Expected(StrContextValue::Description("invalid YC attribute value"))
    )]
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.2511xyz81102 R=0.5)\n  ",
        StrContext::Label("perimeter arc YC value"),
        StrContext::Expected(StrContextValue::Description("invalid YC attribute value"))
    )]
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102xyz R=0.5)\n  ",
        StrContext::Label("perimeter arc YC value"),
        StrContext::Expected(StrContextValue::Description("invalid YC attribute value"))
    )]
    // R malformed cases
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=xyz0.5)\n  ",
        StrContext::Label("perimeter arc R value"),
        StrContext::Expected(StrContextValue::Description("invalid R attribute value"))
    )]
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.xyz5)\n  ",
        StrContext::Label("perimeter arc R value"),
        StrContext::Expected(StrContextValue::Description("invalid R attribute value"))
    )]
    #[test_case(
        "(PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5xyz)\n  ",
        StrContext::Label("perimeter arc R value"),
        StrContext::Expected(StrContextValue::Description("invalid R attribute value"))
    )]
    fn rejects_malformed_perimeter_arc(
        mut input: &str,
        expected_label: StrContext,
        expected_description: StrContext,
    ) {
        let err = perimeter_arc(&mut input).unwrap_err();

        let errors = match err {
            ErrMode::Backtrack(ref err) | ErrMode::Cut(ref err) => {
                let mut iter = err.context();

                (iter.next().unwrap(), iter.next().unwrap())
            }
            ErrMode::Incomplete(_) => todo!(),
        };

        assert_eq!(*errors.0, expected_label);
        assert_eq!(*errors.1, expected_description);
    }

    #[test_case(
        "{BOARD \"/path/to/file/example.kicad_pcb\"\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  (PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n}",
        vec![
            Perimeter { x1: 8.415354331, y1: -5.334321614, x2: 8.251181102, y2: -5.334321614, r#type: PerimeterType::Segment },
            Perimeter { x1: 8.415354331,  y1: -5.334321614, x2: 8.251181102, y2: -5.334321614, r#type: PerimeterType::Arc { xc: 5.334321614, yc: 8.251181102, radius: 0.5 } }
        ]; "path double quotes present"
    )]
    #[test_case(
        "{BOARD /path/to/file/another example.kicad_pcb\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  (PERIMETER_ARC X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614 XC=5.334321614 YC=8.251181102 R=0.5)\n}",
        vec![
            Perimeter { x1: 8.415354331, y1: -5.334321614, x2: 8.251181102, y2: -5.334321614, r#type: PerimeterType::Segment },
            Perimeter { x1: 8.415354331,  y1: -5.334321614, x2: 8.251181102, y2: -5.334321614, r#type: PerimeterType::Arc { xc: 5.334321614, yc: 8.251181102, radius: 0.5 } }
        ]; "path double quotes present"
    )]
    fn parses_board_block(mut input: &str, expected: Vec<Perimeter>) {
        let perimeter_segments = parse_board_block.parse_next(&mut input).unwrap();

        assert_eq!(perimeter_segments, expected);
    }

    // opening block
    #[test_case(
        "xyz{BOARD \"/path/to/file/example.kicad_pcb\"\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.334321614 X2=8.251181102 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.846132638 X2=7.778740157 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.846132638 X2=7.778740157 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.334321614 X2=7.608267717 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-5.334321614 X2=7.608267717 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-4.487864921 X2=8.415354331 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-4.487864921 X2=8.415354331 Y2=-5.334321614)\n}",
        (
            StrContext::Label("opening block"),
            StrContext::Expected(StrContextValue::Description("{"))
        )
    )]
    // block name
    #[test_case(
        "{xyzBOARD \"/path/to/file/example.kicad_pcb\"\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.334321614 X2=8.251181102 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.846132638 X2=7.778740157 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.846132638 X2=7.778740157 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.334321614 X2=7.608267717 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-5.334321614 X2=7.608267717 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-4.487864921 X2=8.415354331 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-4.487864921 X2=8.415354331 Y2=-5.334321614)\n}",
        (
            StrContext::Label("block name"),
            StrContext::Expected(StrContextValue::Description("BOARD"))
        )
    )]
    #[test_case(
        "{BOARDxyz \"/path/to/file/example.kicad_pcb\"\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.334321614 X2=8.251181102 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.846132638 X2=7.778740157 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.846132638 X2=7.778740157 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.334321614 X2=7.608267717 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-5.334321614 X2=7.608267717 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-4.487864921 X2=8.415354331 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-4.487864921 X2=8.415354331 Y2=-5.334321614)\n}",
        (
            StrContext::Label("block name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after block name"))
        )
    )]
    // closing block
    #[test_case(
        "{BOARD \"/path/to/file/example.kicad_pcb\"\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.334321614 X2=8.251181102 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.846132638 X2=7.778740157 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.846132638 X2=7.778740157 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.334321614 X2=7.608267717 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-5.334321614 X2=7.608267717 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-4.487864921 X2=8.415354331 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-4.487864921 X2=8.415354331 Y2=-5.334321614)\nxyz}",
        (
            StrContext::Label("closing block"),
            StrContext::Expected(StrContextValue::Description("}"))
        )
    )]
    // row malversations
    #[test_case(
    "{BOARD \"/path/to/file/example.kicad_pcb\"\n  (xyzPERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.334321614 X2=8.251181102 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.846132638 X2=7.778740157 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.846132638 X2=7.778740157 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.334321614 X2=7.608267717 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-5.334321614 X2=7.608267717 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-4.487864921 X2=8.415354331 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-4.487864921 X2=8.415354331 Y2=-5.334321614)\n}",
        (
            StrContext::Label("perimeter row name"),
            StrContext::Expected(StrContextValue::Description("perimeter_segment/perimeter_arc key name"))
        )
    )]
    #[test_case(
    "{BOARD \"/path/to/file/example.kicad_pcb\"\n  (PERIMETER_SEGMENTxyz X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.334321614 X2=8.251181102 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.846132638 X2=7.778740157 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.846132638 X2=7.778740157 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.334321614 X2=7.608267717 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-5.334321614 X2=7.608267717 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-4.487864921 X2=8.415354331 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-4.487864921 X2=8.415354331 Y2=-5.334321614)\n}",
        (
            StrContext::Label("perimeter row name"),
            StrContext::Expected(StrContextValue::Description("perimeter_segment/perimeter_arc key name"))
        )
    )]
    #[test_case(
    "{BOARD \"/path/to/file/example.kicad_pcb\"\n  (PERIMETER_SEGMENT xyzX1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.334321614 X2=8.251181102 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.846132638 X2=7.778740157 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.846132638 X2=7.778740157 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.334321614 X2=7.608267717 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-5.334321614 X2=7.608267717 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-4.487864921 X2=8.415354331 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-4.487864921 X2=8.415354331 Y2=-5.334321614)\n}",
        (
            StrContext::Label("perimeter segment X1 key"),
            StrContext::Expected(StrContextValue::Description("invalid X1 key name"))
        )
    )]
    fn rejects_malformed_board_block(mut input: &str, expected_errors: (StrContext, StrContext)) {
        let err = parse_board_block(&mut input).unwrap_err();

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
