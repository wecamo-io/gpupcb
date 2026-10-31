// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use crate::model::Units;
use crate::parser::utils::{
    ExtractEntryErrorMessages, build_parser_backtrack_error, close_block,
    extract_unquoted_string_entry, open_block,
};
use winnow::prelude::*;

pub fn parse_units_block(input: &mut &str) -> ModalResult<Units> {
    open_block(input, "")?;

    let units = {
        let u = extract_unquoted_string_entry(
            input,
            "UNITS",
            '}',
            ExtractEntryErrorMessages {
                key_error_label: "units UNITS key",
                key_error_description: "invalid UNITS key name",
                value_error_label: "units UNITS value",
                value_error_description: "invalid UNITS attribute value",
            },
        )?;

        match u.as_str() {
            "ENGLISH LENGTH" => Units::English,
            _ => {
                Err(build_parser_backtrack_error((
                    "units UNITS value",
                    "invalid UNITS attribute value",
                )))
            }?,
        }
    };

    close_block(input)?;

    Ok(units)
}

#[cfg(test)]
mod units_block_parsertests {
    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};

    #[test_case("{UNITS=ENGLISH LENGTH}", Units::English)]
    fn parses_units_block(mut input: &str, expected: Units) {
        let units = parse_units_block.parse_next(&mut input).unwrap();

        assert_eq!(units, expected);
    }

    #[test_case("{UNITS=ENGLISH LENGTH}\n\n{...}", "\n\n{...}", Units::English)]
    fn leaves_content_for_the_next_parser(mut input: &str, content_left: &str, expected: Units) {
        let units = parse_units_block.parse_next(&mut input).unwrap();

        assert_eq!(units, expected);
        assert_eq!(input, content_left);
    }

    #[test_case(
        "xyz{UNITS=ENGLISH LENGTH}",
        (
            StrContext::Label("opening block"),
            StrContext::Expected(StrContextValue::Description("{"))
        ); "opening block corruption"
    )]
    #[test_case(
        "{UNITxyzS=ENGLISH LENGTH}",
        (
            StrContext::Label("units UNITS key"),
            StrContext::Expected(StrContextValue::Description("invalid UNITS key name"))
        ); "block name corruption"
    )]
    #[test_case(
        "{UNITSxyz=ENGLISH LENGTH}",
        (
            StrContext::Label("units UNITS key"),
            StrContext::Expected(StrContextValue::Description("invalid UNITS key name"))
        ); "block name boundary corruption"
    )]
    #[test_case(
        "{UNITS=ENGLISH LENGTH",
        (
            StrContext::Label("closing block"),
            StrContext::Expected(StrContextValue::Description("}"))
        ); "missing closing bracket"
    )]
    #[test_case(
        "{UNITS=FRENCH LENGTH",
        (
            StrContext::Label("units UNITS value"),
            StrContext::Expected(StrContextValue::Description("invalid UNITS attribute value"))
        ); "value corruption"
    )]
    // invalid new lines -
    #[test_case(
        "{UNITS=FRENCH LENGTH",
        (
            StrContext::Label("units UNITS value"),
            StrContext::Expected(StrContextValue::Description("invalid UNITS attribute value"))
        ); "value corruption"
    )]
    fn rejects_malformed_units_blocks(mut input: &str, expected_errors: (StrContext, StrContext)) {
        let err = parse_units_block.parse_next(&mut input).unwrap_err();

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
}
