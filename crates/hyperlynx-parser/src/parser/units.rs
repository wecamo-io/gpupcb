// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::{multispace0};
use winnow::token::{literal, take_till};
use winnow::{ModalResult, Parser};
use crate::model::Units;
use crate::parser::utils::{open_block, close_block};

pub fn units_combinator(input: &mut &str) -> ModalResult<Units> {
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
    use crate::model::Units;

    #[test_case("{UNITS=ENGLISH LENGTH}", Units::English)]
    #[test_case("{UNITS=FRENCH LENGTH}", Units::WrongUnit(String::from("FRENCH LENGTH")))]
    fn units_combinator_parses_value(input: &str, expected: Units) {
        let mut input = input;

        let units = units_combinator.parse_next(&mut input).unwrap();

        assert_eq!(units, expected);
    }

    #[test_case(
        "{UNITS=ENGLISH LENGTH}\n{VERSION=2.14}",
        "{VERSION=2.14}",
        Units::English
    )]
    fn units_combinator_leaves_further_content(input: &str, content_left: &str, expected: Units) {
        let mut input = input;

        let units = units_combinator.parse_next(&mut input).unwrap();

        assert_eq!(units, expected);
        assert_eq!(input, content_left);
    }
}
