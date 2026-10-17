// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

mod utils;
pub mod version;

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};
    use winnow::prelude::*;
    use version::parse_version;

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
