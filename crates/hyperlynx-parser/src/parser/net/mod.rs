// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::multispace0;
use winnow::combinator::{alt, cut_err, peek, repeat};
use winnow::error::{ContextError, ErrMode};
use winnow::prelude::*;
use winnow::token::literal;

use crate::model::{Arc, Net, Pin, Polygon, Polyvoid, Segment, Via};
use crate::parser::net::{
    arc::parse_arc_row_with_err,
    pin::parse_pin_row_with_err,
    polygon::{parse_polygon_block_with_err, parse_polyvoid_block_with_err},
    seg::parse_seg_row_with_err,
    via::parse_via_row_with_err,
};
use crate::parser::utils::{
    ExtractEntryErrorMessages, build_parser_cut_error, close_block, extract_string_entry,
    open_block, skip_ignored,
};

pub mod arc;
pub mod pin;
pub mod polygon;
pub mod seg;
pub mod via;

#[derive(Debug)]
enum NetRowElement {
    Pin(Pin),
    Via(Via),
    Segment(Segment),
    Arc(Arc),
}

#[derive(Debug)]
enum NetBlockElement {
    Polygon(Polygon),
    Polyvoid(Polyvoid),
}

fn is_net_element<'a>(input: &'a mut &str, lit: &str) -> Result<(&'a str, &'a str), ContextError> {
    peek((multispace0::<_, ContextError>, literal(lit))).parse_next(input)
}

// refactor candidate
fn parse_net_row_element(input: &mut &str) -> ModalResult<NetRowElement> {
    let is_pin = is_net_element(input, "(PIN");

    if is_pin.is_ok() {
        cut_err(parse_pin_row_with_err)
            .map(NetRowElement::Pin)
            .parse_next(input)
    } else {
        let is_via = is_net_element(input, "(VIA");

        if is_via.is_ok() {
            cut_err(parse_via_row_with_err)
                .map(NetRowElement::Via)
                .parse_next(input)
        } else {
            let is_seg = is_net_element(input, "(SEG");

            if is_seg.is_ok() {
                cut_err(parse_seg_row_with_err)
                    .map(NetRowElement::Segment)
                    .parse_next(input)
            } else {
                let is_arc = is_net_element(input, "(ARC");

                if is_arc.is_ok() {
                    cut_err(parse_arc_row_with_err)
                        .map(NetRowElement::Arc)
                        .parse_next(input)
                } else {
                    if peek((multispace0::<_, ContextError>, alt(("{", "}"))))
                        .parse_next(input)
                        .is_ok()
                    {
                        // winnow::combinator::repeat omits the backtrack error
                        // and ends the repeating for this part
                        return Err(ErrMode::Backtrack(ContextError::new()));
                    }

                    return Err(build_parser_cut_error(("row name", "PIN, VIA, SEG or ARC")));
                }
            }
        }
    }
}

// refactor candidate
fn parse_net_block_element(input: &mut &str) -> ModalResult<NetBlockElement> {
    let is_polygon = is_net_element(input, "{POLYGON");

    if is_polygon.is_ok() {
        cut_err(parse_polygon_block_with_err)
            .map(NetBlockElement::Polygon)
            .parse_next(input)
    } else {
        let is_polyvoid = is_net_element(input, "{POLYVOID");

        if is_polyvoid.is_ok() {
            cut_err(parse_polyvoid_block_with_err)
                .map(NetBlockElement::Polyvoid)
                .parse_next(input)
        } else {
            if peek((multispace0::<_, ContextError>, "}"))
                .parse_next(input)
                .is_ok()
            {
                // winnow::combinator::repeat omits the backtrack error
                // and ends the repeating for this part
                return Err(ErrMode::Backtrack(ContextError::new()));
            }

            return Err(build_parser_cut_error((
                "block name",
                "POLYGON or POLYVOID",
            )));
        }
    }
}

pub fn parse_net_block(input: &mut &str) -> ModalResult<Net> {
    open_block(input, "")?;

    let name = extract_string_entry(
        input,
        "NET",
        '\r',
        ExtractEntryErrorMessages {
            key_error_label: "net NET key",
            key_error_description: "invalid NET key name",
            value_error_label: "net NET value",
            value_error_description: "invalid NET attribute value",
        },
    )?;

    skip_ignored(input)?;

    let mut pins: Vec<Pin> = Vec::new();
    let mut vias: Vec<Via> = Vec::new();
    let mut segments: Vec<Segment> = Vec::new();
    let mut arcs: Vec<Arc> = Vec::new();

    let row_elements: Vec<NetRowElement> = repeat(0.., parse_net_row_element).parse_next(input)?;

    for row_element in row_elements {
        match row_element {
            NetRowElement::Pin(pin) => pins.push(pin),
            NetRowElement::Via(via) => vias.push(via),
            NetRowElement::Segment(segment) => segments.push(segment),
            NetRowElement::Arc(arc) => arcs.push(arc),
        }
    }

    let mut polygons: Vec<Polygon> = Vec::new();
    let mut polyvoids: Vec<Polyvoid> = Vec::new();

    let block_elements: Vec<NetBlockElement> =
        repeat(0.., parse_net_block_element).parse_next(input)?;

    for block_element in block_elements {
        match block_element {
            NetBlockElement::Polygon(polygon) => polygons.push(polygon),
            NetBlockElement::Polyvoid(polyvoid) => polyvoids.push(polyvoid),
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
    // malformed row and block names
    #[test_case(
        "{NET=\"GND\"\n  (xyzPIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("row name"),
            StrContext::Expected(StrContextValue::Description("PIN, VIA, SEG or ARC"))
        ); "PIN row name malversation"
    )]
    // malformed row and block names
    #[test_case(
        "{NET=\"GND\"\n  (PIxyzN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("row name"),
            StrContext::Expected(StrContextValue::Description("PIN, VIA, SEG or ARC"))
        ); "PIN row name malversation"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PINxyz X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("row name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after row name"))
        ); "PIN row name boundary"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (xyzVIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("row name"),
            StrContext::Expected(StrContextValue::Description("PIN, VIA, SEG or ARC"))
        ); "VIA row name malversation"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIxyzA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("row name"),
            StrContext::Expected(StrContextValue::Description("PIN, VIA, SEG or ARC"))
        ); "VIA row name malversation"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIAxyz X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("row name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after row name"))
        ); "VIA row name boundary"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (xyzSEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("row name"),
            StrContext::Expected(StrContextValue::Description("PIN, VIA, SEG or ARC"))
        ); "SEG row name malversation"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SExyzG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("row name"),
            StrContext::Expected(StrContextValue::Description("PIN, VIA, SEG or ARC"))
        ); "SEG row name malversation"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEGxyz X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("row name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after row name"))
        ); "SEG row name boundary"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (xyzARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("row name"),
            StrContext::Expected(StrContextValue::Description("PIN, VIA, SEG or ARC"))
        ); "ARC row name malversation"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARxyzC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("row name"),
            StrContext::Expected(StrContextValue::Description("PIN, VIA, SEG or ARC"))
        ); "ARC row name malversation"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARCxyz X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("row name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after row name"))
        ); "ARC row name boundary"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {xyzPOLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("block name"),
            StrContext::Expected(StrContextValue::Description("POLYGON or POLYVOID"))
        ); "POLYGON block name malversation"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYxyzGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("block name"),
            StrContext::Expected(StrContextValue::Description("POLYGON or POLYVOID"))
        ); "POLYGON block name malversation"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGONxyz T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("block name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after block name"))
        ); "POLYGON block name boundary"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {xyzPOLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("block name"),
            StrContext::Expected(StrContextValue::Description("POLYGON or POLYVOID"))
        ); "POLYVOID block name malversation"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYxyzVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("block name"),
            StrContext::Expected(StrContextValue::Description("POLYGON or POLYVOID"))
        ); "POLYVOID block name malversation"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOIDxyz ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("block name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after block name"))
        ); "POLYVOID block name boundary"
    )]
    // malformed row data
    #[test_case(
        "{NET=\"GND\"\n  (PIN xyzX=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("pin X key"),
            StrContext::Expected(StrContextValue::Description("invalid X key name"))
        ); "pin X key"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.952xyz7559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("pin X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "pin X value"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R= \"R.1 P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("pin R value"),
            StrContext::Expected(StrContextValue::Description("invalid R attribute value"))
        ); "pin R value"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.04763xyz77953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("via Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "via Y value"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 xyzW=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("seg W key"),
            StrContext::Expected(StrContextValue::Description("invalid W key name"))
        ); "seg W key"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.xyz5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("arc YC value"),
            StrContext::Expected(StrContextValue::Description("invalid YC attribute value"))
        ); "arc YC value"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=xyz1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("polyvoid ID value"),
            StrContext::Expected(StrContextValue::Description("invalid ID attribute value"))
        ); "polyvoid ID value"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=xyz8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("line X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "polyvoid line row X value"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POxyzUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  }\n}",
        (
            StrContext::Label("polygon T value"),
            StrContext::Expected(StrContextValue::Description("invalid T attribute value"))
        ); "polygon T value"
    )]
    #[test_case(
        "{NET=\"GND\"\n  (PIN X=7.9527559055 Y=-4.5472440945 R=\"D1.1\" P=1)\n  (PIN X=8.2677165354 Y=-4.6062992126 R=\"C3.2\" P=2)\n  (VIA X=8.1917322835 Y=-5.0476377953 P=18)\n  (SEG X1=8.1826771654 Y1=-4.9956692913 X2=8.1811023622 Y2=-4.9940944882 W=0.0196850394 L=\"F.Cu\")\n  (ARC X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0 W=0.0118110236 L=\"F.Cu\")\n  {POLYVOID ID=1 X=8.1712598425 Y=-4.9842519685\n    (LINE X=8.1712598425 Y=-4.9842519685)\n  }\n  {POLYVOID ID=1 X=8.1203307480 Y=-4.7089516535\n    (LINE X=8.1203307480 Y=-4.7089516535)\n  }\n  {POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {POLYVOID ID=1 X=7.9108462205 Y=-4.5709723228\n    (LINE X=7.9108462205 Y=-4.5709723228)\n  }\n  {POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.xyz5086490157)\n  }\n}",
        (
            StrContext::Label("line Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "polygon line Y value"
    )]
    fn rejects_malformed_net_block(mut input: &str, expected_errors: (StrContext, StrContext)) {
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
