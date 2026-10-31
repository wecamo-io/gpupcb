// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

#[cfg(test)]
mod altium_export_tests {
    use hyperlynx_parser::{
        model::{
            Arc, Board, Device, Layer, LayerMaterial, LayerType, Pad, PadShape, PadType, Padstack,
            Pin, Polygon, PolygonType, Polyvoid, Segment, Units, Via,
        },
        parser::parse_hyperlynx,
    };

    #[test]
    fn parses_altium_export() {
        let input = include_str!("fixtures/altium designer.HYP");

        let Board {
            version,
            units,
            outline,
            stackup,
            devices,
            padstacks,
            nets,
        } = parse_hyperlynx(input).unwrap();

        // VERSION
        assert_eq!(version, "2.14");

        // UNITS
        assert_eq!(units, Units::English);

        // BOARD
        assert!(outline.is_empty());

        // STACKUP
        assert_eq!(stackup.len(), 3);

        assert_eq!(
            stackup[0],
            Layer {
                layer_type: LayerType::Signal {
                    resistivity: 1.724e-8,
                    plating_thickness: 0.0,
                },
                thickness: 0.003937,
                layer_name: String::from("Top Layer"),
                material: LayerMaterial::COPPER,
            }
        );

        assert_eq!(
            stackup[1],
            Layer {
                layer_type: LayerType::Dielectric {
                    dielectric_constant: 4.8,
                },
                thickness: 0.035433,
                layer_name: String::from("Top Layer"),
                material: LayerMaterial::FR4,
            }
        );

        assert_eq!(
            stackup[2],
            Layer {
                layer_type: LayerType::Signal {
                    resistivity: 1.724e-8,
                    plating_thickness: 0.0,
                },
                thickness: 0.003937,
                layer_name: String::from("Bottom Layer"),
                material: LayerMaterial::COPPER,
            }
        );

        // DEVICES
        assert_eq!(devices.len(), 2);

        assert_eq!(
            devices[0],
            Device {
                reference: String::from("U2"),
                layer: String::from("Top Layer"),
            }
        );

        assert_eq!(
            devices[1],
            Device {
                reference: String::from(" "),
                layer: String::from("Top Layer"),
            }
        );

        // PADSTACKS
        assert_eq!(padstacks.len(), 3);

        assert_eq!(
            padstacks[0],
            Padstack {
                id: 0,
                drill: 0.0,
                pads: vec![Pad {
                    layer: String::from("Top Layer"),
                    shape: PadShape::Roundrect,
                    size_x: 0.06,
                    size_y: 0.06,
                    angle: 0.0,
                    r#type: PadType::M,
                }],
            }
        );

        assert_eq!(
            padstacks[1],
            Padstack {
                id: 1,
                drill: 0.0,
                pads: vec![Pad {
                    layer: String::from("Top Layer"),
                    shape: PadShape::Rectangle,
                    size_x: 0.06,
                    size_y: 0.04,
                    angle: 270.0,
                    r#type: PadType::M,
                }],
            }
        );

        assert_eq!(
            padstacks[2],
            Padstack {
                id: 2,
                drill: 0.022,
                pads: vec![
                    Pad {
                        layer: String::from("MDEF"),
                        shape: PadShape::Oval,
                        size_x: 0.04,
                        size_y: 0.04,
                        angle: 180.0,
                        r#type: PadType::M,
                    },
                    Pad {
                        layer: String::from("ADEF"),
                        shape: PadShape::Oval,
                        size_x: 0.06,
                        size_y: 0.06,
                        angle: 90.0,
                        r#type: PadType::A,
                    },
                ],
            }
        );

        // NETS
        assert_eq!(nets.len(), 1);

        let vcc = &nets[0];

        assert_eq!(vcc.name, "VCC");

        // PINS
        assert_eq!(vcc.pins.len(), 3);

        assert_eq!(
            vcc.pins[0],
            Pin {
                x: 1.0,
                y: 2.0,
                component_reference: String::from("R1.1"),
                padstack_id: 4,
            }
        );

        assert_eq!(
            vcc.pins[1],
            Pin {
                x: 1.0,
                y: 2.0,
                component_reference: String::from(".1"),
                padstack_id: 4,
            }
        );

        assert_eq!(
            vcc.pins[2],
            Pin {
                x: 1.0,
                y: 2.0,
                component_reference: String::from("U1.16"),
                padstack_id: 8,
            }
        );

        // SEGMENTS
        assert_eq!(vcc.segments.len(), 1);

        assert_eq!(
            vcc.segments[0],
            Segment {
                x1: 1.0,
                y1: -2.0,
                x2: 3.0,
                y2: -4.0,
                width: 0.011,
                layer: String::from("Bottom Layer"),
            }
        );

        // ARCS
        assert_eq!(vcc.arcs.len(), 1);

        assert_eq!(
            vcc.arcs[0],
            Arc {
                x1: 1.0,
                y1: -2.0,
                x2: 3.0,
                y2: -4.0,
                xc: 5.0,
                yc: -6.0,
                radius: 0.016,
                width: 0.0049,
                layer: String::from("Bottom Layer"),
            }
        );

        // VIAS
        assert_eq!(vcc.vias.len(), 1);

        assert_eq!(
            vcc.vias[0],
            Via {
                x: 1.0,
                y: 2.0,
                padstack_id: 12,
            }
        );

        // POLYGONS
        assert_eq!(vcc.polygons.len(), 1);

        assert_eq!(
            vcc.polygons[0],
            Polygon {
                polygon_type: PolygonType::Pour,
                layer: String::from("Bottom Layer"),
                id: 1,
                x: 1.0,
                y: 1.0,
                outline: vec![(1.0, 2.0), (1.0, 2.0),],
            }
        );

        // POLYVOIDS
        assert_eq!(vcc.polyvoids.len(), 2);

        assert_eq!(
            vcc.polyvoids[0],
            Polyvoid {
                id: 1,
                x: 1.0,
                y: 2.0,
                outline: vec![(1.0, 2.0)],
            }
        );

        assert_eq!(
            vcc.polyvoids[1],
            Polyvoid {
                id: 1,
                x: 1.0,
                y: 2.0,
                outline: vec![(1.0, 2.0), (1.0, 2.0),],
            }
        );
    }
}
