// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::{dec_int, dec_uint, multispace0};
use winnow::combinator::{alt, backtrack_err, cut_err, delimited, peek};
use winnow::error::{StrContext, StrContextValue};
use winnow::token::{literal, one_of, take_till, take_until, take_while};
use winnow::{ModalResult, Parser};

pub struct ExtractKeyErrorMessages {
    pub key_error_label: &'static str,
    pub key_error_description: &'static str,
}

pub struct ExtractValueErrorMessages {
    pub value_error_label: &'static str,
    pub value_error_description: &'static str,
}

#[derive(Clone)]
pub struct ExtractEntryErrorMessages {
    pub key_error_label: &'static str,
    pub key_error_description: &'static str,
    pub value_error_label: &'static str,
    pub value_error_description: &'static str,
}

pub fn open_block<'a>(input: &mut &'a str, lit: &'static str) -> ModalResult<()> {
    backtrack_err((multispace0, "{", multispace0))
        .context(StrContext::Label("opening block"))
        .context(StrContext::Expected(StrContextValue::Description("{")))
        .parse_next(input)?;

    if lit != "" {
        backtrack_err(literal(lit))
            .context(StrContext::Label("block name"))
            .context(StrContext::Expected(StrContextValue::Description(lit)))
            .parse_next(input)?;

        // applies one_of() parser without advancing the input
        // it only checks if there is no malversation after block name
        peek(one_of(['=', ' ', '\t', '\r', '\n']))
            .context(StrContext::Label("block name boundary"))
            .context(StrContext::Expected(StrContextValue::Description(
                "valid character after block name",
            )))
            .parse_next(input)?;
    }

    multispace0.parse_next(input)?;

    Ok(())
}

pub fn close_block(input: &mut &str) -> ModalResult<()> {
    cut_err((multispace0, "}", multispace0))
        .context(StrContext::Label("closing block"))
        .context(StrContext::Expected(StrContextValue::Description("}")))
        .parse_next(input)?;

    Ok(())
}

pub fn row_open<'a>(input: &mut &str, lit: &'static str) -> ModalResult<()> {
    backtrack_err((multispace0, "(", multispace0))
        .context(StrContext::Label("opening row"))
        .context(StrContext::Expected(StrContextValue::Description("(")))
        .parse_next(input)?;

    if lit != "" {
        backtrack_err(literal(lit))
            .context(StrContext::Label("row name"))
            .context(StrContext::Expected(StrContextValue::Description(lit)))
            .parse_next(input)?;

        peek(one_of(['=', ' ', '\t', '\r', '\n']))
            .context(StrContext::Label("row name boundary"))
            .context(StrContext::Expected(StrContextValue::Description(
                "valid character after row name",
            )))
            .parse_next(input)?;
    }

    multispace0.parse_next(input)?;

    Ok(())
}

pub fn close_row(input: &mut &str) -> ModalResult<()> {
    backtrack_err((multispace0, ")", multispace0))
        .context(StrContext::Label("closing row"))
        .context(StrContext::Expected(StrContextValue::Description(")")))
        .parse_next(input)?;

    Ok(())
}

fn extract_key(
    input: &mut &str,
    key: &str,
    error_messages: ExtractKeyErrorMessages,
) -> ModalResult<()> {
    (literal(key), peek(one_of(['=', ' ', '\t', '\r', '\n'])))
        .void() // check if unwanted characters are present after key name
        .context(StrContext::Label(error_messages.key_error_label))
        .context(StrContext::Expected(StrContextValue::Description(
            error_messages.key_error_description,
        )))
        .parse_next(input)
}

pub fn extract_float_value(
    input: &mut &str,
    till_literal: char,
    error_messages: ExtractValueErrorMessages,
) -> ModalResult<f64> {
    let value: f64 = backtrack_err(
        // for some reason float and terminated(float, multispace0) combinators doesn't work here
        take_while(1.., |c: char| !c.is_whitespace() && c != till_literal).parse_to(),
    )
    .context(StrContext::Label(error_messages.value_error_label))
    .context(StrContext::Expected(StrContextValue::Description(
        error_messages.value_error_description,
    )))
    .parse_next(input)?;

    Ok(value)
}

pub fn extract_quoted_string_value(
    input: &mut &str,
    error_messages: ExtractValueErrorMessages,
) -> ModalResult<String> {
    let value: &str = backtrack_err(
        delimited('"', take_until(0.., '"'), '"'),
    )
    .context(StrContext::Label(error_messages.value_error_label))
    .context(StrContext::Expected(StrContextValue::Description(
        error_messages.value_error_description,
    )))
    .parse_next(input)?;

    Ok(String::from(value))
}

pub fn extract_unquoted_string_value(
    input: &mut &str,
    till_literal: char,
    error_messages: ExtractValueErrorMessages,
) -> ModalResult<String> {
    let value: &str = backtrack_err(
        take_till(1.., |c: char| c == till_literal),
    )
    .context(StrContext::Label(error_messages.value_error_label))
    .context(StrContext::Expected(StrContextValue::Description(
        error_messages.value_error_description,
    )))
    .parse_next(input)?;

    Ok(String::from(value))
}

pub fn extract_unsigned_integer_value(
    input: &mut &str,
    till_literal: char,
    error_messages: ExtractValueErrorMessages,
) -> ModalResult<u32> {
    let value = backtrack_err(dec_uint)
        .context(StrContext::Label(error_messages.value_error_label))
        .context(StrContext::Expected(StrContextValue::Description(
            error_messages.value_error_description,
        )))
        .parse_next(input)?;

    Ok(value)
}

pub fn extract_signed_integer_value(
    input: &mut &str,
    till_literal: char,
    error_messages: ExtractValueErrorMessages,
) -> ModalResult<i32> {
    let value = backtrack_err(dec_int)
        .context(StrContext::Label(error_messages.value_error_label))
        .context(StrContext::Expected(StrContextValue::Description(
            error_messages.value_error_description,
        )))
        .parse_next(input)?;

    Ok(value)
}

pub fn extract_float_entry(
    input: &mut &str,
    key: &str,
    till_literal: char,
    error_messages: ExtractEntryErrorMessages,
) -> ModalResult<f64> {
    extract_key(
        input,
        key,
        ExtractKeyErrorMessages {
            key_error_label: error_messages.key_error_label,
            key_error_description: error_messages.key_error_description,
        },
    )?;
    multispace0(input)?;

    literal('=').parse_next(input)?;
    multispace0(input)?;

    let value: f64 = extract_float_value(
        input,
        till_literal,
        ExtractValueErrorMessages {
            value_error_label: error_messages.value_error_label,
            value_error_description: error_messages.value_error_description,
        },
    )?;
    multispace0(input)?;

    Ok(value)
}

pub fn extract_quoted_string_entry(
    input: &mut &str,
    key: &str,
    error_messages: ExtractEntryErrorMessages,
) -> ModalResult<String> {
    extract_key(
        input,
        key,
        ExtractKeyErrorMessages {
            key_error_label: error_messages.key_error_label,
            key_error_description: error_messages.key_error_description,
        },
    )?;
    multispace0(input)?;

    literal('=').parse_next(input)?;
    multispace0(input)?;

    let value = extract_quoted_string_value(
        input,
        ExtractValueErrorMessages {
            value_error_label: error_messages.value_error_label,
            value_error_description: error_messages.value_error_description,
        },
    )?;
    multispace0(input)?;

    Ok(value)
}

pub fn extract_unquoted_string_entry(
    input: &mut &str,
    key: &str,
    till_literal: char,
    error_messages: ExtractEntryErrorMessages,
) -> ModalResult<String> {
    extract_key(
        input,
        key,
        ExtractKeyErrorMessages {
            key_error_label: error_messages.key_error_label,
            key_error_description: error_messages.key_error_description,
        },
    )?;
    multispace0(input)?;

    literal('=').parse_next(input)?;
    multispace0(input)?;

    let value = extract_unquoted_string_value(
        input,
        till_literal,
        ExtractValueErrorMessages {
            value_error_label: error_messages.value_error_label,
            value_error_description: error_messages.value_error_description,
        },
    )?;
    multispace0(input)?;

    Ok(value)
}

pub fn extract_unsigned_integer_entry(
    input: &mut &str,
    key: &str,
    till_literal: char,
    error_messages: ExtractEntryErrorMessages,
) -> ModalResult<u32> {
    extract_key(
        input,
        key,
        ExtractKeyErrorMessages {
            key_error_label: error_messages.key_error_label,
            key_error_description: error_messages.key_error_description,
        },
    )?;
    multispace0(input)?;

    literal('=').parse_next(input)?;
    multispace0(input)?;

    let value = extract_unsigned_integer_value(
        input,
        till_literal,
        ExtractValueErrorMessages {
            value_error_label: error_messages.value_error_label,
            value_error_description: error_messages.value_error_description,
        },
    )?;
    multispace0(input)?;

    Ok(value)
}

pub fn extract_signed_integer_entry(
    input: &mut &str,
    key: &str,
    till_literal: char,
    error_messages: ExtractEntryErrorMessages,
) -> ModalResult<i32> {
    extract_key(
        input,
        key,
        ExtractKeyErrorMessages {
            key_error_label: error_messages.key_error_label,
            key_error_description: error_messages.key_error_description,
        },
    )?;
    multispace0(input)?;

    literal('=').parse_next(input)?;
    multispace0(input)?;

    let value = extract_signed_integer_value(
        input,
        till_literal,
        ExtractValueErrorMessages {
            value_error_label: error_messages.value_error_label,
            value_error_description: error_messages.value_error_description,
        },
    )?;
    multispace0(input)?;

    Ok(value)
}
