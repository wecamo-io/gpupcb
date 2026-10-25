// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::{ModalResult, Parser};
use winnow::combinator::{alt, repeat};
use winnow::error::{ErrMode, ContextError, StrContext, StrContextValue};

use crate::model::{Polygon, PolygonType, Polyvoid};
use crate::parser::net::polygon::line::parse_line_row;
use crate::parser::utils::{ExtractEntryErrorMessages, close_block, extract_float_entry, extract_quoted_string_entry, extract_unquoted_string_entry, extract_unsigned_integer_entry, open_block};

mod line;

pub fn parse_polyvoid_block(input: &mut &str) -> ModalResult<Polyvoid> {
    open_block(input, "POLYVOID")?;

    let id = extract_unsigned_integer_entry(
        input,
        "ID",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polyvoid ID key",
            key_error_description: "invalid ID key name",
            value_error_label: "polyvoid ID value",
            value_error_description: "invalid ID attribute value",
        },
    )?;

    let x = extract_float_entry(
        input,
        "X",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polyvoid X key",
            key_error_description: "invalid X key name",
            value_error_label: "polyvoid X value",
            value_error_description: "invalid X attribute value",
        }
    )?;

    let y = extract_float_entry(
        input,
        "Y",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polyvoid Y key",
            key_error_description: "invalid Y key name",
            value_error_label: "polyvoid Y value",
            value_error_description: "invalid Y attribute value",
        }
    )?;

    let outline: Vec<(f64, f64)> = repeat(0.., parse_line_row).parse_next(input)?;

    close_block(input)?;

    Ok(Polyvoid {
        id,
        x,
        y,
        outline,
    })
}

pub fn parse_polygon_block(input: &mut &str) -> ModalResult<Polygon> {
    println!("{:#?}", input);
    open_block(input, "POLYGON")?;

    let polygon_type = {
        let t = alt(
            (
                |input: &mut &str| {
                    extract_quoted_string_entry(
                        input,
                        "T",
                        ExtractEntryErrorMessages {
                            key_error_label: "polygon T key",
                            key_error_description: "invalid T key name",
                            value_error_label: "polygon T value",
                            value_error_description: "invalid T attribute value",
                        }
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
                        }
                    )  
                },
            )
        ).parse_next(input)?;

        match t.as_str() {
            "POUR" => PolygonType::Pour,
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

    let layer = extract_quoted_string_entry(
        input,
        "L",
        ExtractEntryErrorMessages {
            key_error_label: "polyvoid L key",
            key_error_description: "invalid L key name",
            value_error_label: "polyvoid L value",
            value_error_description: "invalid L attribute value",
        },
    )?;

    let id = extract_unsigned_integer_entry(
        input,
        "ID",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polyvoid ID key",
            key_error_description: "invalid ID key name",
            value_error_label: "polyvoid ID value",
            value_error_description: "invalid ID attribute value",
        },
    )?;

    let x = extract_float_entry(
        input,
        "X",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polyvoid X key",
            key_error_description: "invalid X key name",
            value_error_label: "polyvoid X value",
            value_error_description: "invalid X attribute value",
        }
    )?;

    let y = extract_float_entry(
        input,
        "Y",
        ' ',
        ExtractEntryErrorMessages {
            key_error_label: "polyvoid Y key",
            key_error_description: "invalid Y key name",
            value_error_label: "polyvoid Y value",
            value_error_description: "invalid Y attribute value",
        }
    )?;

    let outline: Vec<(f64, f64)> = repeat(0.., parse_line_row).parse_next(input)?;

    close_block(input)?;

    Ok(Polygon {
        polygon_type,
        layer: String::from(layer),
        id,
        x,
        y,
        outline,
    })
}

#[cfg(test)]
mod polyvoid_block_parser_tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        "{POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\n    (LINE X=8.1200787402 Y=-4.6751968504)\n}",
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
        "{POLYGON T=POUR L=\"B.Cu\" ID=2 X=8.3127283071 Y=-4.5086490157\n    (LINE X=8.3127283071 Y=-4.5086490157)\n    }",
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
        "{POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504xyz\n    (LINE X=8.1200787402 Y=-4.6634706693)\n}",
        (
            StrContext::Label("polyvoid Y value"),
            StrContext::Expected(StrContextValue::Description("invalid Y attribute value"))
        ); "after Y value"
    )]
    // closing block
    #[test_case(
        "{POLYVOID ID=1 X=8.1200787402 Y=-4.6751968504\n    (LINE X=8.1200787402 Y=-4.6634706693)\nxyz}",
        (
            StrContext::Label("closing block"),
            StrContext::Expected(StrContextValue::Description("}"))
        )
    )]
    fn rejects_malformed_polyvoid_block(
        mut input: &str,
        expected_errors: (StrContext, StrContext)
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
}
