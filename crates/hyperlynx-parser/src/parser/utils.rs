// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::{multispace0};
use winnow::combinator::{backtrack_err, peek};
use winnow::token::{literal, one_of, take_while};
use winnow::{ModalResult, Parser};
use winnow::error::{StrContext, StrContextValue};

pub fn open_block<'a>(
    input: &mut &'a str,
    lit: &'static str,
) -> ModalResult<()> {
    backtrack_err((multispace0, "{", multispace0))
        .context(StrContext::Label("opening block"))
        .context(StrContext::Expected(StrContextValue::Description("{")))
        .parse_next(input)?;

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

    multispace0.parse_next(input)?;

    Ok(())
}

pub fn close_block(input: &mut &str) -> ModalResult<()> {
    backtrack_err((multispace0, "}", multispace0))
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

    backtrack_err(literal(lit))
        .context(StrContext::Label("row name"))
        .context(StrContext::Expected(StrContextValue::Description(lit)))
        .parse_next(input)?;

    peek(one_of(['=', ' ', '\t', '\r', '\n']))
        .context(StrContext::Label("block name boundary"))
        .context(StrContext::Expected(StrContextValue::Description(
            "valid character after row name",
        )))
        .parse_next(input)?;

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

pub fn extract_float(
    input: &mut &str,
    key: &str,
    key_error_label: &'static str,
    key_error_description: &'static str,
    value_error_label: &'static str,
    value_error_description: &'static str,
) -> ModalResult<f64> {
    backtrack_err(literal(key))
        .context(StrContext::Label(key_error_label))
        .context(StrContext::Expected(StrContextValue::Description(key_error_description)))
        .parse_next(input)?;
    multispace0(input)?;
    literal('=').parse_next(input)?;
    multispace0(input)?;
    let value: f64 = backtrack_err(
        // for some reason float and terminated(float, multispace0) combinators doesn't work here
        take_while(1.., |c: char| !c.is_whitespace()).parse_to()
    )
        .context(StrContext::Label(value_error_label))
        .context(StrContext::Expected(StrContextValue::Description(value_error_description))
    )
        .parse_next(input)?;
    multispace0(input)?;

    Ok(value)
}
