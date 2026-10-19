// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::{multispace0};
use winnow::token::{literal, take_till};
use winnow::{ModalResult, Parser};
use crate::parser::utils::{open_block, close_block};

pub fn units_block(input: &mut &str) -> ModalResult<String> {
    open_block(input, "UNITS")?;

    literal('=').parse_next(input)?;
    multispace0(input)?;

    let units = take_till(0.., '}')
        .parse_next(input)?;

    close_block(input)?;

    Ok(String::from(units))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use crate::model::Units;

    #[test_case("{UNITS=ENGLISH LENGTH}", Units::English; "no whitespaces")]
    #[test_case("  {  
            UNITS  = 
    ENGLISH    LENGTH  
         }", Units::English; "whitespaces present")]
    fn parses_units_block(input: &str, expected: Units) {
        let mut input = input;

        let units = units_block.parse_next(&mut input).unwrap();
    }

}
