// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::{float, multispace0};
use winnow::combinator::backtrack_err;
use winnow::token::{literal};
use winnow::{ModalResult, Parser};
use winnow::error::{StrContext, StrContextValue};

use crate::parser::utils::close_row;
use crate::{model::PerimeterSegment, parser::utils::row_open};

fn parse_perimeter_segment_row(input: &mut &str) -> ModalResult<PerimeterSegment> {
    row_open(input, "PERIMETER_SEGMENT")?;
    multispace0(input)?;

    backtrack_err(literal("X1"))
        .context(StrContext::Label("perimeter segment X1 value"))
        .context(StrContext::Expected(StrContextValue::Description("invalid X1 attribute name")))
        .parse_next(input)?;
    multispace0(input)?;
    literal('=').parse_next(input)?;
    multispace0(input)?;
    let x1: f64 = backtrack_err(
        float
    )
        .context(StrContext::Label("perimeter segment X1 value is not float"))
        .context(StrContext::Expected(StrContextValue::Description("invalid X1 attribute value"))
    )
        .parse_next(input)?;
    multispace0(input)?;

    backtrack_err(literal("Y1"))
        .context(StrContext::Label("perimeter segment Y1 value"))
        .context(StrContext::Expected(StrContextValue::Description("invalid Y1 attribute name")))
        .parse_next(input)?;
    multispace0(input)?;
    literal('=').parse_next(input)?;
    multispace0(input)?;
    let y1: f64 = float.parse_next(input)?;
    multispace0(input)?;

    backtrack_err(literal("X2"))
        .context(StrContext::Label("perimeter segment X2 value"))
        .context(StrContext::Expected(StrContextValue::Description("invalid X2 attribute name")))
        .parse_next(input)?;
    multispace0(input)?;
    literal('=').parse_next(input)?;
    multispace0(input)?;
    let x2: f64 = float.parse_next(input)?;
    multispace0(input)?;

    backtrack_err(literal("Y2"))
        .context(StrContext::Label("perimeter segment Y2 value"))
        .context(StrContext::Expected(StrContextValue::Description("invalid Y2 attribute name")))
        .parse_next(input)?;
    multispace0(input)?;
    literal('=').parse_next(input)?;
    multispace0(input)?;
    let y2: f64 = float.parse_next(input)?;
    multispace0(input)?;

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
        StrContext::Label("perimeter segment X1 value is not float"),
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
