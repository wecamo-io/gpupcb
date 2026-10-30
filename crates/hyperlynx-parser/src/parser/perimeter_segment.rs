// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::multispace0;
use winnow::combinator::{alt, cut_err, peek, repeat};
use winnow::error::{ContextError, ErrMode, StrContext, StrContextValue};
use winnow::prelude::*;

use crate::parser::utils::{
    ExtractEntryErrorMessages, ExtractValueErrorMessages, close_block, close_row,
    extract_float_entry, extract_quoted_string_value, extract_unquoted_string_value,
    line_ending_with_two_spaces, open_block, single_line_ending, single_space,
};
use crate::{model::PerimeterSegment, parser::utils::row_open};

fn parse_perimeter_segment_row(input: &mut &str) -> ModalResult<PerimeterSegment> {
    row_open(input, "PERIMETER_SEGMENT")?;

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

    alt((single_line_ending, line_ending_with_two_spaces))
        .map_err(|_| {
            let mut err = ContextError::new();
            err.push(StrContext::Label("line ending"));
            err.push(StrContext::Expected(StrContextValue::Description(
                "expected single line ending or new line with 2 spaces after",
            )));
            return winnow::error::ErrMode::Cut(err);
        })
        .parse_next(input)?;

    Ok(PerimeterSegment { x1, y1, x2, y2 })
}

fn parse_perimeter_segment_row_with_err(input: &mut &str) -> ModalResult<PerimeterSegment> {
    peek((multispace0, "(")).parse_next(input)?;

    cut_err(parse_perimeter_segment_row).parse_next(input)
}

pub fn parse_board_block(input: &mut &str) -> ModalResult<Vec<PerimeterSegment>> {
    open_block(input, "BOARD")?;

    single_space.parse_next(input)?;

    alt((
        |input: &mut &str| {
            extract_quoted_string_value(
                input,
                ExtractValueErrorMessages {
                    value_error_label: "",
                    value_error_description: "",
                },
            )
        },
        |input: &mut &str| {
            extract_unquoted_string_value(
                input,
                '\n',
                ExtractValueErrorMessages {
                    value_error_label: "",
                    value_error_description: "",
                },
            )
        },
    ))
    .parse_next(input)?;

    // due to possible lack of PERIMETER_SEGMENT rows inside BOARD block
    alt((line_ending_with_two_spaces, single_line_ending))
        .map_err(|_| {
            let mut err = ContextError::new();
            err.push(StrContext::Label("line ending"));
            err.push(StrContext::Expected(StrContextValue::Description(
                "expected single line ending or new line with 2 spaces after",
            )));
            return ErrMode::Cut(err);
        })
        .parse_next(input)?;

    let perimeter_segments: Vec<PerimeterSegment> =
        repeat(0.., parse_perimeter_segment_row_with_err).parse_next(input)?;

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
        PerimeterSegment {
            x1: 8.415354331,
            y1: -5.334321614,
            x2: 8.251181102,
            y2: -5.334321614,
        }
    )]
    fn parses_single_perimeter_segment(mut input: &str, expected: PerimeterSegment) {
        let perimeter_segment = parse_perimeter_segment_row(&mut input).unwrap();

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
    // line ending
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n (...)",
        StrContext::Label("line ending"),
        StrContext::Expected(StrContextValue::Description(
            "expected single line ending or new line with 2 spaces after"
        )); "one space after line ending"
    )]
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)(...)",
        StrContext::Label("line ending"),
        StrContext::Expected(StrContextValue::Description(
            "expected single line ending or new line with 2 spaces after"
        )); "no line ending"
    )]
    fn rejects_malformed_perimeter_row(
        input: &str,
        expected_label: StrContext,
        expected_description: StrContext,
    ) {
        let mut input = input;
        let err = parse_perimeter_segment_row(&mut input).unwrap_err();

        let errors = match err {
            ErrMode::Backtrack(ref err) | ErrMode::Cut(ref err) => {
                let mut iter = err.context().into_iter();
                let errors = (iter.next().unwrap(), iter.next().unwrap());

                errors
            }
            ErrMode::Incomplete(_) => todo!(),
        };

        assert_eq!(*errors.0, expected_label);
        assert_eq!(*errors.1, expected_description);
    }

    #[test_case(
        "{BOARD \"/path/to/file/example.kicad_pcb\"\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.334321614 X2=8.251181102 Y2=-5.846132638)\n}",
        vec![
            PerimeterSegment { x1: 8.415354331, y1: -5.334321614, x2: 8.251181102, y2: -5.334321614 },
            PerimeterSegment { x1: 8.251181102, x2: 8.251181102, y1: -5.334321614, y2: -5.846132638 }
        ]; "path double quotes present"
    )]
    #[test_case(
        "{BOARD /path/to/file/another example.kicad_pcb\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.334321614 X2=8.251181102 Y2=-5.846132638)\n}",
        vec![
            PerimeterSegment { x1: 8.415354331, y1: -5.334321614, x2: 8.251181102, y2: -5.334321614 },
            PerimeterSegment { x1: 8.251181102, x2: 8.251181102, y1: -5.334321614, y2: -5.846132638 }
        ]; "path double quotes present"
    )]
    fn parses_board_block(mut input: &str, expected: Vec<PerimeterSegment>) {
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
    // new line malversations
    #[test_case(
        "{BOARD \"/path/to/file/example.kicad_pcb\"\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.334321614 X2=8.251181102 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.846132638 X2=7.778740157 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.846132638 X2=7.778740157 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.334321614 X2=7.608267717 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-5.334321614 X2=7.608267717 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-4.487864921 X2=8.415354331 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-4.487864921 X2=8.415354331 Y2=-5.334321614)\n }",
        (
            StrContext::Label("line ending"),
            StrContext::Expected(StrContextValue::Description("expected single line ending or new line with 2 spaces after"))
        ); "last row new line"
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
            StrContext::Label("row name"),
            StrContext::Expected(StrContextValue::Description("PERIMETER_SEGMENT"))
        )
    )]
    #[test_case(
    "{BOARD \"/path/to/file/example.kicad_pcb\"\n  (PERIMETER_SEGMENTxyz X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.334321614 X2=8.251181102 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=8.251181102 Y1=-5.846132638 X2=7.778740157 Y2=-5.846132638)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.846132638 X2=7.778740157 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.778740157 Y1=-5.334321614 X2=7.608267717 Y2=-5.334321614)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-5.334321614 X2=7.608267717 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=7.608267717 Y1=-4.487864921 X2=8.415354331 Y2=-4.487864921)\n  (PERIMETER_SEGMENT X1=8.415354331 Y1=-4.487864921 X2=8.415354331 Y2=-5.334321614)\n}",
        (
            StrContext::Label("row name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after row name"))
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
