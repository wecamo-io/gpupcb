// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::{float, multispace0};
use winnow::combinator::backtrack_err;
use winnow::token::literal;
use winnow::{ModalResult, Parser};
use winnow::error::{StrContext, StrContextValue};

use crate::parser::utils::{ExtractValueErrorMessages, close_row, extract_float};
use crate::{model::PerimeterSegment, parser::utils::row_open};

fn parse_perimeter_segment_row(input: &mut &str) -> ModalResult<PerimeterSegment> {
    row_open(input, "PERIMETER_SEGMENT")?;

    let x1 = extract_float(
        input,
        "X1",
        ExtractValueErrorMessages {
            key_error_label: "perimeter segment X1 key",
            key_error_description: "invalid X1 key name",
            value_error_label: "perimeter segment X1 value",
            value_error_description: "invalid X1 attribute value",
        },
    )?;

    let y1 = extract_float(
        input,
        "Y1",
        ExtractValueErrorMessages {
            key_error_label: "perimeter segment Y1 key",
            key_error_description: "invalid Y1 key name",
            value_error_label: "perimeter segment Y1 value",
            value_error_description: "invalid Y1 attribute value",
        },
    )?;

    let x2 = extract_float(
        input,
        "X2",
        ExtractValueErrorMessages {
            key_error_label: "perimeter segment X2 key",
            key_error_description: "invalid X2 key name",
            value_error_label: "perimeter segment X2 value",
            value_error_description: "invalid X2 attribute value",
        },
    )?;

    let y2 = extract_float(
        input,
        "Y2",
        ExtractValueErrorMessages {
            key_error_label: "perimeter segment Y2 key",
            key_error_description: "invalid Y2 key name",
            value_error_label: "perimeter segment Y2 value",
            value_error_description: "invalid Y2 attribute value",
        },
    )?;

    close_row(input)?;

    Ok(PerimeterSegment {
        x1,
        y1,
        x2,
        y2,
    })
}

mod tests {
    use super::*;
    use winnow::error::ErrMode;
    use test_case::test_case;

    #[test_case(
        "(PERIMETER_SEGMENT X1=8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)",
        PerimeterSegment {
            x1: 8.415354331,
            y1: -5.334321614,
            x2: 8.251181102,
            y2: -5.334321614,
        }; "no whitespaces"
    )]
    #[test_case(
        " ( PERIMETER_SEGMENT\n X1\t=\n8.415354331 Y1\r=\n -5.334321614   X2 =\t8.251181102 Y2 =-5.334321614  ) \t",
        PerimeterSegment {
            x1: 8.415354331,
            y1: -5.334321614,
            x2: 8.251181102,
            y2: -5.334321614,
        }; "no whitespaces"
    )]
    fn parses_single_perimeter_segment(input: &str, expected: PerimeterSegment) {
        let mut input = input;

        let perimeter_segment = parse_perimeter_segment_row(&mut input).unwrap();
    
        assert_eq!(perimeter_segment, expected);
    }

    #[test_case(
        "(PERIMETER_SEGMENT X1=xyz8.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)",
        StrContext::Label("perimeter segment X1 value"),
        StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_SEGMENT X1=8xyz.415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)",
        StrContext::Label("perimeter segment X1 value"),
        StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.xyz415354331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)",
        StrContext::Label("perimeter segment X1 value"),
        StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.4153xyz54331 Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)",
        StrContext::Label("perimeter segment X1 value"),
        StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
    )]
    #[test_case(
        "(PERIMETER_SEGMENT X1=8.415354331xyz Y1=-5.334321614 X2=8.251181102 Y2=-5.334321614)",
        StrContext::Label("perimeter segment X1 value"),
        StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
    )]
    fn rejects_malformed_perimeter_row(
        input: &str,
        expected_label: StrContext,
        expected_description: StrContext
    ) {
        let mut input = input;
        let err = parse_perimeter_segment_row(&mut input).unwrap_err();

        let errors = match err {
            ErrMode::Backtrack(ref err) | ErrMode::Cut(ref err) => {
                let errors = err.context().collect::<Vec<_>>();
                errors                
            }
            ErrMode::Incomplete(_) => todo!(),
        };

        assert_eq!(*errors[0], expected_label);
        assert_eq!(*errors[1], expected_description);
    }
}
