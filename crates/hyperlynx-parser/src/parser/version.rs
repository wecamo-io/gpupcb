// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use crate::parser::utils::{close_block, open_block};
use winnow::ascii::digit1;
use winnow::error::{StrContext, StrContextValue};
use winnow::prelude::*;
use winnow::token::literal;

pub fn parse_version_block(input: &mut &str) -> ModalResult<String> {
    open_block(input, "VERSION")?;

    literal('=').parse_next(input)?;

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

    #[test_case(
        "{VERSION=2.14}", "2.14"; "single new line"
    )]
    fn parses_version_block(mut input: &str, expected: &str) {
        let version = parse_version_block.parse_next(&mut input).unwrap();

        assert_eq!(version, expected);
    }

    #[test_case(
        "xyz{VERSION=2.14}",
        (
            StrContext::Label("opening block"),
            StrContext::Expected(StrContextValue::Description("{"))
        ); "opening block corruption"
    )]
    #[test_case(
        "{VERxyzSION=2.14}",
        (
            StrContext::Label("block name"),
            StrContext::Expected(StrContextValue::Description("VERSION"))
        ); "block name corruption"
    )]
    #[test_case(
        "{VERSIONxyz=2.14}",
        (
            StrContext::Label("block name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after block name"))
        ); "opening block corruption"
    )]
    #[test_case(
        "{VERSION=xyz2.14}",
        (
            StrContext::Label("value boundary"),
            StrContext::Expected(StrContextValue::Description("valid form of version value"))
        ); "before first digit corruption"
    )]
    #[test_case(
        "{VERSION=2xyz.14}",
        (
            StrContext::Label("value boundary"),
            StrContext::Expected(StrContextValue::Description("valid form of version value"))
        ); "after first digit corruption"
    )]
    #[test_case(
        "{VERSION=2xyz14}",
        (
            StrContext::Label("value boundary"),
            StrContext::Expected(StrContextValue::Description("valid form of version value"))
        ); "after first digit corruption"
    )]
    #[test_case(
        "{VERSION=2.xyz14}",
        (
            StrContext::Label("value boundary"),
            StrContext::Expected(StrContextValue::Description("valid form of version value"))
        ); "before second digit corruption"
    )]
    #[test_case(
        "{VERSION=2.14xyz}",
        (
            StrContext::Label("closing block"),
            StrContext::Expected(StrContextValue::Description("}"))
        ); "closing block corruption"
    )]
    fn rejects_malformed_blocks(mut input: &str, expected_errors: (StrContext, StrContext)) {
        let err = parse_version_block.parse_next(&mut input).unwrap_err();

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

    #[test_case("{VERSION=2.14}\n{UNITS=ENGLISH LENGTH}", "\n{UNITS=ENGLISH LENGTH}")]
    fn leaves_content_for_the_next_parser(mut input: &str, expected: &str) {
        let version = parse_version_block.parse_next(&mut input).unwrap();

        assert_eq!(version, "2.14");
        assert_eq!(input, expected);
    }
}
