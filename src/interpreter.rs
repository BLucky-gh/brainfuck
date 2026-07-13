use std::io::{BufRead, Read};

use crate::parser::Token;

pub struct Interpreter {
    state: Vec<u8>,
    idx: usize,
}

impl Interpreter {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            state: vec![0; 128],
            idx: 0,
        }
    }
    #[inline(always)]
    pub fn interpret(&mut self, tokens: &[Token]) {
        // let mut input_buf = String::new();

        tokens.iter().for_each(|token| match token {
            //wrapping add but saturting sub cause I don't feel like dealing with a sudden huge vec, but if it's already huge, might as well wrap
            Token::Next => self.idx = self.idx.wrapping_add(1),
            Token::Prev => self.idx = self.idx.saturating_sub(1),
            Token::Incr => self.incr(),
            Token::Decr => self.decr(),
            Token::In => {
                // println!("Please input value:");
                // Can't be bothered figuring out how to turn off line buffering so I'll just read the first character of the line
                let mut handle = std::io::stdin().lock();
                let mut buf = [0u8; 1];
                *self.cell_mut() = match handle.read_exact(&mut buf) {
                    Ok(_) if !buf.is_empty() => {
                        let c = buf[0];
                        if c == 92 { 0 } else { c }
                    }
                    //default to MAX to be similar to -1 for purposes of EOF checking, will probably implement EOF checking properly at some point
                    _ => 255,
                };
            }
            Token::Out => print!("{}", self.cell() as char),
            Token::Block(tokens) => {
                if self.cell() != 0 {
                    loop {
                        self.interpret(tokens);

                        // loop on nonzero
                        if self.cell() == 0 {
                            break;
                        }
                    }
                }
            }
        })
    }
    fn incr(&mut self) {
        if self.state.len() < self.idx {
            self.state.resize(self.idx, 0);
        }
        *self.cell_mut() = self.cell().wrapping_add(1);
    }
    fn decr(&mut self) {
        if self.state.len() < self.idx {
            self.state.resize(self.idx, 0);
        }
        *self.cell_mut() = self.cell().wrapping_sub(1);
    }

    fn cell(&self) -> u8 {
        self.state[self.idx]
    }
    fn cell_mut(&mut self) -> &mut u8 {
        &mut self.state[self.idx]
    }
}
