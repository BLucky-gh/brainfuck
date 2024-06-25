use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};
use winnow::{
    combinator::{dispatch, empty, fail, preceded, repeat, terminated},
    token::{any, none_of, take_until},
    PResult, Parser,
};

const VALID_TOKENS: [char; 8] = ['>', '<', '+', '-', ',', '.', '[', ']'];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Next,
    Previous,
    Incr,
    Decr,
    Input,
    Output,
    Loop(Vec<Token>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast {
    root: Vec<Token>,
}

impl FromStr for Ast {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Ast {
            root: parse_bf.parse(input).map_err(|e| e.to_string())?,
        })
    }
}

fn junk_parser(input: &mut &str) -> PResult<()> {
    repeat::<_, _, (), _, _>(0.., none_of(VALID_TOKENS)).parse_next(input)
}
fn parse_bf(input: &mut &str) -> PResult<Vec<Token>> {
    terminated(repeat(0.., preceded(junk_parser, parse_token)), junk_parser).parse_next(input)
}

fn parse_token(input: &mut &str) -> PResult<Token> {
    dispatch! {any;
        '+' => empty.value(Token::Incr),
        '-' => empty.value(Token::Decr),
        '>' => empty.value(Token::Next),
        '<' => empty.value(Token::Previous),
        ',' => empty.value(Token::Input),
        '.' => empty.value(Token::Output),
        '[' => terminated(take_until(0..,']').and_then(parse_bf),']').map(Token::Loop),
        _ => fail,
    }
    .parse_next(input)
}
impl Deref for Ast {
    type Target = [Token];
    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl DerefMut for Ast {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.root[..]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_example() {
        use Token::*;
        let sample = "junk     +,+  ->.<-  junk   [+-+],   junk";

        let expected = Ast {
            root: vec![
                Incr,
                Input,
                Incr,
                Decr,
                Next,
                Output,
                Previous,
                Decr,
                Loop(vec![Incr, Decr, Incr]),
                Input,
            ],
        };
        let actual = Ast::from_str(sample).map_err(|e| eprintln!("{e}")).unwrap();

        assert_eq!(actual, expected)
    }
}
