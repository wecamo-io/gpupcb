// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::multispace0;
use winnow::combinator::{alt, cut_err, peek, repeat};
use winnow::error::{ContextError, ErrMode};
use winnow::prelude::*;
use winnow::token::literal;

use crate::model::{Curve, Polygon, PolygonType, Polyline, Polyvoid};
use crate::parser::net::polygon::curve::parse_curve_row_with_err;
use crate::parser::net::polygon::line::parse_line_row_with_err;
use crate::parser::utils::{
    ExtractEntryErrorMessages, build_parser_backtrack_error, build_parser_cut_error, close_block,
    extract_float_entry, extract_quoted_string_entry, extract_string_entry,
    extract_unsigned_integer_entry, open_block, single_space, skip_ignored,
};

mod curve;
mod line;

#[derive(Debug)]
enum PolyRowElement {
    Line((f64, f64)),
    Curve(Curve),
}

fn is_poly_element<'a>(input: &'a mut &str, lit: &str) -> Result<(&'a str, &'a str), ContextError> {
    peek((multispace0::<_, ContextError>, literal(lit))).parse_next(input)
}

fn parse_poly_row_element(input: &mut &str) -> ModalResult<PolyRowElement> {
    let is_line = is_poly_element(input, "(LINE");

    if is_line.is_ok() {
        cut_err(parse_line_row_with_err)
            .map(PolyRowElement::Line)
            .parse_next(input)
    } else {
        let is_curve = is_poly_element(input, "(CURVE");

        if is_curve.is_ok() {
            cut_err(parse_curve_row_with_err)
                .map(PolyRowElement::Curve)
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

            Err(build_parser_cut_error(("row name", "LINE or CURVE")))
        }
    }
}

pub fn parse_polyvoid_block(input: &mut &str) -> ModalResult<Polyvoid> {
    open_block(input, "POLYVOID")?;

    single_space(input)?;

    let id = extract_unsigned_integer_entry(
        input,
        "ID",
        ExtractEntryErrorMessages {
            key_error_label: "polyvoid ID key",
            key_error_description: "invalid ID key name",
            value_error_label: "polyvoid ID value",
            value_error_description: "invalid ID attribute value",
        },
    )?;

    single_space(input)?;

    let x = extract_float_entry(
        input,
        "X",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polyvoid X key",
            key_error_description: "invalid X key name",
            value_error_label: "polyvoid X value",
            value_error_description: "invalid X attribute value",
        },
    )?;

    single_space(input)?;

    let y = extract_float_entry(
        input,
        "Y",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polyvoid Y key",
            key_error_description: "invalid Y key name",
            value_error_label: "polyvoid Y value",
            value_error_description: "invalid Y attribute value",
        },
    )?;

    skip_ignored(input)?;

    let mut lines: Vec<(f64, f64)> = Vec::new();
    let mut curves: Vec<Curve> = Vec::new();

    let row_elements: Vec<PolyRowElement> =
        repeat(0.., parse_poly_row_element).parse_next(input)?;

    for row_element in row_elements {
        match row_element {
            PolyRowElement::Line(line) => lines.push(line),
            PolyRowElement::Curve(curve) => curves.push(curve),
        }
    }

    close_block(input)?;

    skip_ignored(input)?;

    Ok(Polyvoid {
        id,
        x,
        y,
        lines,
        curves,
    })
}

pub fn parse_polygon_block(input: &mut &str) -> ModalResult<Polygon> {
    open_block(input, "POLYGON")?;

    single_space(input)?;

    let polygon_type = {
        let t = extract_string_entry(
            input,
            "T",
            ' ',
            ExtractEntryErrorMessages {
                key_error_label: "polygon T key",
                key_error_description: "invalid T key name",
                value_error_label: "polygon T value",
                value_error_description: "invalid T attribute value",
            },
        )?;

        match t.as_str() {
            "POUR" => PolygonType::Pour,
            _ => Err(build_parser_backtrack_error((
                "polygon T value",
                "invalid T attribute value",
            )))?,
        }
    };

    single_space(input)?;

    let layer = extract_quoted_string_entry(
        input,
        "L",
        ExtractEntryErrorMessages {
            key_error_label: "polygon L key",
            key_error_description: "invalid L key name",
            value_error_label: "polygon L value",
            value_error_description: "invalid L attribute value",
        },
    )?;

    single_space(input)?;

    let id = extract_unsigned_integer_entry(
        input,
        "ID",
        ExtractEntryErrorMessages {
            key_error_label: "polygon ID key",
            key_error_description: "invalid ID key name",
            value_error_label: "polygon ID value",
            value_error_description: "invalid ID attribute value",
        },
    )?;

    single_space(input)?;

    let x = extract_float_entry(
        input,
        "X",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polygon X key",
            key_error_description: "invalid X key name",
            value_error_label: "polygon X value",
            value_error_description: "invalid X attribute value",
        },
    )?;

    single_space(input)?;

    let y = extract_float_entry(
        input,
        "Y",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polygon Y key",
            key_error_description: "invalid Y key name",
            value_error_label: "polygon Y value",
            value_error_description: "invalid Y attribute value",
        },
    )?;

    skip_ignored(input)?;

    let mut lines: Vec<(f64, f64)> = Vec::new();
    let mut curves: Vec<Curve> = Vec::new();

    let row_elements: Vec<PolyRowElement> =
        repeat(0.., parse_poly_row_element).parse_next(input)?;

    for row_element in row_elements {
        match row_element {
            PolyRowElement::Line(line) => lines.push(line),
            PolyRowElement::Curve(curve) => curves.push(curve),
        }
    }

    close_block(input)?;

    skip_ignored(input)?;

    Ok(Polygon {
        polygon_type,
        layer,
        id,
        x,
        y,
        lines,
        curves,
    })
}

pub fn parse_polyline_block(input: &mut &str) -> ModalResult<Polyline> {
    open_block(input, "POLYLINE")?;

    single_space(input)?;

    let layer = extract_quoted_string_entry(
        input,
        "L",
        ExtractEntryErrorMessages {
            key_error_label: "polyline L key",
            key_error_description: "invalid L key name",
            value_error_label: "polyline L value",
            value_error_description: "invalid L attribute value",
        },
    )?;

    single_space(input)?;

    let width = extract_float_entry(
        input,
        "W",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polyline W key",
            key_error_description: "invalid W key name",
            value_error_label: "polyline W value",
            value_error_description: "invalid W attribute value",
        },
    )?;

    single_space(input)?;

    let id = extract_unsigned_integer_entry(
        input,
        "ID",
        ExtractEntryErrorMessages {
            key_error_label: "polyline ID key",
            key_error_description: "invalid ID key name",
            value_error_label: "polyline ID value",
            value_error_description: "invalid ID attribute value",
        },
    )?;

    single_space(input)?;

    let x = extract_float_entry(
        input,
        "X",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polyline X key",
            key_error_description: "invalid X key name",
            value_error_label: "polyline X value",
            value_error_description: "invalid X attribute value",
        },
    )?;

    single_space(input)?;

    let y = extract_float_entry(
        input,
        "Y",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polyline Y key",
            key_error_description: "invalid Y key name",
            value_error_label: "polyline Y value",
            value_error_description: "invalid Y attribute value",
        },
    )?;

    skip_ignored(input)?;

    let mut lines: Vec<(f64, f64)> = Vec::new();
    let mut curves: Vec<Curve> = Vec::new();

    let row_elements: Vec<PolyRowElement> =
        repeat(0.., parse_poly_row_element).parse_next(input)?;

    for row_element in row_elements {
        match row_element {
            PolyRowElement::Line(line) => lines.push(line),
            PolyRowElement::Curve(curve) => curves.push(curve),
        }
    }

    close_block(input)?;

    skip_ignored(input)?;

    Ok(Polyline {
        layer,
        width,
        id,
        x,
        y,
        lines,
        curves,
    })
}

pub fn parse_polyvoid_block_with_err(input: &mut &str) -> ModalResult<Polyvoid> {
    peek((multispace0, "{POLYVOID")).parse_next(input)?;

    cut_err(parse_polyvoid_block).parse_next(input)
}

pub fn parse_polygon_block_with_err(input: &mut &str) -> ModalResult<Polygon> {
    peek((multispace0, "{POLYGON")).parse_next(input)?;

    cut_err(parse_polygon_block).parse_next(input)
}

pub fn parse_polyline_block_with_err(input: &mut &str) -> ModalResult<Polyline> {
    peek((multispace0, "{POLYLINE")).parse_next(input)?;

    cut_err(parse_polyline_block).parse_next(input)
}

#[cfg(test)]
mod polyvoid_block_parser_tests {
    use super::*;
    use test_case::test_case;
    use winnow::error::{ErrMode, StrContext, StrContextValue};

    #[test_case(
        "{POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)\n  (LINE X=8.1200787402 Y=-4.6634706693)\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {...}",
        Polyvoid {
            id: 1,
            x: 8.1200787402,
            y: -4.6751968504,
            lines: vec![(8.1200787402, -4.6634706693), (8.1200787402, -4.6751968504)],
            curves: vec![
                Curve {
                    x1: 8.2480314961,
                    y1: -4.8854330709,
                    x2: 8.3055118110,
                    y2: -4.9429133858,
                    xc: 0.5,
                    yc: 0.5,
                    radius: 1.0,
                }
            ],
        }
    )]
    fn parses_polyvoid_block(mut input: &str, expected: Polyvoid) {
        let polyvoid = parse_polyvoid_block(&mut input).unwrap();

        assert_eq!(polyvoid, expected);
    }

    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)\n  (LINE X=8.3127283071 Y=-4.5086490157)\n    }\n  (...)",
        Polygon {
            polygon_type: PolygonType::Pour,
            layer: String::from("B.Cu"),
            id: 2,
            x: 8.3127283071,
            y: -4.5086490157,
            lines: vec![(8.3127283071, -4.5086490157)],
            curves: vec![
                Curve {
                    x1: 8.2480314961,
                    y1: -4.8854330709,
                    x2: 8.3055118110,
                    y2: -4.9429133858,
                    xc: 0.5,
                    yc: 0.5,
                    radius: 1.0,
                }
            ],
        }
    )]
    fn parses_polygon_block(mut input: &str, expected: Polygon) {
        let polygon = parse_polygon_block(&mut input).unwrap();

        assert_eq!(polygon, expected);
    }

    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 ID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n    (CURVE X1=8.2480314961 Y1=-4.8854330709 X2=8.3055118110 Y2=-4.9429133858 XC=0.5 YC=0.5 R=1.0)\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }",
        Polyline {
            layer: String::from("B.Cu"),
            width: 0.25,
            id: 3,
            x: 8.1200787402,
            y: -4.6751968504,
            lines: vec![
                (8.1200787402, -4.6634706693),
                (8.1200787402, -4.6751968504),
            ],
            curves: vec![
                Curve {
                    x1: 8.2480314961,
                    y1: -4.8854330709,
                    x2: 8.3055118110,
                    y2: -4.9429133858,
                    xc: 0.5,
                    yc: 0.5,
                    radius: 1.0,
                }
            ],
        }
    )]
    fn parses_polyline_block(mut input: &str, expected: Polyline) {
        let polyline = parse_polyline_block(&mut input).unwrap();

        assert_eq!(polyline, expected);
    }

    // opening block
    #[test_case(
        "xyz{POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("opening block"),
            StrContext::Expected(StrContextValue::Description("{"))
        )
    )]
    // block name
    #[test_case(
        "{xyzPOLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("block name"),
            StrContext::Expected(StrContextValue::Description("POLYVOID"))
        )
    )]
    #[test_case(
        "{POLYVOIDxyz ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("block name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after block name"))
        )
    )]
    // ID entry
    #[test_case(
        "{POLYVOID xyzID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyvoid ID key"),
            StrContext::Expected(StrContextValue::Description("invalid ID key name"))
        ); "before ID key"
    )]
    #[test_case(
        "{POLYVOID IDxyz=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyvoid ID key"),
            StrContext::Expected(StrContextValue::Description("invalid ID key name"))
        ); "after ID key"
    )]
    #[test_case(
        "{POLYVOID ID=xyz1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyvoid ID value"),
            StrContext::Expected(StrContextValue::Description("invalid ID attribute value"))
        ); "before ID value"
    )]
    // X entry
    #[test_case(
        "{POLYVOID ID=1 xyzX=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyvoid X key"),
            StrContext::Expected(StrContextValue::Description("invalid X key name"))
        ); "before X key"
    )]
    #[test_case(
        "{POLYVOID ID=1 Xxyz=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyvoid X key"),
            StrContext::Expected(StrContextValue::Description("invalid X key name"))
        ); "after X key"
    )]
    #[test_case(
        "{POLYVOID ID=1 X=xyz8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyvoid X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "before X value"
    )]
    #[test_case(
        "{POLYVOID ID=1 X=8xyz.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyvoid X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "inside X value"
    )]
    #[test_case(
        "{POLYVOID ID=1 X=8.xyz1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyvoid X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "inside X value"
    )]
    #[test_case(
        "{POLYVOID ID=1 X=8.1200787402xyz Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyvoid X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "after X value"
    )]
    // Y entry
    #[test_case(
        "{POLYVOID ID=1 X=8.1200787402 xyzY=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyvoid Y key"),
            StrContext::Expected(StrContextValue::Description("invalid Y key name"))
        ); "before Y key"
    )]
    #[test_case(
        "{POLYVOID ID=1 X=8.1200787402 Yxyz=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyvoid Y key"),
            StrContext::Expected(StrContextValue::Description("invalid Y key name"))
        ); "after Y key"
    )]
    #[test_case(
        "{POLYVOID ID=1 X=8.1200787402 Y=xyz-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyvoid Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "before Y value"
    )]
    #[test_case(
        "{POLYVOID ID=1 X=8.1200787402 Y=-4xyz.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyvoid Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "inside Y value"
    )]
    #[test_case(
        "{POLYVOID ID=1 X=8.1200787402 Y=-4.xyz6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyvoid Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "inside Y value"
    )]
    #[test_case(
        "{POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504xyz\n    (LINE X=8.1200787402 Y=-4.6634706693)\n  }\n  {...}",
        (
            StrContext::Label("polyvoid Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "after Y value"
    )]
    fn rejects_malformed_polyvoid_block(
        mut input: &str,
        expected_errors: (StrContext, StrContext),
    ) {
        let err = parse_polyvoid_block(&mut input).unwrap_err();

        let errors = match err {
            ErrMode::Backtrack(ref err) | ErrMode::Cut(ref err) => {
                let mut iter = err.context();

                (iter.next().unwrap(), iter.next().unwrap())
            }
            ErrMode::Incomplete(_) => todo!(),
        };

        assert_eq!(*errors.0, expected_errors.0);
        assert_eq!(*errors.1, expected_errors.1);
    }

    // opening block
    #[test_case(
        "xyz{POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("opening block"),
            StrContext::Expected(StrContextValue::Description("{"))
        )
    )]
    // block name
    #[test_case(
        "{xyzPOLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("block name"),
            StrContext::Expected(StrContextValue::Description("POLYGON"))
        )
    )]
    #[test_case(
        "{POLYGONxyz T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("block name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after block name"))
        )
    )]
    // T entry
    #[test_case(
        "{POLYGON xyzT=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon T key"),
            StrContext::Expected(StrContextValue::Description("invalid T key name"))
        ); "before T key"
    )]
    #[test_case(
        "{POLYGON Txyz=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon T key"),
            StrContext::Expected(StrContextValue::Description("invalid T key name"))
        ); "after T key"
    )]
    #[test_case(
        "{POLYGON T=xyzPOUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon T value"),
            StrContext::Expected(StrContextValue::Description("invalid T attribute value"))
        ); "before T value"
    )]
    #[test_case(
        "{POLYGON T=POURxyz L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon T value"),
            StrContext::Expected(StrContextValue::Description("invalid T attribute value"))
        ); "after T value"
    )]
    // L entry
    #[test_case(
        "{POLYGON T=POUR xyzL=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon L key"),
            StrContext::Expected(StrContextValue::Description("invalid L key name"))
        ); "before L key"
    )]
    #[test_case(
        "{POLYGON T=POUR Lxyz=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon L key"),
            StrContext::Expected(StrContextValue::Description("invalid L key name"))
        ); "after L key"
    )]
    #[test_case(
        "{POLYGON T=POUR L=xyz\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon L value"),
            StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
        ); "before L value"
    )]
    #[test_case(
        "{POLYGON T=POUR L=B.Cu ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon L value"),
            StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
        ); "after L value"
    )]
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon L value"),
            StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
        ); "after L value"
    )]
    // ID entry
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" xyzID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon ID key"),
            StrContext::Expected(StrContextValue::Description("invalid ID key name"))
        ); "before ID key"
    )]
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" IDxyz=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon ID key"),
            StrContext::Expected(StrContextValue::Description("invalid ID key name"))
        ); "after ID key"
    )]
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=xyz2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon ID value"),
            StrContext::Expected(StrContextValue::Description("invalid ID attribute value"))
        ); "before ID value"
    )]
    // X entry
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 xyzX=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon X key"),
            StrContext::Expected(StrContextValue::Description("invalid X key name"))
        ); "before X key"
    )]
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 Xxyz=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon X key"),
            StrContext::Expected(StrContextValue::Description("invalid X key name"))
        ); "after X key"
    )]
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 X=xyz8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "before X value"
    )]
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 X=8xyz.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "inside X value"
    )]
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.xyz3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "inside X value"
    )]
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071xyz Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "after X value"
    )]
    // Y entry
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 xyzY=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon Y key"),
            StrContext::Expected(StrContextValue::Description("invalid Y key name"))
        ); "before Y key"
    )]
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Yxyz=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon Y key"),
            StrContext::Expected(StrContextValue::Description("invalid Y key name"))
        ); "after Y key"
    )]
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=xyz-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "before Y value"
    )]
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4xyz.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "inside Y value"
    )]
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.xyz5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "inside Y value"
    )]
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157xyz\n    (LINE X=8.3127283071 Y=-4.5086490157)\n}",
        (
            StrContext::Label("polygon Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "after Y value"
    )]
    fn rejects_malformed_polygon_block(mut input: &str, expected_errors: (StrContext, StrContext)) {
        let err = parse_polygon_block(&mut input).unwrap_err();

        let errors = match err {
            ErrMode::Backtrack(ref err) | ErrMode::Cut(ref err) => {
                let mut iter = err.context();

                (iter.next().unwrap(), iter.next().unwrap())
            }
            ErrMode::Incomplete(_) => todo!(),
        };

        assert_eq!(*errors.0, expected_errors.0);
        assert_eq!(*errors.1, expected_errors.1);
    }

    // opening block
    #[test_case(
        "xyz{POLYLINE L=\"B.Cu\" W=0.25 ID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("opening block"),
            StrContext::Expected(StrContextValue::Description("{"))
        )
    )]
    // block name
    #[test_case(
        "{xyzPOLYLINE L=\"B.Cu\" W=0.25 ID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("block name"),
            StrContext::Expected(StrContextValue::Description("POLYLINE"))
        )
    )]
    #[test_case(
        "{POLYLINExyz L=\"B.Cu\" W=0.25 ID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("block name boundary"),
            StrContext::Expected(StrContextValue::Description("valid character after block name"))
        )
    )]
    // L entry
    #[test_case(
        "{POLYLINE xyzL=\"B.Cu\" W=0.25 ID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline L key"),
            StrContext::Expected(StrContextValue::Description("invalid L key name"))
        ); "before L key"
    )]
    #[test_case(
        "{POLYLINE Lxyz=\"B.Cu\" W=0.25 ID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline L key"),
            StrContext::Expected(StrContextValue::Description("invalid L key name"))
        ); "after L key"
    )]
    #[test_case(
        "{POLYLINE L=xyz\"B.Cu\" W=0.25 ID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline L value"),
            StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
        ); "before L value"
    )]
    #[test_case(
        "{POLYLINE L=B.Cu W=0.25 ID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline L value"),
            StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
        ); "after L value"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu W=0.25 ID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline L value"),
            StrContext::Expected(StrContextValue::Description("invalid L attribute value"))
        ); "unterminated L value"
    )]
    // W entry
    #[test_case(
        "{POLYLINE L=\"B.Cu\" xyzW=0.25 ID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline W key"),
            StrContext::Expected(StrContextValue::Description("invalid W key name"))
        ); "before W key"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" Wxyz=0.25 ID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline W key"),
            StrContext::Expected(StrContextValue::Description("invalid W key name"))
        ); "after W key"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=xyz0.25 ID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline W value"),
            StrContext::Expected(StrContextValue::Description("invalid W attribute value"))
        ); "before W value"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0xyz.25 ID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline W value"),
            StrContext::Expected(StrContextValue::Description("invalid W attribute value"))
        ); "inside W value"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.xyz25 ID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline W value"),
            StrContext::Expected(StrContextValue::Description("invalid W attribute value"))
        ); "inside W value"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25xyz ID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline W value"),
            StrContext::Expected(StrContextValue::Description("invalid W attribute value"))
        ); "after W value"
    )]
    // ID entry
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 xyzID=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline ID key"),
            StrContext::Expected(StrContextValue::Description("invalid ID key name"))
        ); "before ID key"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 IDxyz=3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline ID key"),
            StrContext::Expected(StrContextValue::Description("invalid ID key name"))
        ); "after ID key"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 ID=xyz3 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline ID value"),
            StrContext::Expected(StrContextValue::Description("invalid ID attribute value"))
        ); "before ID value"
    )]
    // X entry
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 ID=3 xyzX=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline X key"),
            StrContext::Expected(StrContextValue::Description("invalid X key name"))
        ); "before X key"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 ID=3 Xxyz=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline X key"),
            StrContext::Expected(StrContextValue::Description("invalid X key name"))
        ); "after X key"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 ID=3 X=xyz8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "before X value"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 ID=3 X=8xyz.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "inside X value"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 ID=3 X=8.xyz1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "inside X value"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 ID=3 X=8.1200787402xyz Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline X value"),
            StrContext::Expected(StrContextValue::Description("invalid X attribute value"))
        ); "after X value"
    )]
    // Y entry
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 ID=3 X=8.1200787402 xyzY=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline Y key"),
            StrContext::Expected(StrContextValue::Description("invalid Y key name"))
        ); "before Y key"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 ID=3 X=8.1200787402 Yxyz=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline Y key"),
            StrContext::Expected(StrContextValue::Description("invalid Y key name"))
        ); "after Y key"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 ID=3 X=8.1200787402 Y=xyz-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "before Y value"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 ID=3 X=8.1200787402 Y=-4xyz.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "inside Y value"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 ID=3 X=8.1200787402 Y=-4.xyz6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "inside Y value"
    )]
    #[test_case(
        "{POLYLINE L=\"B.Cu\" W=0.25 ID=3 X=8.1200787402 Y=-4.6751968504xyz\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyline Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "after Y value"
    )]
    fn rejects_malformed_polyline_block(
        mut input: &str,
        expected_errors: (StrContext, StrContext),
    ) {
        let err = parse_polyline_block(&mut input).unwrap_err();

        let errors = match err {
            ErrMode::Backtrack(ref err) | ErrMode::Cut(ref err) => {
                let mut iter = err.context();

                (iter.next().unwrap(), iter.next().unwrap())
            }
            ErrMode::Incomplete(_) => todo!(),
        };

        assert_eq!(*errors.0, expected_errors.0);
        assert_eq!(*errors.1, expected_errors.1);
    }
}
