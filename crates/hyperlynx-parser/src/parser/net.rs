// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::{ModalResult, Parser};

use crate::model::Pin;
use crate::parser::utils::{ExtractEntryErrorMessages, close_row, extract_float_entry, extract_string_entry, extract_unsigned_integer_entry, row_open};

fn parse_pin_row(input: &mut &str) -> ModalResult<Pin> {
    row_open(input, "PIN")?;

    let x = extract_float_entry(
        input,
        "X",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "pin X key",
            key_error_description: "invalid X key name",
            value_error_label: "pin X value",
            value_error_description: "invalid X attribute value",
        },
    )?;

    let y = extract_float_entry(
        input,
        "Y",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "pin Y key",
            key_error_description: "invalid Y key name",
            value_error_label: "pin Y value",
            value_error_description: "invalid Y attribute value",
        },
    )?;

    let component_reference = extract_string_entry(
        input,
        "R",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "pin R key",
            key_error_description: "invalid R key name",
            value_error_label: "pin R value",
            value_error_description: "invalid R attribute value",
        },
    )?;

    let padstack_id = extract_unsigned_integer_entry(
        input,
        "P",
        ')',
        ExtractEntryErrorMessages {
            key_error_label: "pin P key",
            key_error_description: "invalid P key name",
            value_error_label: "pin P value",
            value_error_description: "invalid P attribute value",
        },
    )?;

    close_row(input)?;

    Ok(Pin {
        x,
        y,
        component_reference: String::from(component_reference),
        padstack_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        r#"(PIN X=8.2480314961 Y=-4.8622047244 R="R3.1" P=0)"#,
        Pin {
            x: 8.2480314961,
            y: -4.8622047244,
            component_reference: String::from("R3.1"),
            padstack_id: 0,
        }
    )]
    #[test_case(
        r#"(PIN X=8.3572834646 Y=-4.5978346457 R="U2.2" P=12)"#,
        Pin {
            x: 8.3572834646,
            y: -4.5978346457,
            component_reference: String::from("U2.2"),
            padstack_id: 12,
        }
    )]
    fn parses_pin_row(mut input: &str, expected: Pin) {
        let pin = parse_pin_row(&mut input).unwrap();

        assert_eq!(pin, expected);
    }
}
