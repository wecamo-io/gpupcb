// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::combinator::{alt, repeat};
use winnow::{ModalResult, Parser};

use crate::model::{Arc, Net, Pin, Polygon, Polyvoid, Segment, Via};
use crate::parser::net::{pin::parse_pin_row, via::parse_via_row, seg::parse_seg_row, arc::parse_arc_row, polygon::{parse_polygon_block, parse_polyvoid_block}};
use crate::parser::utils::{ExtractEntryErrorMessages, extract_quoted_string_entry, open_block, close_block};

pub mod arc;
pub mod pin;
pub mod polygon;
pub mod seg;
pub mod via;

enum NetElement {
    Pin(Pin),
    Via(Via),
    Segment(Segment),
    Arc(Arc),
    Polygon(Polygon),
    Polyvoid(Polyvoid)
}

fn parse_net_element(input: &mut &str) -> ModalResult<NetElement> {
    alt((
        parse_pin_row.map(NetElement::Pin),
        parse_via_row.map(NetElement::Via),
        parse_seg_row.map(NetElement::Segment),
        parse_arc_row.map(NetElement::Arc),
        parse_polygon_block.map(NetElement::Polygon),
        parse_polyvoid_block.map(NetElement::Polyvoid),
    ))
    .parse_next(input)
}

pub fn parse_net_block(input: &mut &str) -> ModalResult<Net> {
    open_block(input, "")?;

    let name = extract_quoted_string_entry(
        input,
        "NET",
        ExtractEntryErrorMessages {
            key_error_label: "net NET key",
            key_error_description: "invalid NET key name",
            value_error_label: "net NET value",
            value_error_description: "invalid NET attribute value",
        },
    )?;

    let mut pins: Vec<Pin> = vec![];
    let mut vias: Vec<Via> = vec![];
    let mut segments: Vec<Segment> = vec![];
    let mut arcs: Vec<Arc> = vec![];
    let mut polygons: Vec<Polygon> = vec![];
    let mut polyvoids: Vec<Polyvoid> = vec![];

    let elements: Vec<NetElement> = repeat(0.., parse_net_element).parse_next(input)?;

    for element in elements {
        match element {
            NetElement::Pin(pin) => pins.push(pin),
            NetElement::Via(via) => vias.push(via),
            NetElement::Segment(segment) => segments.push(segment),
            NetElement::Arc(arc) => arcs.push(arc),
            NetElement::Polygon(polygon) => polygons.push(polygon),
            NetElement::Polyvoid(polyvoid) => polyvoids.push(polyvoid),
        }
    }

    close_block(input)?;

    Ok(Net {
        name: String::from(name),
        pins,
        vias,
        segments,
        arcs,
        polygons,
        polyvoids,
    })
}

#[cfg(test)]
mod net_block_parser_tests {
    use crate::model::PolygonType;

    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};

    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        Net {
        name: String::from("GND"),
        pins: vec![
            Pin {
                x: 7.9527559055,
                y: -4.5472440945,
                component_reference: String::from("D1.1"),
                padstack_id: 1,
            },
            Pin {
                x: 8.2677165354,
                y: -4.6062992126,
                component_reference: String::from("C3.2"),
                padstack_id: 2,
            },
        ],
        vias: vec![
            Via {
                x: 8.1917322835,
                y: -5.0476377953,
                padstack_id: 18,
            },
        ],
        segments: vec![
            Segment {
                x1: 8.1826771654,
                y1: -4.9956692913,
                x2: 8.1811023622,
                y2: -4.9940944882,
                width: 0.0196850394,
                layer: String::from("F.Cu"),
            },
        ],
        arcs: vec![
            Arc {
                x1: 8.2480314961,
                y1: -4.8854330709,
                x2: 8.3055118110,
                y2: -4.9429133858,
                xc: 0.5,
                yc: 0.5,
                radius: 1.0,
                width: 0.0118110236,
                layer: String::from("F.Cu"),
            },
        ],
        polyvoids: vec![
            Polyvoid {
                id: 1,
                x: 8.1712598425,
                y: -4.9842519685,
                outline: vec![
                    (8.1712598425, -4.9842519685),
                ],
            },
            Polyvoid {
                id: 1,
                x: 8.1203307480,
                y: -4.7089516535,
                outline: vec![
                    (8.1203307480, -4.7089516535),
                ],
            },
            Polyvoid {
                id: 1,
                x: 8.1200787402,
                y: -4.6751968504,
                outline: vec![
                    (8.1200787402, -4.6751968504),
                ],
            },
            Polyvoid {
                id: 1,
                x: 7.9108462205,
                y: -4.5709723228,
                outline: vec![
                    (7.9108462205, -4.5709723228),
                ],
            },
        ],
        polygons: vec![
            Polygon {
                polygon_type: PolygonType::Pour,
                layer: String::from("B.Cu"),
                id: 2,
                x: 8.3127283071,
                y: -4.5086490157,
                outline: vec![
                    (8.3127283071, -4.5086490157),
                ],
            },
        ],
    }
    )]
    fn parses_net_block(mut input: &str, expected: Net) {
        let net = parse_net_block(&mut input).unwrap();

        assert_eq!(net, expected);
    }

    // opening block
    #[test_case(
        "xyz{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("opening block"),
            StrContext::Expected(StrContextValue::Description("{"))
        )
    )]
    // NET entry
    #[test_case(
        "{xyzNET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("net NET key"),
            StrContext::Expected(StrContextValue::Description("invalid NET key name"))
        ); "before NET key"
    )]
    #[test_case(
        "{NETxyz=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("net NET key"),
            StrContext::Expected(StrContextValue::Description("invalid NET key name"))
        ); "after NET key"
    )]
    #[test_case(
        "{NET=GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("net NET value"),
            StrContext::Expected(StrContextValue::Description("invalid NET attribute value"))
        ); "before NET value"
    )]
    fn rejects_malformed_net_block(
        mut input: &str,
        expected_errors: (StrContext, StrContext)
    ) {
        let err = parse_net_block(&mut input).unwrap_err();

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
