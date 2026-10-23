// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::{ModalResult, Parser};
use winnow::combinator::repeat;

use crate::model::Device;
use crate::parser::utils::{ExtractEntryErrorMessages, close_block, close_row, extract_string_entry, open_block, row_open};

fn parse_device_row(input: &mut &str) -> ModalResult<Device> {
    row_open(input, "?")?;

    let reference = extract_string_entry(
        input,
        "REF",
        ExtractEntryErrorMessages {
            key_error_label: "device REF key",
            key_error_description: "invalid REF key name",
            value_error_label: "device REF value",
            value_error_description: "invalid REF attribute value",
        }
    )?;

    let layer = extract_string_entry(
        input,
        "L",
        ExtractEntryErrorMessages {
            key_error_label: "device L key",
            key_error_description: "invalid L key name",
            value_error_label: "device L value",
            value_error_description: "invalid L attribute value",
        }
    )?;

    close_row(input)?;

    Ok(Device {
        reference,
        layer,
    })
}

pub fn parse_devices_block(input: &mut &str) -> ModalResult<Vec<Device>> {
    open_block(input, "DEVICES")?;

    let devices: Vec<Device> = repeat(0.., parse_device_row).parse_next(input)?;

    close_block(input)?;

    Ok(devices)
}

#[cfg(test)]
mod tests {
    use super::*;
    use winnow::error::{ErrMode, StrContext, StrContextValue};
    use test_case::test_case;

    #[test_case(
        r#"(? REF="R3" L="F.Cu")"#,
        Device { reference: String::from("R3"), layer: String::from("F.Cu") }
    )]
    #[test_case(
        r#"(? REF="D1" L="F.Cu")"#,
        Device { reference: String::from("D1"), layer: String::from("F.Cu") }
    )]
    #[test_case(
        r#"(? REF="U2" L="B.Cu")"#,
        Device { reference: String::from("U2"), layer: String::from("B.Cu") }
    )]
    #[test_case(
        r#"(? REF="SW1" L="F.Cu")"#,
        Device { reference: String::from("SW1"), layer: String::from("F.Cu") }
    )]
    fn parses_device(input: &str, expected: Device) {
        let mut input = input;

        let device = parse_device_row.parse_next(&mut input).unwrap();

        assert_eq!(device, expected);
    }

    #[test]
    fn parses_devices_block() {
        let mut input = "{DEVICES\n  (? REF=\"R3\" L=\"F.Cu\")\n  (? REF=\"D1\" L=\"F.Cu\")\n  (? REF=\"C3\" L=\"F.Cu\")\n  (? REF=\"U3\" L=\"F.Cu\")\n  (? REF=\"SW1\" L=\"B.Cu\")\n}";
        let expected = vec![
            Device { reference: String::from("R3"), layer: String::from("F.Cu") },
            Device { reference: String::from("D1"), layer: String::from("F.Cu") },
            Device { reference: String::from("C3"), layer: String::from("F.Cu") },
            Device { reference: String::from("U3"), layer: String::from("F.Cu") },
            Device { reference: String::from("SW1"), layer: String::from("B.Cu") },
        ];

        let devices = parse_devices_block.parse_next(&mut input).unwrap();

        assert_eq!(devices.len(), expected.len());

        for (i, device) in devices.iter().enumerate() {
            assert_eq!(*device, expected[i]);
        }
    }

    #[test_case(
        r#"xyz(? REF="R3" L="F.Cu")"#,
        (
        StrContext::Label("opening row"),
        StrContext::Expected(StrContextValue::Description("("))
        )
    )]
    #[test_case(
        r#"(xyz? REF="R3" L="F.Cu")"#,
        (
        StrContext::Label("row name"),
        StrContext::Expected(StrContextValue::Description("?"))
        )
    )]
    #[test_case(
        r#"(?xyz REF="R3" L="F.Cu")"#,
        (
        StrContext::Label("block name boundary"),
        StrContext::Expected(StrContextValue::Description("valid character after row name"))
        )
    )]
    // REF attribute malversations
    #[test_case(
        r#"(? xyzREF="R3" L="F.Cu")"#,
        (
        StrContext::Label("device REF key"),
        StrContext::Expected(StrContextValue::Description("invalid REF key name"))
        )
    )]
    #[test_case(
        r#"(? REFxyz="R3" L="F.Cu")"#,
        (
        StrContext::Label("device REF key"),
        StrContext::Expected(StrContextValue::Description("invalid REF key name"))
        )
    )]
    // L attribute malversations
    #[test_case(
        r#"(? REF="R3" xyzL="F.Cu")"#,
        (
        StrContext::Label("device L key"),
        StrContext::Expected(StrContextValue::Description("invalid L key name"))
        )
    )]
    #[test_case(
        r#"(? REF="R3" Lxyz="F.Cu")"#,
        (
        StrContext::Label("device L key"),
        StrContext::Expected(StrContextValue::Description("invalid L key name"))
        )
    )]
    #[test_case(
        r#"(? REF="R3" L="F.Cu" xyz)"#,
        (
        StrContext::Label("closing row"),
        StrContext::Expected(StrContextValue::Description(")"))
        )
    )]
    fn rejects_malformed_device_row(
        input: &str,
        expected_errors: (StrContext, StrContext)
    ) {
        let mut input = input;
        let err = parse_device_row.parse_next(&mut input).unwrap_err();

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

    #[test_case(
        r#"xyz{DEVICES\n  (? REF=\"R3\" L=\"F.Cu\")\n}"#,
        (
        StrContext::Label("opening block"),
        StrContext::Expected(StrContextValue::Description("{"))
        )
    )]
    #[test_case(
        r#"{xyzDEVICES\n  (? REF=\"R3\" L=\"F.Cu\")\n}"#,
        (
        StrContext::Label("block name"),
        StrContext::Expected(StrContextValue::Description("DEVICES"))
        )
    )]
    #[test_case(
        r#"{DEVICES\n  (? REF=\"R3\" L=\"F.Cu\")\nxyz}"#,
        (
        StrContext::Label("block name boundary"),
        StrContext::Expected(StrContextValue::Description("valid character after block name"))
        )
    )]
    #[test_case(
        r#"{DEVICES\n  (? REF=\"R3\" L=\"F.Cu\")\n}xyz"#,
        (
        StrContext::Label("block name boundary"),
        StrContext::Expected(StrContextValue::Description("valid character after block name"))
        )
    )]
    fn rejects_malformed_devices_block(
        mut input: &str,
        expected_errors: (StrContext, StrContext)
    ) {
        let err = parse_devices_block.parse_next(&mut input).unwrap_err();

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
