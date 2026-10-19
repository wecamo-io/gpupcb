// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::{multispace0};
use winnow::token::{literal, take_till};
use winnow::{ModalResult, Parser};
use crate::model::Units;
use crate::parser::utils::{open_block, close_block};

pub fn parse_units(input: &mut &str) -> ModalResult<Units> {
    open_block(input, "UNITS")?;

    literal('=').parse_next(input)?;
    multispace0(input)?;

    let units = take_till(0.., '}')
        .parse_next(input)?;

    close_block(input)?;

    match units {
        "ENGLISH LENGTH" => Ok(Units::English),
        other => Ok(Units::WrongUnit(other.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};
    use crate::model::Units;

    #[test_case("{UNITS=ENGLISH LENGTH}", Units::English)]
    #[test_case("{UNITS=FRENCH LENGTH}", Units::WrongUnit(String::from("FRENCH LENGTH")))]
    fn parses_units_block(input: &str, expected: Units) {
        let mut input = input;

        let units = parse_units.parse_next(&mut input).unwrap();

        assert_eq!(units, expected);
    }

    #[test_case(
        "{UNITS=ENGLISH LENGTH}\n{VERSION=2.14}",
        "{VERSION=2.14}",
        Units::English
    )]
    fn leaves_content_for_the_next_parser(input: &str, content_left: &str, expected: Units) {
        let mut input = input;

        let units = parse_units.parse_next(&mut input).unwrap();

        assert_eq!(units, expected);
        assert_eq!(input, content_left);
    }

    #[test_case("xyz{UNITS=ENGLISH LENGTH}", StrContext::Label("opening block"); "opening block corruption")]
    #[test_case("{UNITxyzS=ENGLISH LENGTH}", StrContext::Expected(StrContextValue::Description("UNITS")); "block name corruption")]
    #[test_case("{UNITSxyz=ENGLISH LENGTH}", StrContext::Label("block name boundary"); "block name boundary corruption")]
    #[test_case("{UNITS=ENGLISH LENGTH", StrContext::Label("closing block"); "closing block corruption")]
    fn rejects_malformed_units_blocks(input: &str, expected: StrContext) {
        let mut input = input;
        let err = parse_units.parse_next(&mut input).unwrap_err();

        let has_expected_error = match err {
            ErrMode::Backtrack(ref err) | ErrMode::Cut(ref err) => {
                err.context().any(|ctx| *ctx == expected)
            }
            ErrMode::Incomplete(_) => todo!(),
        };

        assert!(has_expected_error, "Expected UNITS error context {:?}, got {:?}", expected, err);
    }
}
