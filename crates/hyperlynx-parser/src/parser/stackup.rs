use winnow::{ModalResult, Parser};

use crate::model::{Layer, LayerMaterial, LayerType};
use crate::parser::utils::{ExtractValueErrorMessages, close_row, extract_float, extract_string, row_open};

fn dielectric(input: &mut &str) -> ModalResult<Layer> {
    row_open(input, "DIELECTRIC")?;

    let thickness = extract_float(
        input,
        "T",
        ExtractValueErrorMessages {
            key_error_label: "dielectric T key",
            key_error_description: "invalid T key name",
            value_error_label: "dielectric T value",
            value_error_description: "invalid T attribute value",
        }
    )?;

    let dielectric_constant = extract_float(
        input,
        "C",
        ExtractValueErrorMessages {
            key_error_label: "dielectric C key",
            key_error_description: "invalid C key name",
            value_error_label: "dielectric C value",
            value_error_description: "invalid C attribute value",
        }
    )?;

    let layer_name = extract_string(
        input,
        "L",
        ExtractValueErrorMessages {
            key_error_label: "dielectric L key",
            key_error_description: "invalid L key name",
            value_error_label: "dielectric L value",
            value_error_description: "invalid L attribute value",
        }
    )?;

    let material = {
        let m = extract_string(
            input,
            "M",
            ExtractValueErrorMessages {
                key_error_label: "dielectric M key",
                key_error_description: "invalid M key name",
                value_error_label: "dielectric M value",
                value_error_description: "invalid M attribute value",
            }
        ).unwrap();

        match m.as_str() {
            "Solder Mask" => LayerMaterial::SolderMask,
            "COPPER" => LayerMaterial::COPPER,
            "FR4" => LayerMaterial::FR4,
            "PR1080" => LayerMaterial::PR1080,
            "PR2116" => LayerMaterial::PR2116,
            other => LayerMaterial::UnknownMaterial(String::from(other)),
        }
    };

    close_row(input)?;

    Ok(Layer {
        layer_type: LayerType::Dielectric { dielectric_constant },
        thickness,
        layer_name: String::from(layer_name),
        material
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        r#"(DIELECTRIC T=0.0594488 C=4.5 L="F.Cu" M="FR4")"#,
        Layer {
            layer_type: LayerType::Dielectric { dielectric_constant: 4.5 },
            thickness: 0.0594488,
            layer_name: String::from("F.Cu"),
            material: LayerMaterial::FR4
        }
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.3 L="B.Mask" M="Solder Mask")"#,
        Layer {
            layer_type: LayerType::Dielectric { dielectric_constant: 3.3 },
            thickness: 0.000393701,
            layer_name: String::from("B.Mask"),
            material: LayerMaterial::SolderMask
        }
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.002992126 C=4.2 L="PR1080" M=PR1080)"#,
        Layer {
            layer_type: LayerType::Dielectric { dielectric_constant: 4.2 },
            thickness: 0.002992126,
            layer_name: String::from("PR1080"),
            material: LayerMaterial::PR1080
        }
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.00480315 C=4.0 L="PR2116" M=PR2116)"#,
        Layer {
            layer_type: LayerType::Dielectric { dielectric_constant: 4.0 },
            thickness: 0.00480315,
            layer_name: String::from("PR2116"),
            material: LayerMaterial::PR2116
        }
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.001 C=3.8 L="EXOTIC" M=CUSTOM_FOAM)"#,
        Layer {
            layer_type: LayerType::Dielectric { dielectric_constant: 3.8 },
            thickness: 0.001,
            layer_name: String::from("EXOTIC"),
            material: LayerMaterial::UnknownMaterial(String::from("CUSTOM_FOAM"))
        }
    )]
    fn parses_dielectric_layer(input: &str, expected: Layer) {
        let mut input = input;
        
        let dielectric_layer = dielectric.parse_next(&mut input).unwrap();

        assert_eq!(dielectric_layer, expected);
    }
}
