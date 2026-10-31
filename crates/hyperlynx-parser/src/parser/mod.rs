// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::multispace0;
use winnow::error::ContextError;
use winnow::{ModalResult, Parser};

use crate::model::{Board, Device, Layer, Net, Padstack, PerimeterSegment, Units};
use crate::parser::utils::{
    build_parser_backtrack_error, double_line_ending, parse_either_line_ending, single_line_ending,
};

mod devices;
mod net;
mod padstack;
mod perimeter_segment;
mod stackup;
mod units;
mod utils;
mod version;

#[derive(Debug)]
pub struct ReturnParseError {
    pub line: u32,
    pub placement: String,
    pub inner: ContextError,
    pub offset: usize,
}

fn parse_single_or_double_line_ending(input: &mut &str) -> ModalResult<()> {
    parse_either_line_ending(
        input,
        single_line_ending,
        double_line_ending,
        ("line ending", "expected single or double line ending"),
    )?;

    Ok(())
}

fn parse_file_content(input: &mut &str) -> ModalResult<Board> {
    // {VERSION}
    let version: String = version::parse_version_block(input)?;

    parse_single_or_double_line_ending(input)?;

    // {UNITS}
    let units: Units = units::parse_units_block(input)?;

    double_line_ending(input)?;

    // {BOARD}
    let outline: Vec<PerimeterSegment> = perimeter_segment::parse_board_block(input)?;

    double_line_ending(input)?;

    // {STACKUP}
    let stackup: Vec<Layer> = stackup::parse_stackup_block(input)?;

    double_line_ending(input)?;

    // {DEVICES}
    let devices: Vec<Device> = devices::parse_devices_block(input)?;

    double_line_ending(input)?;

    // {PADSTACK}
    let mut padstacks: Vec<Padstack> = Vec::new();
    loop {
        // solves the issue with first NET block after PADSTACKs
        // throwing block name - PADSTACK error context
        if input.is_empty() || input.starts_with("{N") {
            break;
        }

        if !input.starts_with("{PADSTACK=") {
            return Err(build_parser_backtrack_error(("block name", "PADSTACK")));
        }

        let (padstack, _): (Padstack, _) = (
            padstack::parse_padstack_block,
            parse_single_or_double_line_ending,
        )
            .parse_next(input)?;

        padstacks.push(padstack);
    }

    // double_line_ending between last PADSTACK and first NET is not required
    // because last iteration in parse_padstack_block loop already handles that

    // {NET}
    let mut nets: Vec<Net> = Vec::new();
    loop {
        if input.is_empty() || input.starts_with("{END}") {
            break;
        }

        if !input.starts_with("{NET=") {
            return Err(build_parser_backtrack_error(("block name", "NET")));
        }

        let (net, _): (Net, _) = (net::parse_net_block, multispace0).parse_next(input)?;
        nets.push(net);
    }

    // {END} and {KEY=...} are ignored
    *input = "";

    Ok(Board {
        version,
        units,
        outline,
        stackup,
        devices,
        padstacks,
        nets,
    })
}

pub fn parse_hyperlynx(input: &str) -> Result<Board, ReturnParseError> {
    let mut remaining = input;

    parse_file_content.parse(&mut remaining).map_err(|err| {
        let index: usize = err.offset();
        let s: &str = err.input();

        let space: usize = 16;
        let mut start: usize = index.saturating_sub(space);
        while !s.is_char_boundary(start) {
            start += 1;
        }
        let mut end: usize = index.saturating_add(space).min(s.len());
        while !s.is_char_boundary(end) {
            end += 1;
        }

        let placement: String = format!("...{}...", s[start..end].to_string());
        let line: u32 = s[..index].matches('\n').count() as u32 + 1;

        ReturnParseError {
            line,
            placement,
            inner: err.inner().clone(),
            offset: err.offset(),
        }
    })
}
