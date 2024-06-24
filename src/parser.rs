use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};
use winnow::{
    combinator::{dispatch, empty, fail, opt, preceded, repeat, terminated},
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

fn parse_bf(input: &mut &str) -> PResult<Vec<Token>> {
    repeat(0.., preceded(opt(none_of(VALID_TOKENS)), parse_token)).parse_next(input)
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

impl IntoIterator for Ast {
    type Item = Token;
    type IntoIter = <Vec<Token> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.root.into_iter()
    }
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
        let sample = "+,+ ->.<- [+-+],";

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
