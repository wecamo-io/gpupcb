// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::{multispace0};
use winnow::combinator::{backtrack_err, peek};
use winnow::token::{literal, one_of};
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
