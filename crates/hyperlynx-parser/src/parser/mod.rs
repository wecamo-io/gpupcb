// SPDX-FileCopyrightText: 2026 Wecamo <https://www.wecamo.io/>
//
// SPDX-License-Identifier: GPL 3.0

use winnow::{ModalResult, Parser};
use winnow::token::{literal, take_until};

pub fn parse_version(input: &mut &str) -> ModalResult<String> {
    literal("{VERSION=").parse_next(input)?;
    let version = take_until(0.., '}').parse_next(input)?;
    literal("}").parse_next(input)?;

    Ok(version.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    // VERSION BLOCK UNIT TESTS

    #[test]
    fn parses_version_block() {
        let mut input = "{VERSION=2.14}";
        let expected = "2.14";

        let version = parse_version.parse_next(&mut input).unwrap();

        assert_eq!(version, expected);
    }
}
