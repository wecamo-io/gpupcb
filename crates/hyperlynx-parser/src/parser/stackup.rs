// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::multispace0;
use winnow::combinator::{alt, cut_err, peek, repeat};
use winnow::error::{ContextError, ErrMode, StrContext, StrContextValue};
use winnow::token::literal;
use winnow::{ModalResult, Parser};

use crate::model::{Layer, LayerMaterial, LayerType};
use crate::parser::utils::{
    close_block, close_row, extract_float_entry, extract_quoted_string_entry,
    extract_unquoted_string_entry, open_block, row_open, ExtractEntryErrorMessages,
};

fn parse_m_attribute_quoted(input: &mut &str, layer: &str) -> ModalResult<String> {
    extract_quoted_string_entry(
        input,
        "M",
        ExtractEntryErrorMessages {
            key_error_label: " M key",
            key_error_description: "invalid M key name",
            value_error_label: "dielectric M value",
            value_error_description: "invalid M attribute value",
        },
    )
}

fn parse_m_attribute_unquoted(input: &mut &str) -> ModalResult<String> {
    extract_unquoted_string_entry(
        input,
        "M",
        ')',
        ExtractEntryErrorMessages {
            key_error_label: "dielectric M key",
            key_error_description: "invalid M key name",
            value_error_label: "dielectric M value",
            value_error_description: "invalid M attribute value",
        },
    )
}

fn dielectric(input: &mut &str) -> ModalResult<Layer> {
    match row_open(input, "DIELECTRIC") {
        Ok(_) => {}
        Err(_) => {
            let mut err = ContextError::new();
            err.push(StrContext::Label("layer row name"));
            err.push(StrContext::Expected(StrContextValue::Description(
                "dielectric/signal key name",
            )));
            return Err(ErrMode::Cut(err));
        }
    };

    let thickness = extract_float_entry(
        input,
        "T",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "dielectric T key",
            key_error_description: "invalid T key name",
            value_error_label: "dielectric T value",
            value_error_description: "invalid T attribute value",
        },
    )?;

    let dielectric_constant = extract_float_entry(
        input,
        "C",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "dielectric C key",
            key_error_description: "invalid C key name",
            value_error_label: "dielectric C value",
            value_error_description: "invalid C attribute value",
        },
    )?;

    let layer_name = extract_quoted_string_entry(
        input,
        "L",
        ExtractEntryErrorMessages {
            key_error_label: "dielectric L key",
            key_error_description: "invalid L key name",
            value_error_label: "dielectric L value",
            value_error_description: "invalid L attribute value",
        },
    )?;

    let material = {
        let m = alt((
            |input: &mut &str| {
                extract_quoted_string_entry(
                    input,
                    "M",
                    ExtractEntryErrorMessages {
                        key_error_label: "dielectric M key",
                        key_error_description: "invalid M key name",
                        value_error_label: "dielectric M value",
                        value_error_description: "invalid M attribute value",
                    },
                )
            },
            |input: &mut &str| {
                extract_unquoted_string_entry(
                    input,
                    "M",
                    ')',
                    ExtractEntryErrorMessages {
                        key_error_label: "dielectric M key",
                        key_error_description: "invalid M key name",
                        value_error_label: "dielectric M value",
                        value_error_description: "invalid M attribute value",
                    },
                )
            },
        ))
        .parse_next(input)?;

        match m.as_str() {
            "Solder Mask" => LayerMaterial::SolderMask,
            "COPPER" => LayerMaterial::COPPER,
            "FR4" => LayerMaterial::FR4,
            "PR1080" => LayerMaterial::PR1080,
            "PR2116" => LayerMaterial::PR2116,
            _ => {
                let mut err = ContextError::new();
                err.push(StrContext::Label("dielectric M value"));
                err.push(StrContext::Expected(StrContextValue::Description(
                    "invalid M attribute value",
                )));

                Err(ErrMode::Backtrack(err))
            }?,
        }
    };

    close_row(input)?;

    Ok(Layer {
        layer_type: LayerType::Dielectric {
            dielectric_constant,
        },
        thickness,
        layer_name: String::from(layer_name),
        material,
    })
}

fn signal(input: &mut &str) -> ModalResult<Layer> {
    match row_open(input, "SIGNAL") {
        Ok(_) => {}
        Err(_) => {
            let mut err = ContextError::new();
            err.push(StrContext::Label("layer row name"));
            err.push(StrContext::Expected(StrContextValue::Description(
                "dielectric/signal key name",
            )));
            return Err(ErrMode::Cut(err));
        }
    };

    let thickness = extract_float_entry(
        input,
        "T",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "signal T key",
            key_error_description: "invalid T key name",
            value_error_label: "signal T value",
            value_error_description: "invalid T attribute value",
        },
    )?;

    let plating_thickness = extract_float_entry(
        input,
        "P",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "signal P key",
            key_error_description: "invalid P key name",
            value_error_label: "signal P value",
            value_error_description: "invalid P attribute value",
        },
    )?;

    let resistivity = extract_float_entry(
        input,
        "C",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "signal C key",
            key_error_description: "invalid C key name",
            value_error_label: "signal C value",
            value_error_description: "invalid C attribute value",
        },
    )?;

    let layer_name = extract_quoted_string_entry(
        input,
        "L",
        ExtractEntryErrorMessages {
            key_error_label: "signal L key",
            key_error_description: "invalid L key name",
            value_error_label: "signal L value",
            value_error_description: "invalid L attribute value",
        },
    )?;

    let material = {
        let m = alt((
            |input: &mut &str| {
                extract_quoted_string_entry(
                    input,
                    "M",
                    ExtractEntryErrorMessages {
                        key_error_label: "signal M key",
                        key_error_description: "invalid M key name",
                        value_error_label: "signal M value",
                        value_error_description: "invalid M attribute value",
                    },
                )
            },
            |input: &mut &str| {
                extract_unquoted_string_entry(
                    input,
                    "M",
                    ')',
                    ExtractEntryErrorMessages {
                        key_error_label: "signal M key",
                        key_error_description: "invalid M key name",
                        value_error_label: "signal M value",
                        value_error_description: "invalid M attribute value",
                    },
                )
            },
        ))
        .parse_next(input)?;

        match m.as_str() {
            "COPPER" => LayerMaterial::COPPER,
            _ => {
                let mut err = ContextError::new();
                err.push(StrContext::Label("signal M value"));
                err.push(StrContext::Expected(StrContextValue::Description(
                    "invalid M attribute value",
                )));

                Err(ErrMode::Backtrack(err))
            }?,
        }
    };

    close_row(input)?;

    Ok(Layer {
        layer_type: LayerType::Signal {
            resistivity,
            plating_thickness,
        },
        thickness,
        layer_name: String::from(layer_name),
        material,
    })
}

// use conditions instead of winnow::alt due to wrong error context
// for any case other than SIGNAL or DIELECTRIC run alt()
fn parse_layer(input: &mut &str) -> ModalResult<Layer> {
    // see what is the layer's type with winnow::peek
    // this does not mutate the imput
    let is_signal: ModalResult<&str> = peek(literal("(SIGNAL")).parse_next(input);

    if is_signal.is_ok() {
        signal.parse_next(input)
    } else {
        let is_dielectric: ModalResult<&str> = peek(literal("(DIELECTRIC")).parse_next(input);

        if is_dielectric.is_ok() {
            dielectric.parse_next(input)
        } else {
            alt((dielectric, signal)).parse_next(input)
        }
    }
}

fn parse_layer_row_with_err(input: &mut &str) -> ModalResult<Layer> {
    peek((multispace0, "(")).parse_next(input)?;

    cut_err(parse_layer).parse_next(input)
}

pub fn parse_stackup_block(input: &mut &str) -> ModalResult<Vec<Layer>> {
    open_block(input, "STACKUP")?;

    let layers: Vec<Layer> = repeat(0.., parse_layer_row_with_err).parse_next(input)?;

    close_block(input)?;

    Ok(layers)
}

#[cfg(test)]
mod stackup_block_parser_tests {
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

        let (signals, dielectrics): (Vec<_>, Vec<_>) = layers.into_iter().partition(|l| {
            matches!(
                l.layer_type,
                LayerType::Signal {
                    resistivity: _,
                    plating_thickness: _
                }
            )
        });

        assert_eq!(expected_signals_length, signals.len());
        assert_eq!(expected_dielectrics_length, dielectrics.len());

        // leaves further input
        assert_eq!(input, expected_further_input);
    }

    #[test_case(
        r#"xyz(DIELECTRIC T=0.000393701 C=3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("layer row name"),
        StrContext::Expected(StrContextValue::Description("dielectric/signal key name"))
    )]
    // row name malformations
    #[test_case(
        r#"(xyzDIELECTRIC T=0.000393701 C=3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("layer row name"),
        StrContext::Expected(StrContextValue::Description("dielectric/signal key name"))
    )]
    #[test_case(
        r#"(DIELExyzCTRIC T=0.000393701 C=3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("layer row name"),
        StrContext::Expected(StrContextValue::Description("dielectric/signal key name"))
    )]
    #[test_case(
        r#"(DIELECTRICxyz T=0.000393701 C=3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("layer row name"),
        StrContext::Expected(StrContextValue::Description("dielectric/signal key name"))
    )]
    #[test_case(
        r#"(xyzSIGNAL T=0.00137795 P=0 C=1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("layer row name"),
        StrContext::Expected(StrContextValue::Description("dielectric/signal key name"))
    )]
    #[test_case(
        r#"(SIGxyzNAL T=0.00137795 P=0 C=1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("layer row name"),
        StrContext::Expected(StrContextValue::Description("dielectric/signal key name"))
    )]
    #[test_case(
        r#"(SIGNALxyz T=0.00137795 P=0 C=1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("layer row name"),
        StrContext::Expected(StrContextValue::Description("dielectric/signal key name"))
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
    // C attribute malformation
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 xyzC=3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("dielectric C key"),
        StrContext::Expected(StrContextValue::Description("invalid C key name"))
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 Cxyz=3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("dielectric C key"),
        StrContext::Expected(StrContextValue::Description("invalid C key name"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 xyzC=1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("signal C key"),
        StrContext::Expected(StrContextValue::Description("invalid C key name"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 Cxyz=1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("signal C key"),
        StrContext::Expected(StrContextValue::Description("invalid C key name"))
    )]
    // L attribute malformation
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.3 xyzL="F.Mask" M="Solder Mask")"#,
        StrContext::Label("dielectric L key"),
        StrContext::Expected(StrContextValue::Description("invalid L key name"))
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.3 Lxyz="F.Mask" M="Solder Mask")"#,
        StrContext::Label("dielectric L key"),
        StrContext::Expected(StrContextValue::Description("invalid L key name"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724e-08 xyzL="F.Cu" M=COPPER)"#,
        StrContext::Label("signal L key"),
        StrContext::Expected(StrContextValue::Description("invalid L key name"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724e-08 Lxyz="F.Cu" M=COPPER)"#,
        StrContext::Label("signal L key"),
        StrContext::Expected(StrContextValue::Description("invalid L key name"))
    )]
    // M attribute malformation
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.3 L="F.Mask" xyzM="Solder Mask")"#,
        StrContext::Label("dielectric M key"),
        StrContext::Expected(StrContextValue::Description("invalid M key name"))
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.3 L="F.Mask" Mxyz="Solder Mask")"#,
        StrContext::Label("dielectric M key"),
        StrContext::Expected(StrContextValue::Description("invalid M key name"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724e-08 L="F.Cu" xyzM=COPPER)"#,
        StrContext::Label("signal M key"),
        StrContext::Expected(StrContextValue::Description("invalid M key name"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724e-08 L="F.Cu" Mxyz=COPPER)"#,
        StrContext::Label("signal M key"),
        StrContext::Expected(StrContextValue::Description("invalid M key name"))
    )]
    // P attribute malformation
    #[test_case(
        r#"(SIGNAL T=0.00137795 xyzP=0 C=1.724e-08 L="F.Cu" xyzM=COPPER)"#,
        StrContext::Label("signal P key"),
        StrContext::Expected(StrContextValue::Description("invalid P key name"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 Pxyz=0 C=1.724e-08 L="F.Cu" Mxyz=COPPER)"#,
        StrContext::Label("signal P key"),
        StrContext::Expected(StrContextValue::Description("invalid P key name"))
    )]
    // T attribute value malformation
    #[test_case(
        r#"(DIELECTRIC T=xyz0.000393701 C=3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("dielectric T value"),
        StrContext::Expected(StrContextValue::Description("invalid T attribute value"))
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.000xyz393701 C=3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("dielectric T value"),
        StrContext::Expected(StrContextValue::Description("invalid T attribute value"))
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.000393701xyz C=3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("dielectric T value"),
        StrContext::Expected(StrContextValue::Description("invalid T attribute value"))
    )]
    #[test_case(
        r#"(SIGNAL T=xyz0.00137795 P=0 C=1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("signal T value"),
        StrContext::Expected(StrContextValue::Description("invalid T attribute value"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.001xyz37795 P=0 C=1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("signal T value"),
        StrContext::Expected(StrContextValue::Description("invalid T attribute value"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795xyz P=0 C=1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("signal T value"),
        StrContext::Expected(StrContextValue::Description("invalid T attribute value"))
    )]
    // C attribute value malformation
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=xyz3.3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("dielectric C value"),
        StrContext::Expected(StrContextValue::Description("invalid C attribute value"))
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.xyz3 L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("dielectric C value"),
        StrContext::Expected(StrContextValue::Description("invalid C attribute value"))
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.3xyz L="F.Mask" M="Solder Mask")"#,
        StrContext::Label("dielectric C value"),
        StrContext::Expected(StrContextValue::Description("invalid C attribute value"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=xyz1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("signal C value"),
        StrContext::Expected(StrContextValue::Description("invalid C attribute value"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724xyz e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("signal C value"),
        StrContext::Expected(StrContextValue::Description("invalid C attribute value"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724e-08xyz L="F.Cu" M=COPPER)"#,
        StrContext::Label("signal C value"),
        StrContext::Expected(StrContextValue::Description("invalid C attribute value"))
    )]
    // L attribute value malformation
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.3 L=xyz"F.Mask" M="Solder Mask")"#,
        StrContext::Label("dielectric L value"),
        StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.3 L=F.Mask" M="Solder Mask")"#,
        StrContext::Label("dielectric L value"),
        StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.3 L="F.Mask M="Solder Mask")"#,
        StrContext::Label("dielectric M key"),
        StrContext::Expected(StrContextValue::Description("invalid M key name"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724e-08 L=xyz"F.Cu" M=COPPER)"#,
        StrContext::Label("signal L value"),
        StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724e-08 L=F.Cu" M=COPPER)"#,
        StrContext::Label("signal L value"),
        StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724e-08 L="F.Cu M=COPPER)"#,
        StrContext::Label("signal L value"),
        StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
    )]
    // M attribute value malformation
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.3 L="F.Mask" M=xyz"Solder Mask")"#,
        StrContext::Label("dielectric M value"),
        StrContext::Expected(StrContextValue::Description("invalid M attribute value"))
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.3 L="F.Mask" M="Solder xyzMask")"#,
        StrContext::Label("dielectric M value"),
        StrContext::Expected(StrContextValue::Description("invalid M attribute value"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724e-08 L="F.Cu" M=xyzCOPPER)"#,
        StrContext::Label("signal M value"),
        StrContext::Expected(StrContextValue::Description("invalid M attribute value"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724e-08 L="F.Cu" M=COxyzPPER)"#,
        StrContext::Label("signal M value"),
        StrContext::Expected(StrContextValue::Description("invalid M attribute value"))
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.3 L="F.Mask" M=xyz"Solder Mask")"#,
        StrContext::Label("dielectric M value"),
        StrContext::Expected(StrContextValue::Description("invalid M attribute value"))
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.3 L="F.Mask" M=xyzSolder Mask")"#,
        StrContext::Label("dielectric M value"),
        StrContext::Expected(StrContextValue::Description("invalid M attribute value"))
    )]
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.3 L="F.Mask" M="Solder Mask)"#,
        StrContext::Label("dielectric M value"),
        StrContext::Expected(StrContextValue::Description("invalid M attribute value"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724e-08 L="F.Cu" M="COPPER)"#,
        StrContext::Label("signal M value"),
        StrContext::Expected(StrContextValue::Description("invalid M attribute value"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724e-08 L="F.Cu" M=COPPER")"#,
        StrContext::Label("signal M value"),
        StrContext::Expected(StrContextValue::Description("invalid M attribute value"))
    )]
    // P attribute value malformation (SIGNAL only)
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=xyz0 C=1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("signal P value"),
        StrContext::Expected(StrContextValue::Description("invalid P attribute value"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0xyz C=1.724e-08 L="F.Cu" M=COPPER)"#,
        StrContext::Label("signal P value"),
        StrContext::Expected(StrContextValue::Description("invalid P attribute value"))
    )]
    // closing parenthesis ')'
    #[test_case(
        r#"(DIELECTRIC T=0.000393701 C=3.3 L="F.Mask" M="Solder Mask""#,
        StrContext::Label("closing row"),
        StrContext::Expected(StrContextValue::Description(")"))
    )]
    #[test_case(
        r#"(SIGNAL T=0.00137795 P=0 C=1.724e-08 L="F.Cu" M=COPPER"#,
        StrContext::Label("closing row"),
        StrContext::Expected(StrContextValue::Description(")"))
    )]
    fn rejects_malformed_layer_row(
        input: &str,
        expected_label: StrContext,
        expected_description: StrContext,
    ) {
        let mut input = input;
        let err = parse_layer.parse_next(&mut input).unwrap_err();

        let errors = match err {
            ErrMode::Backtrack(ref err) | ErrMode::Cut(ref err) => {
                let mut iter = err.context().into_iter();
                let errors = (iter.next().unwrap(), iter.next().unwrap());

                errors
            }
            ErrMode::Incomplete(_) => todo!(),
        };

        assert_eq!(*errors.0, expected_label);
        assert_eq!(*errors.1, expected_description);
    }
}
