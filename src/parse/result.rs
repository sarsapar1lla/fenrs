use nom::{branch::alt, bytes::complete::tag, combinator::map_res, IResult};

use crate::model::GameResult;

use super::error::PgnParseError;

pub fn parse(input: &str) -> IResult<&str, GameResult> {
    let parser = alt((tag("1-0"), tag("0-1"), tag("1/2-1/2"), tag("*")));
    map_res(parser, |result| match result {
        "1-0" => Ok(GameResult::WhiteWin),
        "0-1" => Ok(GameResult::BlackWin),
        "1/2-1/2" => Ok(GameResult::Draw),
        "*" => Ok(GameResult::Ongoing),
        _ => Err(PgnParseError::new(format!(
            "'{result}' is not a valid game result"
        ))),
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_err_if_not_game_result() {
        let result = parse("something");
        assert!(result.is_err())
    }

    #[test]
    fn parses_white_win() {
        let result = parse("1-0 something").unwrap();
        assert_eq!(result, (" something", GameResult::WhiteWin))
    }

    #[test]
    fn parses_black_win() {
        let result = parse("0-1 something").unwrap();
        assert_eq!(result, (" something", GameResult::BlackWin))
    }

    #[test]
    fn parses_draw() {
        let result = parse("1/2-1/2 something").unwrap();
        assert_eq!(result, (" something", GameResult::Draw))
    }

    #[test]
    fn parses_ongoing() {
        let result = parse("* something").unwrap();
        assert_eq!(result, (" something", GameResult::Ongoing))
    }
}
