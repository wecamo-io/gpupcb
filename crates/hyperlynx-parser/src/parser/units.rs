// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use crate::model::Units;
use crate::parser::utils::{
    close_block, double_line_ending, extract_unquoted_string_entry, open_block,
    ExtractEntryErrorMessages,
};
use winnow::error::{ContextError, ErrMode, StrContext, StrContextValue};
use winnow::{ModalResult, Parser};

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
                let mut err = ContextError::new();
                err.push(StrContext::Label("units UNITS value"));
                err.push(StrContext::Expected(StrContextValue::Description(
                    "invalid UNITS attribute value",
                )));

                Err(ErrMode::Backtrack(err))
            }?,
        }
    };

    close_block(input)?;

    double_line_ending(input)?;

    Ok(units)
}

#[cfg(test)]
mod units_block_parsertests {
    use super::*;
    use test_case::test_case;

    #[test_case("{UNITS=ENGLISH LENGTH}\n\n{...}", Units::English)]
    fn parses_units_block(mut input: &str, expected: Units) {
        let units = parse_units_block.parse_next(&mut input).unwrap();

        assert_eq!(units, expected);
    }

    #[test_case("{UNITS=ENGLISH LENGTH}\n\n{...}", "{...}", Units::English)]
    fn leaves_content_for_the_next_parser(mut input: &str, content_left: &str, expected: Units) {
        let units = parse_units_block.parse_next(&mut input).unwrap();

        assert_eq!(units, expected);
        assert_eq!(input, content_left);
    }

    #[test_case(
        "xyz{UNITS=ENGLISH LENGTH}\n\n{...}",
        (
            StrContext::Label("opening block"),
            StrContext::Expected(StrContextValue::Description("{"))
        ); "opening block corruption"
    )]
    #[test_case(
        "{UNITxyzS=ENGLISH LENGTH}\n\n{...}",
        (
            StrContext::Label("units UNITS key"),
            StrContext::Expected(StrContextValue::Description("invalid UNITS key name"))
        ); "block name corruption"
    )]
    #[test_case(
        "{UNITSxyz=ENGLISH LENGTH}\n\n{...}",
        (
            StrContext::Label("units UNITS key"),
            StrContext::Expected(StrContextValue::Description("invalid UNITS key name"))
        ); "block name boundary corruption"
    )]
    #[test_case(
        "{UNITS=ENGLISH LENGTH\n\n{...}",
        (
            StrContext::Label("units UNITS value"),
            StrContext::Expected(StrContextValue::Description("invalid UNITS attribute value"))
        ); "missing closing bracket"
    )]
    #[test_case(
        "{UNITS=FRENCH LENGTH\n\n{...}",
        (
            StrContext::Label("units UNITS value"),
            StrContext::Expected(StrContextValue::Description("invalid UNITS attribute value"))
        ); "value corruption"
    )]
    // invalid new lines -
    #[test_case(
        "{UNITS=FRENCH LENGTH\n\n{...}",
        (
            StrContext::Label("units UNITS value"),
            StrContext::Expected(StrContextValue::Description("invalid UNITS attribute value"))
        ); "value corruption"
    )]
    fn rejects_malformed_units_blocks(mut input: &str, expected_errors: (StrContext, StrContext)) {
        let err = parse_units_block.parse_next(&mut input).unwrap_err();

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
}
