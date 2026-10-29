// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::ascii::multispace0;
use winnow::combinator::{alt, cut_err, peek, repeat};
use winnow::error::{ContextError, ErrMode, StrContext, StrContextValue};
use winnow::prelude::*;

use crate::model::{Polygon, PolygonType, Polyvoid};
use crate::parser::net::polygon::line::parse_line_row_with_err;
use crate::parser::utils::{
    ExtractEntryErrorMessages, close_block, extract_float_entry, extract_quoted_string_entry,
    extract_unquoted_string_entry, extract_unsigned_integer_entry, line_ending_with_four_spaces,
    line_ending_with_two_spaces, open_block, single_line_ending, single_space,
};

mod line;

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

    line_ending_with_four_spaces(input)?;

    let outline: Vec<(f64, f64)> = repeat(1.., parse_line_row_with_err).parse_next(input)?;

    close_block(input)?;

    alt((line_ending_with_two_spaces, single_line_ending))
        .map_err(|_| {
            let mut err = ContextError::new();
            err.push(StrContext::Label("line ending"));
            err.push(StrContext::Expected(StrContextValue::Description(
                "expected single line ending or new line with 2 spaces after",
            )));
            return ErrMode::Cut(err);
        })
        .parse_next(input)?;

    Ok(Polyvoid { id, x, y, outline })
}

pub fn parse_polygon_block(input: &mut &str) -> ModalResult<Polygon> {
    open_block(input, "POLYGON")?;

    single_space(input)?;

    let polygon_type = {
        let t = alt((
            |input: &mut &str| {
                extract_quoted_string_entry(
                    input,
                    "T",
                    ExtractEntryErrorMessages {
                        key_error_label: "polygon T key",
                        key_error_description: "invalid T key name",
                        value_error_label: "polygon T value",
                        value_error_description: "invalid T attribute value",
                    },
                )
            },
            |input: &mut &str| {
                extract_unquoted_string_entry(
                    input,
                    "T",
                    ' ',
                    ExtractEntryErrorMessages {
                        key_error_label: "polygon T key",
                        key_error_description: "invalid T key name",
                        value_error_label: "polygon T value",
                        value_error_description: "invalid T attribute value",
                    },
                )
            },
        ))
        .parse_next(input)?;

        match t.as_str() {
            "POUR" => PolygonType::Pour,
            _ => {
                let mut err = ContextError::new();
                err.push(StrContext::Label("polygon T value"));
                err.push(StrContext::Expected(StrContextValue::Description(
                    "invalid T attribute value",
                )));

                Err(ErrMode::Backtrack(err))
            }?,
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

    line_ending_with_four_spaces(input)?;

    let outline: Vec<(f64, f64)> = repeat(1.., parse_line_row_with_err).parse_next(input)?;

    close_block(input)?;

    alt((line_ending_with_two_spaces, single_line_ending))
        .map_err(|_| {
            let mut err = ContextError::new();
            err.push(StrContext::Label("line ending"));
            err.push(StrContext::Expected(StrContextValue::Description(
                "expected single line ending or new line with 2 spaces after",
            )));
            return ErrMode::Cut(err);
        })
        .parse_next(input)?;

    Ok(Polygon {
        polygon_type,
        layer: String::from(layer),
        id,
        x,
        y,
        outline,
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

#[cfg(test)]
mod polyvoid_block_parser_tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        "{POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n    (LINE X=8.1200787402 Y=-4.6751968504)\n  }\n  {...}",
        Polyvoid {
            id: 1,
            x: 8.1200787402,
            y: -4.6751968504,
            outline: vec![(8.1200787402, -4.6634706693), (8.1200787402, -4.6751968504)],
        }
    )]
    fn parses_polyvoid_block(mut input: &str, expected: Polyvoid) {
        let polyvoid = parse_polyvoid_block(&mut input).unwrap();

        assert_eq!(polyvoid, expected);
    }

    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n    }\n  (...)",
        Polygon {
            polygon_type: PolygonType::Pour,
            layer: String::from("B.Cu"),
            id: 2,
            x: 8.3127283071,
            y: -4.5086490157,
            outline: vec![(8.3127283071, -4.5086490157)],
        }
    )]
    fn parses_polygon_block(mut input: &str, expected: Polygon) {
        let polygon = parse_polygon_block(&mut input).unwrap();

        assert_eq!(polygon, expected);
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
    // closing block
    #[test_case(
        "{POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n  xyz}",
        (
            StrContext::Label("closing block"),
            StrContext::Expected(StrContextValue::Description("}"))
        )
    )]
    fn rejects_malformed_polyvoid_block(
        mut input: &str,
        expected_errors: (StrContext, StrContext),
    ) {
        let err = parse_polyvoid_block(&mut input).unwrap_err();

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
    // closing block
    #[test_case(
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n  xyz}",
        (
            StrContext::Label("closing block"),
            StrContext::Expected(StrContextValue::Description("}"))
        )
    )]
    fn rejects_malformed_polygon_block(mut input: &str, expected_errors: (StrContext, StrContext)) {
        let err = parse_polygon_block(&mut input).unwrap_err();

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
