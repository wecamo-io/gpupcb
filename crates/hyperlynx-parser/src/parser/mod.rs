// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::multispace0;
use winnow::{ModalResult, Parser};
use winnow::token::{literal, take_till};

fn open_block(input: &mut &str, lit: &str) -> ModalResult<()> {
    multispace0(input)?;
    literal('{').parse_next(input)?;
    multispace0(input)?;
    literal(lit).parse_next(input)?;
    multispace0(input)?;

    Ok(())
}

fn close_block(input: &mut &str) -> ModalResult<()> {
    multispace0(input)?;
    literal("}").parse_next(input)?;
    multispace0(input)?;

    Ok(())
}

pub fn parse_version(input: &mut &str) -> ModalResult<String> {
    open_block(input, "VERSION")?;

    literal('=').parse_next(input)?;
    multispace0(input)?;
    let version = take_till(0.., '}').parse_next(input)?;

    close_block(input)?;
    
    Ok(version.trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    // VERSION BLOCK UNIT TESTS

    #[test]
    fn parses_version_block() {
        let mut input = "{VERSION=2.14}";
        let expected = "2.14";

        let version = parse_version.parse_next(&mut input).unwrap();

        assert_eq!(version, expected);
    }

    #[test]
    fn parses_version_block_with_whitespaces() {
        let mut input = "{   VERSION
          =    2.14
          
    } 
        ";
        let expected = "2.14";

        let version = parse_version.parse_next(&mut input).unwrap();

        assert_eq!(version, expected);
    }
}
