// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use std::collections::HashSet;

use crate::model::Board;
use crate::parser::ReturnParseError;
use crate::parser::utils::build_parser_cut_error;

// checks that every layer reference in the PCB exists in its stackup.
pub fn validate_layer_references(board: &Board) -> Result<(), ReturnParseError> {
    let defined_layers: HashSet<&str> = board
        .stackup
        .iter()
        .map(|layer| layer.layer_name.as_str())
        .collect();

    let check_layer = |reference: &str| {
        if !defined_layers.contains(reference) {
            return Err(build_parser_cut_error((
                "Invalid layer reference",
                "a layer defined in the board stackup",
            )));
        }

        Ok(())
    };

    for device in &board.devices {
        check_layer(&device.layer)?;
    }

    // omit PADSTACK's PAD rows because they may point to non-listed layer

    for net in &board.nets {
        for segment in net.segments.iter() {
            check_layer(&segment.layer)?;
        }

        for segment in net.unrouted_segments.iter() {
            check_layer(&segment.layer1)?;

            check_layer(&segment.layer2)?;
        }

        for arc in net.arcs.iter() {
            check_layer(&arc.layer)?;
        }

        for polygon in net.polygons.iter() {
            check_layer(&polygon.layer)?;
        }

        for polyline in net.polylines.iter() {
            check_layer(&polyline.layer)?;
        }
    }

    Ok(())
}
