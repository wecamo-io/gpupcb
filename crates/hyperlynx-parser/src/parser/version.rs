// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use crate::parser::utils::{close_block, double_line_ending, open_block, single_line_ending};
use winnow::ascii::{digit1, line_ending};
use winnow::combinator::alt;
use winnow::error::{ContextError, ErrMode, StrContext, StrContextValue};
use winnow::token::{literal, one_of};
use winnow::{ModalResult, Parser};

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

    alt((single_line_ending, double_line_ending))
        .map_err(|_| {
            let mut err = ContextError::new();
            err.push(StrContext::Label("line ending"));
            err.push(StrContext::Expected(StrContextValue::Description(
                "expected single or double line ending",
            )));
            return ErrMode::Cut(err);
        })
        .parse_next(input)?;

    Ok(version.trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::ErrMode;

    // VERSION BLOCK UNIT TESTS

    #[test_case(
        "{VERSION=2.14}\n{...}", "2.14"; "single new line"
    )]
    #[test_case(
        "{VERSION=2.14}\n\n{...}", "2.14"; "double"
    )]
    fn parses_version_block(mut input: &str, expected: &str) {
        let version = parse_version_block.parse_next(&mut input).unwrap();

        assert_eq!(version, expected);
    }

    #[test_case(
        "xyz{VERSION=2.14}\n",
        (
            StrContext::Label("opening block"),
            StrContext::Expected(StrContextValue::Description("{"))
        ); "opening block corruption"
    )]
    #[test_case(
        "{VERxyzSION=2.14}\n",
        (
            StrContext::Label("block name"),
            StrContext::Expected(StrContextValue::Description("VERSION"))
        ); "block name corruption"
    )]
    #[test_case(
        "{VERSIONxyz=2.14}\n",
        (
            StrContext::Label("block name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after block name"))
        ); "opening block corruption"
    )]
    #[test_case(
        "{VERSION=xyz2.14}\n",
        (
            StrContext::Label("value boundary"),
            StrContext::Expected(StrContextValue::Description("valid form of version value"))
        ); "before first digit corruption"
    )]
    #[test_case(
        "{VERSION=2xyz.14}\n",
        (
            StrContext::Label("value boundary"),
            StrContext::Expected(StrContextValue::Description("valid form of version value"))
        ); "after first digit corruption"
    )]
    #[test_case(
        "{VERSION=2xyz14}\n",
        (
            StrContext::Label("value boundary"),
            StrContext::Expected(StrContextValue::Description("valid form of version value"))
        ); "after first digit corruption"
    )]
    #[test_case(
        "{VERSION=2.xyz14}\n",
        (
            StrContext::Label("value boundary"),
            StrContext::Expected(StrContextValue::Description("valid form of version value"))
        ); "before second digit corruption"
    )]
    #[test_case(
        "{VERSION=2.14xyz}\n",
        (
            StrContext::Label("closing block"),
            StrContext::Expected(StrContextValue::Description("}"))
        ); "closing block corruption"
    )]
    #[test_case(
        "{VERSION=2.14}",
        (
            StrContext::Label("line ending"),
            StrContext::Expected(StrContextValue::Description("expected single or double line ending"))
        ); "missing line ending"
    )]
    #[test_case(
        "{VERSION=2.14} {",
        (
            StrContext::Label("line ending"),
            StrContext::Expected(StrContextValue::Description("expected single or double line ending"))
        ); "missing line ending"
    )]
    #[test_case(
        "{VERSION=2.14}\n {",
        (
            StrContext::Label("line ending"),
            StrContext::Expected(StrContextValue::Description("expected single or double line ending"))
        ); "invalid one-space indentation"
    )]
    fn rejects_malformed_blocks(mut input: &str, expected_errors: (StrContext, StrContext)) {
        let err = parse_version_block.parse_next(&mut input).unwrap_err();

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

    #[test_case("{VERSION=2.14}\n{UNITS=ENGLISH LENGTH}", "{UNITS=ENGLISH LENGTH}")]
    fn leaves_content_for_the_next_parser(mut input: &str, expected: &str) {
        let version = parse_version_block.parse_next(&mut input).unwrap();

        assert_eq!(version, "2.14");
        assert_eq!(input, expected);
    }
}
