use winnow::{ModalResult, Parser};

use crate::model::Device;
use crate::parser::utils::{ExtractValueErrorMessages, close_row, extract_string, row_open};

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
}
