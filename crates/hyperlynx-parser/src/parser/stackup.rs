use winnow::combinator::{alt, peek, repeat};
use winnow::token::literal;
use winnow::{ModalResult, Parser};
use winnow::error::{StrContext, StrContextValue};

use crate::model::{Layer, LayerMaterial, LayerType};
use crate::parser::utils::{ExtractValueErrorMessages, close_block, close_row, extract_float, extract_string, open_block, row_open};

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

fn signal(input: &mut &str) -> ModalResult<Layer> {
    row_open(input, "SIGNAL")?;

    let thickness = extract_float(
        input,
        "T",
        ExtractValueErrorMessages {
            key_error_label: "signal T key",
            key_error_description: "invalid T key name",
            value_error_label: "signal T value",
            value_error_description: "invalid T attribute value",
        }
    )?;

    let plating_thickness = extract_float(
        input,
        "P",
        ExtractValueErrorMessages {
            key_error_label: "signal P key",
            key_error_description: "invalid P key name",
            value_error_label: "signal P value",
            value_error_description: "invalid P attribute value",
        }
    )?;

    let resistivity = extract_float(
        input,
        "C",
        ExtractValueErrorMessages {
            key_error_label: "signal C key",
            key_error_description: "invalid C key name",
            value_error_label: "signal C value",
            value_error_description: "invalid C attribute value",
        }
    )?;

    let layer_name = extract_string(
        input,
        "L",
        ExtractValueErrorMessages {
            key_error_label: "signal L key",
            key_error_description: "invalid L key name",
            value_error_label: "signal L value",
            value_error_description: "invalid L attribute value",
        }
    )?;

    let material = {
        let m = extract_string(
            input,
            "M",
            ExtractValueErrorMessages {
                key_error_label: "signal M key",
                key_error_description: "invalid M key name",
                value_error_label: "signal M value",
                value_error_description: "invalid M attribute value",
            }
        ).unwrap();

        match m.as_str() {
            "COPPER" => LayerMaterial::COPPER,
            other => LayerMaterial::UnknownMaterial(String::from(format!("Material {other} is not valid for Signal layer"))),
        }
    };

    close_row(input)?;

    Ok(Layer {
        layer_type: LayerType::Signal { resistivity, plating_thickness },
        thickness,
        layer_name: String::from(layer_name),
        material
    })
}

// use conditions instead of winnow::alt due to wrong error context
// for any case other than SIGNAL or DIELECTRIC run alt()
fn parse_layer(input: &mut &str) -> ModalResult<Layer> {
    // see what is the layer's type with winnow::peek 
    // this does not mutate the imput
    let is_signal: ModalResult<&str> =
        peek(literal("(SIGNAL")).parse_next(input);

    if is_signal.is_ok() {
        signal.parse_next(input)
    } else {
        let is_dielectric: ModalResult<&str> =
            peek(literal("(DIELECTRIC")).parse_next(input);

        if is_dielectric.is_ok() {
            dielectric.parse_next(input)
        } else {
            alt((dielectric, signal)).parse_next(input)
        }
    }
}

pub fn parse_stackup_block(input: &mut &str) -> ModalResult<Vec<Layer>> {
    open_block(input, "STACKUP")?;

    let layers: Vec<Layer> = repeat(0.., parse_layer).parse_next(input)?;

    close_block(input)?;

    Ok(layers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use winnow::error::ErrMode;
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

    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724e-08 L="F.Cu" M=COPPER)"#,
        Layer {
            layer_type: LayerType::Signal { resistivity: 1.724e-08, plating_thickness: 0.0 },
            thickness: 0.00137795,
            layer_name: String::from("F.Cu"),
            material: LayerMaterial::COPPER
        }
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724e-08 L="B.Cu" M=COPPER)"#,
        Layer {
            layer_type: LayerType::Signal { resistivity: 1.724e-08, plating_thickness: 0.0 },
            thickness: 0.00137795,
            layer_name: String::from("B.Cu"),
            material: LayerMaterial::COPPER
        }
    )]
    #[test_case(
        r#"(SIGNAL T=0.000708661 P=1 C=2.1e-08 L="F.Cu" M="Solder Mask")"#,
        Layer {
            layer_type: LayerType::Signal { resistivity: 2.1e-08, plating_thickness: 1.0 },
            thickness: 0.000708661,
            layer_name: String::from("F.Cu"),
            material: LayerMaterial::UnknownMaterial(String::from("Material Solder Mask is not valid for Signal layer"))
        }
    )]
    fn parses_signal_layer(input: &str, expected: Layer) {
        let mut input = input;

        let signal_layer = signal.parse_next(&mut input).unwrap();

        assert_eq!(signal_layer, expected);
    }

    #[test]
    fn parses_stackup_block() {
        let mut input = r#"{STACKUP
  (DIELECTRIC T=0.000393701 C=3.3 L="F.Mask" M="Solder Mask")
  (SIGNAL T=0.00137795 P=0 C=1.724e-08 L="F.Cu" M=COPPER)
  (DIELECTRIC T=0.0594488 C=4.5 L="DE_F.Cu" M="FR4")
  (SIGNAL T=0.00137795 P=0 C=1.724e-08 L="B.Cu" M=COPPER)
  (DIELECTRIC T=0.000393701 C=3.3 L="B.Mask" M="Solder Mask")
} some_further_input"#;
        let expected_signals_length: usize = 2;
        let expected_dielectrics_length: usize = 3;
        let expected_further_input = "some_further_input";

        let layers = parse_stackup_block(&mut input).unwrap();

        let (signals, dielectrics): (Vec<_>, Vec<_>) = layers
            .into_iter()
            .partition(
                |l| matches!(l.layer_type, LayerType::Signal { resistivity: _, plating_thickness: _ })
            );

        assert_eq!(expected_signals_length, signals.len());
        assert_eq!(expected_dielectrics_length, dielectrics.len());

        // leaves further input
        assert_eq!(input, expected_further_input);
    }

    #[test_case(
        r#"xyz(DIELECTRIC T=0.000393701 C=3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("opening row"),
        StrContext::Expected(StrContextValue::Description("("))
    )]
    // row name malformations
    #[test_case(
        r#"(xyzDIELECTRIC T=0.000393701 C=3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("row name"),
        StrContext::Expected(StrContextValue::Description("SIGNAL"))
    )]
    #[test_case(
        r#"(DIELExyzCTRIC T=0.000393701 C=3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("row name"),
        StrContext::Expected(StrContextValue::Description("SIGNAL"))
    )]
    #[test_case(
        r#"(DIELECTRICxyz T=0.000393701 C=3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("block name boundary"),
        StrContext::Expected(StrContextValue::Description("valid character after row name"))
    )]
    #[test_case(
        r#"(xyzSIGNAL T=0.00137795 P=0 C=1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("row name"),
        StrContext::Expected(StrContextValue::Description("SIGNAL"))
    )]
    #[test_case(
        r#"(SIGxyzNAL T=0.00137795 P=0 C=1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("row name"),
        StrContext::Expected(StrContextValue::Description("SIGNAL"))
    )]
    #[test_case(
        r#"(SIGNALxyz T=0.00137795 P=0 C=1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("block name boundary"),
        StrContext::Expected(StrContextValue::Description("valid character after row name"))
    )]
    // T attribute malformation
    #[test_case(
        r#"(DIELECTRIC xyzT=0.000393701 C=3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("dielectric T key"),
        StrContext::Expected(StrContextValue::Description("invalid T key name"))
    )]
    #[test_case(
        r#"(DIELECTRIC Txyz=0.000393701 C=3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("dielectric T key"),
        StrContext::Expected(StrContextValue::Description("invalid T key name"))
    )]
    #[test_case(
        r#"(SIGNAL xyzT=0.00137795 P=0 C=1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("signal T key"),
        StrContext::Expected(StrContextValue::Description("invalid T key name"))
    )]
    #[test_case(
        r#"(SIGNAL Txyz=0.00137795 P=0 C=1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("signal T key"),
        StrContext::Expected(StrContextValue::Description("invalid T key name"))
    )]
    fn rejects_malformed_layer_row(
        input: &str,
        expected_label: StrContext,
        expected_description: StrContext
    ) {
        let mut input = input;
        let err = parse_layer(&mut input).unwrap_err();

        let errors = match err {
            ErrMode::Backtrack(ref err) | ErrMode::Cut(ref err) => {
                let mut iter = err.context().into_iter();
                let errors = (iter.next().unwrap(), iter.next().unwrap());
                
                errors                
            }
            ErrMode::Incomplete(_) => todo!(),
        };

        println!("{:?}", errors);
        assert_eq!(*errors.0, expected_label);
        assert_eq!(*errors.1, expected_description);
    }
}
