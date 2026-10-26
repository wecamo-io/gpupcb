// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::{ModalResult, Parser};
use winnow::combinator::alt;
use winnow::error::{ContextError, ParseError};

use crate::model::{Board, Device, Layer, Net, Padstack, PerimeterSegment, Units};

mod devices;
mod net;
mod padstack;
mod perimeter_segment;
mod stackup;
mod units;
mod utils;
mod version;

enum Section {
    Version(String),
    Units(Units),
    PerimeterSegment(Vec<PerimeterSegment>),
    Stackup(Vec<Layer>),
    Devices(Vec<Device>),
    Padstack(Padstack),
    Net(Net),
}

// checks which section type is currently at the beggining of input (VERSION, UNITS, NET etc.)
fn section(input: &mut &str) -> ModalResult<Section> {
    alt((
        version::parse_version_block.map(Section::Version),
        units::parse_units_block.map(Section::Units),
        perimeter_segment::parse_board_block.map(Section::PerimeterSegment),
        stackup::parse_stackup_block.map(Section::Stackup),
        devices::parse_devices_block.map(Section::Devices),
        padstack::parse_padstack_block.map(Section::Padstack),
        net::parse_net_block.map(Section::Net),
    ))
    .parse_next(input)
}

fn parse_hyperlynx(input: &mut &str) -> ModalResult<Board> {
    let mut version = String::new();
    let mut units = Units::English;
    let mut outline = Vec::new();
    let mut stackup = Vec::new();
    let mut devices = Vec::new();
    let mut padstacks = Vec::new();
    let mut nets = Vec::new();

    loop {
        if input.is_empty() {
            break;
        }

        match section.parse_next(input) {
            Ok(Section::Version(v)) => {
                version = v;
            }
            Ok(Section::Units(u)) => {
                units = u;
            }
            Ok(Section::PerimeterSegment(pr)) => {
                outline = pr;
            }
            Ok(Section::Stackup(st)) => {
                stackup = st;
            }
            Ok(Section::Devices(ds)) => {
                devices = ds;
            }
            Ok(Section::Padstack(ps)) => {
                padstacks.push(ps);
            }
            Ok(Section::Net(n)) => {
                nets.push(n);
            }
            Err(err) => return Err(err),
        }
    }

    Ok(Board {
        version,
        units,
        outline,
        stackup,
        nets,
        devices,
        padstacks,
    })
}

pub fn parse(input: &str) -> Result<Board, ParseError<&str, ContextError>> {
    let mut remaining = input;

    parse_hyperlynx.parse(&mut remaining).map_err(|err| err)
}
