use winnow::{ModalResult, Parser};
use winnow::combinator::repeat;

use crate::model::Device;
use crate::parser::utils::{ExtractValueErrorMessages, close_block, close_row, extract_string, open_block, row_open};

fn parse_device_row(input: &mut &str) -> ModalResult<Device> {
    row_open(input, "?")?;

    let reference = extract_string(
        input,
        "REF",
        ExtractValueErrorMessages {
            key_error_label: "device REF key",
            key_error_description: "invalid REF key name",
            value_error_label: "device REF value",
            value_error_description: "invalid REF attribute value",
        }
    )?;

    let layer = extract_string(
        input,
        "L",
        ExtractValueErrorMessages {
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
    use winnow::error::ErrMode;
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
}
