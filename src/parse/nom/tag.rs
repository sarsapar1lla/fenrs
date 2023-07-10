use nom::character::complete::char;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::line_ending,
    multi::many_till,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

use crate::model::Tags;

pub fn parse_tags(input: &str) -> IResult<&str, Tags> {
    let (input, (tags, _)) = many_till(parse_tag, tag("\n"))(input)?;
    let tags: Tags = tags
        .into_iter()
        .map(|pair: (&str, &str)| (pair.0.to_string(), pair.1.to_string()))
        .collect();

    Ok((input, tags))
}

fn parse_tag(input: &str) -> IResult<&str, (&str, &str)> {
    fn parse_key_value(input: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(take_until(" "), char(' '), parse_value)(input)
    }
    terminated(
        delimited(char('['), parse_key_value, char(']')),
        line_ending,
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_until("\""), char('"'))(input)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn parses_tags() {
        let result = parse_tags("[Tag1 \"Value 1\"]\n[Tag2 \"Value 2\"]\n\n1. e4").unwrap();
        let mut expected = HashMap::new();
        expected.insert("Tag1".to_string(), "Value 1".to_string());
        expected.insert("Tag2".to_string(), "Value 2".to_string());

        assert_eq!(result, ("1. e4", expected))
    }

    #[test]
    fn parses_tag() {
        let result = parse_tag("[Tag \"Value\"]\n1.e4").unwrap();
        assert_eq!(result, ("1.e4", ("Tag", "Value")))
    }

    #[test]
    fn parses_value() {
        let result = parse_value(r#""Value""#).unwrap();
        assert_eq!(result, ("", "Value"))
    }
}
