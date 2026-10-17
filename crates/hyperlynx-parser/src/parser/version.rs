// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::{digit1, multispace0};
use winnow::error::{StrContext, StrContextValue};
use winnow::token::{literal};
use winnow::{ModalResult, Parser};
use crate::parser::utils::{open_block, close_block};

pub fn parse_version(input: &mut &str) -> ModalResult<String> {
    open_block(input, "VERSION")?;

    literal('=').parse_next(input)?;
    multispace0(input)?;

    // ensure version is in form of d.d
    let version = (digit1, literal('.'), digit1)
        .take()
        .context(StrContext::Label("value boundary"))
        .context(StrContext::Expected(StrContextValue::Description(
            "valid form of version value",
        )))
        .parse_next(input)?;

    close_block(input)?;

    Ok(version.trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::ErrMode;

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

    #[test_case("xyz{VERSION=2.14}", StrContext::Label("opening block"); "opening block corruption")]
    #[test_case("{VERxyzSION=2.14}", StrContext::Expected(StrContextValue::Description("VERSION")); "block name corruption")]
    #[test_case("{VERSIONxyz=2.14}", StrContext::Label("block name boundary"); "opening block corruption")]
    #[test_case("{VERSION=xyz2.14}", StrContext::Label("value boundary"); "before first digit corruption")]
    #[test_case("{VERSION=2xyz.14}", StrContext::Label("value boundary"); "after first digit corruption")]
    #[test_case("{VERSION=2xyz14}", StrContext::Label("value boundary"); "after first digit corruption")]
    #[test_case("{VERSION=2.xyz14}", StrContext::Label("value boundary"); "before second digit corruption")]
    #[test_case("{VERSION=2.14xyz}", StrContext::Label("closing block"); "closing block corruption")]
    fn rejects_malformed_blocks(input: &str, expected: StrContext) {
        let mut input = input;
        let err = parse_version.parse_next(&mut input).unwrap_err();

        let has_expected_error = match err {
            ErrMode::Backtrack(ref err) | ErrMode::Cut(ref err) => {
                err.context().any(|ctx| *ctx == expected)
            }
            ErrMode::Incomplete(_) => todo!(),
        };

        assert!(has_expected_error, "Expected VERSION error, got {:?}", err);
    }

    #[test_case("{VERSION=2.14}xyz", "xyz"; "no whitespace")]
    #[test_case("{VERSION=2.14}         xyz", "xyz"; "with whitespace")]
    fn leaves_content_for_the_next_parser(input: &str, expected: &str) {
        let mut input = input;

        let version = parse_version.parse_next(&mut input).unwrap();

        assert_eq!(version, "2.14");
        assert_eq!(input, expected);
    }
}
