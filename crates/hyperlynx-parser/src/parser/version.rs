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
