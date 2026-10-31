// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

#[cfg(test)]
mod kicad_export_tests {
    use hyperlynx_parser::{
        model::{
            Arc, Board, Device, Layer, LayerMaterial, LayerType, Pad, PadShape, PadType, Padstack,
            PerimeterSegment, Pin, Polygon, PolygonType, Polyvoid, Segment, Units, Via,
        },
        parser::parse_hyperlynx,
    };

    #[test]
    fn parses_kicad_export() {
        let input = include_str!("fixtures/kicad.hyp");

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
        assert_eq!(outline.len(), 2);

        assert_eq!(
            outline[0],
            PerimeterSegment {
                x1: 1.0,
                y1: -2.0,
                x2: 3.0,
                y2: -4.0,
            }
        );

        assert_eq!(
            outline[1],
            PerimeterSegment {
                x1: 1.0,
                y1: -2.0,
                x2: 3.0,
                y2: -4.0,
            }
        );

        // STACKUP
        assert_eq!(stackup.len(), 3);

        assert_eq!(
            stackup[0],
            Layer {
                layer_type: LayerType::Dielectric {
                    dielectric_constant: 3.3,
                },
                thickness: 0.000393701,
                layer_name: String::from("F.Cu"),
                material: LayerMaterial::SolderMask,
            }
        );

        assert_eq!(
            stackup[1],
            Layer {
                layer_type: LayerType::Signal {
                    resistivity: 1.724e-08,
                    plating_thickness: 0.0,
                },
                thickness: 0.00137795,
                layer_name: String::from("F.Cu"),
                material: LayerMaterial::COPPER,
            }
        );

        assert_eq!(
            stackup[2],
            Layer {
                layer_type: LayerType::Dielectric {
                    dielectric_constant: 4.5,
                },
                thickness: 0.0594488,
                layer_name: String::from("F.Cu"),
                material: LayerMaterial::FR4,
            }
        );

        // DEVICES
        assert_eq!(devices.len(), 2);

        assert_eq!(
            devices[0],
            Device {
                reference: String::from("R3"),
                layer: String::from("F.Cu"),
            }
        );

        assert_eq!(
            devices[1],
            Device {
                reference: String::from("SW1"),
                layer: String::from("F.Cu"),
            }
        );

        // PADSTACKS
        assert_eq!(padstacks.len(), 2);

        assert_eq!(
            padstacks[0],
            Padstack {
                id: 0,
                drill: 0.0,
                pads: vec![Pad {
                    layer: String::from("F.Cu"),
                    shape: PadShape::Rectangle,
                    size_x: 0.05,
                    size_y: 0.06,
                    angle: 90.0,
                    r#type: PadType::M,
                }],
            }
        );

        assert_eq!(
            padstacks[1],
            Padstack {
                id: 16,
                drill: 0.007,
                pads: vec![Pad {
                    layer: String::from("MDEF"),
                    shape: PadShape::Oval,
                    size_x: 0.02,
                    size_y: 0.03,
                    angle: 270.0,
                    r#type: PadType::M,
                }],
            }
        );

        // NETS
        assert_eq!(nets.len(), 2);

        // NET with "GND" name
        let gnd = &nets[0];

        assert_eq!(gnd.name, "GND");

        // PINS
        assert_eq!(gnd.pins.len(), 2);

        assert_eq!(
            gnd.pins[0],
            Pin {
                x: 1.0,
                y: -2.0,
                component_reference: String::from("R3.1"),
                padstack_id: 1,
            }
        );

        assert_eq!(
            gnd.pins[1],
            Pin {
                x: 1.0,
                y: -2.0,
                component_reference: String::from("C3.2"),
                padstack_id: 2,
            }
        );

        // SEGMENTS
        assert_eq!(gnd.segments.len(), 1);

        assert_eq!(
            gnd.segments[0],
            Segment {
                x1: 1.0,
                y1: -2.0,
                x2: 3.0,
                y2: -4.0,
                width: 0.02,
                layer: String::from("F.Cu"),
            }
        );

        // ARCS
        assert_eq!(gnd.arcs.len(), 1);

        assert_eq!(
            gnd.arcs[0],
            Arc {
                x1: 1.0,
                y1: -2.0,
                x2: 3.0,
                y2: -4.0,
                xc: 5.0,
                yc: -6.0,
                radius: 0.016,
                width: 0.0049,
                layer: String::from("B.Cu"),
            }
        );

        // VIAS
        assert_eq!(gnd.vias.len(), 1);

        assert_eq!(
            gnd.vias[0],
            Via {
                x: 1.0,
                y: 2.0,
                padstack_id: 12,
            }
        );

        // POLYGONS
        assert_eq!(gnd.polygons.len(), 2);

        assert_eq!(
            gnd.polygons[0],
            Polygon {
                polygon_type: PolygonType::Pour,
                layer: String::from("F.Cu"),
                id: 1,
                x: 1.0,
                y: -2.0,
                outline: vec![(1.0, -2.0), (1.0, -2.0)],
            }
        );

        assert_eq!(
            gnd.polygons[1],
            Polygon {
                polygon_type: PolygonType::Pour,
                layer: String::from("B.Cu"),
                id: 2,
                x: 1.0,
                y: -2.0,
                outline: vec![(1.0, -2.0), (1.0, -2.0)],
            }
        );

        // POLYVOIDS
        assert_eq!(gnd.polyvoids.len(), 2);

        assert_eq!(
            gnd.polyvoids[0],
            Polyvoid {
                id: 1,
                x: 1.0,
                y: -2.0,
                outline: vec![(1.0, -2.0)],
            }
        );

        assert_eq!(
            gnd.polyvoids[1],
            Polyvoid {
                id: 1,
                x: 1.0,
                y: -2.0,
                outline: vec![(1.0, -2.0)],
            }
        );

        // NET with "unconnected" name
        let unconnected = &nets[1];

        assert_eq!(unconnected.name, "unconnected");

        assert_eq!(unconnected.pins.len(), 1);

        assert_eq!(
            unconnected.pins[0],
            Pin {
                x: 1.0,
                y: 2.0,
                component_reference: String::from("R3.1"),
                padstack_id: 12,
            }
        );

        assert!(unconnected.vias.is_empty());
        assert!(unconnected.segments.is_empty());
        assert!(unconnected.arcs.is_empty());
        assert!(unconnected.polygons.is_empty());
        assert!(unconnected.polyvoids.is_empty());
    }
}
