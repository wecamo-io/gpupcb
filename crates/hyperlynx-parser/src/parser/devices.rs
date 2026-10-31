// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::multispace0;
use winnow::combinator::{cut_err, peek, repeat};
use winnow::prelude::*;
use winnow::token::take_until;

use crate::model::Device;
use crate::parser::utils::{
    ExtractEntryErrorMessages, close_block, close_row, extract_quoted_string_entry,
    extract_string_entry, open_block, row_open, single_space, skip_ignored,
};

fn parse_device_row(input: &mut &str) -> ModalResult<Device> {
    row_open(input, "?")?;

    single_space(input)?;

    let reference: String = extract_string_entry(
        input,
        "REF",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "device REF key",
            key_error_description: "invalid REF key name",
            value_error_label: "device REF value",
            value_error_description: "invalid REF attribute value",
        },
    )?;

    // omit unnecessary NAME entry that appears in some exporterts
    take_until(0.., "L=").parse_next(input)?;

    let layer = extract_quoted_string_entry(
        input,
        "L",
        ExtractEntryErrorMessages {
            key_error_label: "device L key",
            key_error_description: "invalid L key name",
            value_error_label: "device L value",
            value_error_description: "invalid L attribute value",
        },
    )?;

    close_row(input)?;

    skip_ignored(input)?;

    Ok(Device { reference, layer })
}

fn parse_device_row_with_err(input: &mut &str) -> ModalResult<Device> {
    peek((multispace0, "(")).parse_next(input)?;

    cut_err(parse_device_row).parse_next(input)
}

pub fn parse_devices_block(input: &mut &str) -> ModalResult<Vec<Device>> {
    open_block(input, "DEVICES")?;

    skip_ignored(input)?;

    let devices: Vec<Device> = repeat(0.., parse_device_row_with_err).parse_next(input)?;

    close_block(input)?;

    Ok(devices)
}

#[cfg(test)]
mod device_block_parser_tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};

    #[test_case(
        "(? REF=\"R3\" L=\"F.Cu\")\n  (...)",
        Device { reference: String::from("R3"), layer: String::from("F.Cu") }
    )]
    #[test_case(
        "(? REF=\"D1\" L=\"F.Cu\")\n  (...)",
        Device { reference: String::from("D1"), layer: String::from("F.Cu") }
    )]
    #[test_case(
        "(? REF=\"U2\" L=\"B.Cu\")\n(...)",
        Device { reference: String::from("U2"), layer: String::from("B.Cu") }
    )]
    #[test_case(
        "(? REF=\"SW1\" L=\"F.Cu\")\n(...)",
        Device { reference: String::from("SW1"), layer: String::from("F.Cu") }
    )]
    #[test_case(
        "(? REF=SW1 L=\"F.Cu\")\n(...)",
        Device { reference: String::from("SW1"), layer: String::from("F.Cu") }
    )]
    #[test_case(
        "(? REF=  NAME=xyz L=\"F.Cu\")\n(...)",
        Device { reference: String::from(" "), layer: String::from("F.Cu") };
        "Altium Deisgner export case"
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
            Device {
                reference: String::from("R3"),
                layer: String::from("F.Cu"),
            },
            Device {
                reference: String::from("D1"),
                layer: String::from("F.Cu"),
            },
            Device {
                reference: String::from("C3"),
                layer: String::from("F.Cu"),
            },
            Device {
                reference: String::from("U3"),
                layer: String::from("F.Cu"),
            },
            Device {
                reference: String::from("SW1"),
                layer: String::from("B.Cu"),
            },
        ];

        let devices = parse_devices_block.parse_next(&mut input).unwrap();

        assert_eq!(devices.len(), expected.len());

        for (i, device) in devices.iter().enumerate() {
            assert_eq!(*device, expected[i]);
        }
    }

    #[test_case(
        "xyz(? REF=\"R3\" L=\"F.Cu\")\n  (...)",
        (
        StrContext::Label("opening row"),
        StrContext::Expected(StrContextValue::Description("("))
        )
    )]
    #[test_case(
        "(xyz? REF=\"R3\" L=\"F.Cu\")\n  (...)",
        (
        StrContext::Label("row name"),
        StrContext::Expected(StrContextValue::Description("?"))
        )
    )]
    #[test_case(
        "(?xyz REF=\"R3\" L=\"F.Cu\")\n  (...)",
        (
        StrContext::Label("row name boundary"),
        StrContext::Expected(StrContextValue::Description("valid character after row name"))
        )
    )]
    // REF attribute malversations
    #[test_case(
        "(? xyzREF=\"R3\" L=\"F.Cu\")\n  (...)",
        (
        StrContext::Label("device REF key"),
        StrContext::Expected(StrContextValue::Description("invalid REF key name"))
        )
    )]
    #[test_case(
        "(? REFxyz=\"R3\" L=\"F.Cu\")\n  (...)",
        (
        StrContext::Label("device REF key"),
        StrContext::Expected(StrContextValue::Description("invalid REF key name"))
        )
    )]
    // // TODO: should throw invalid REF value context
    // #[test_case(
    //     "(? REF=\"R3 L=\"F.Cu\")\n  (...)",
    //     (
    //     StrContext::Label("device L key"),
    //     StrContext::Expected(StrContextValue::Description("invalid L key name"))
    //     )
    // )]
    // L attribute malversations
    #[test_case(
        "(? REF=\"R3\" L=xyz\"F.Cu\")\n  (...)",
        (
        StrContext::Label("device L value"),
        StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
        )
    )]
    #[test_case(
        "(? REF=\"R3\" L=F.Cu\")\n  (...)",
        (
        StrContext::Label("device L value"),
        StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
        )
    )]
    #[test_case(
        "(? REF=\"R3\" L=\"F.Cu)\n  (...)",
        (
        StrContext::Label("device L value"),
        StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
        )
    )]
    // closing row
    #[test_case(
        "(? REF=\"R3\" L=\"F.Cu\" xyz)\n  (...)",
        (
        StrContext::Label("closing row"),
        StrContext::Expected(StrContextValue::Description(")"))
        )
    )]
    fn rejects_malformed_device_row(input: &str, expected_errors: (StrContext, StrContext)) {
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

    // opening block
    #[test_case(
        "xyz{DEVICES\\n  (? REF=\\\"R3\\\" L=\\\"F.Cu\\\")\\n}",
        (
            StrContext::Label("opening block"),
            StrContext::Expected(StrContextValue::Description("{"))
        )
    )]
    #[test_case(
        "{xyzDEVICES\\n  (? REF=\\\"R3\\\" L=\\\"F.Cu\\\")\\n}",
        (
            StrContext::Label("block name"),
            StrContext::Expected(StrContextValue::Description("DEVICES"))
        )
    )]
    // row malversations
    #[test_case(
        "{DEVICES\n  (? xyzREF=\"R3\" L=\"F.Cu\")\n}",
        (
            StrContext::Label("device REF key"),
            StrContext::Expected(StrContextValue::Description("invalid REF key name"))
        )
    )]
    #[test_case(
        "{DEVICES\n  (? REFxyz=\"R3\" L=\"F.Cu\")\n}",
        (
            StrContext::Label("device REF key"),
            StrContext::Expected(StrContextValue::Description("invalid REF key name"))
        )
    )]
    #[test_case(
        "{DEVICES\n  (? REF=\"R3\" L=F.Cu\")\n}",
        (
            StrContext::Label("device L value"),
            StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
        )
    )]
    // closing block
    #[test_case(
        "{DEVICES\n  (? REF=\"R3\" L=\"F.Cu\")\nxyz}",
        (
            StrContext::Label("closing block"),
            StrContext::Expected(StrContextValue::Description("}"))
        )
    )]
    #[test_case(
        "{DEVICESxyz\n  (? REF=\"R3\" L=\"F.Cu\")\n}xyz",
        (
            StrContext::Label("block name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after block name"))
        )
    )]
    fn rejects_malformed_devices_block(mut input: &str, expected_errors: (StrContext, StrContext)) {
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
