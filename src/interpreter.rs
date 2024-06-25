use std::{
    io::{Read, Write},
    slice,
};

use crate::parser::{Ast, Token};

pub enum InterpretLogEntry {
    Next,
    Previous,
    Incr,
    Decr,
    Input(u8),
    Output(u8),
    JumpBack,
    JumpAhead,
    WalkIntoLoop,
    WalkOutOfLoop,
}

pub struct Interpreter<'a, I: Read, O: Write> {
    mem: Vec<u8>,
    idx: usize,
    input: I,
    output: O,
    stack: Vec<(usize, &'a [Token])>,
    pc: usize, //program counter
}

impl<'a, I: Read, O: Write> Interpreter<'a, I, O> {
    pub fn new(ast: &'a Ast, input: I, output: O) -> Self {
        Interpreter {
            stack: vec![(0, ast)], // ret_addr doesn't matter for the root of the stack
            input,
            output,
            mem: vec![0; 10],
            idx: 0,
            pc: 0,
        }
    }

    fn handle_token(&mut self, t: &'a Token) -> InterpretLogEntry {
        let current_item = Self::get_cursor_mut(&mut self.mem, self.idx);
        match t {
            Token::Next => {
                let new_idx = self.idx.saturating_add(1);
                if new_idx > self.mem.len() {
                    self.mem.resize(new_idx + 10, 0)
                }
                self.idx = new_idx;
                InterpretLogEntry::Next
            }
            Token::Previous => {
                let new_idx = self.idx.saturating_sub(1);
                self.idx = new_idx;
                InterpretLogEntry::Previous
            }
            Token::Incr => {
                *current_item = current_item.wrapping_add(1);
                InterpretLogEntry::Incr
            }
            Token::Decr => {
                *current_item = current_item.wrapping_sub(1);
                InterpretLogEntry::Decr
            }
            Token::Input => {
                let item = &mut [0];
                //TODO: figure out what I wanna do with the `Result`s
                self.input
                    .read_exact(item)
                    .unwrap_or_else(|e| panic!("Input error: {e:#?}"));
                *current_item = item[0];
                InterpretLogEntry::Input(item[0])
            }
            Token::Output => {
                let item = slice::from_ref(current_item);
                //TODO: figure out what I wanna do with the `Result`s
                self.output
                    .write_all(item)
                    .unwrap_or_else(|e| panic!("Output error: {e:#?}"));
                InterpretLogEntry::Output(item[0])
            }
            Token::Loop(subtree) => {
                if *current_item > 0 {
                    self.stack.push((self.pc, subtree));
                    InterpretLogEntry::WalkIntoLoop
                } else {
                    InterpretLogEntry::JumpAhead
                }
            }
        }
    }

    pub fn interpret_all(&mut self) {
        self.for_each(drop);
    }

    ///This function doesn't take `self` because that would mutably borrow `self` entirely and bind the lifetime
    /// of the returned borrow to `self` which makes us unable to do anything else with `self` anywhere else
    fn get_cursor_mut(mem: &mut Vec<u8>, idx: usize) -> &mut u8 {
        if idx >= mem.len() {
            mem.resize(idx + 10, 0)
        }
        mem.get_mut(idx)
            .expect("item should exist now since vector was just resized")
    }
}

impl<'a, I: Read, O: Write> Iterator for Interpreter<'a, I, O> {
    type Item = InterpretLogEntry;
    //this is a mess
    //TODO: try cleaning this up later
    fn next(&mut self) -> Option<Self::Item> {
        let (ret_addr, stack_top) = self
            .stack
            .last()
            .expect("The stack should always have at least the root of the ast");
        let ret_addr = *ret_addr;
        if self.pc >= stack_top.len() {
            if self.stack.len() == 1 {
                return None;
            }

            let current = Self::get_cursor_mut(&mut self.mem, self.idx);
            if *current == 0 {
                //go down the stack. I was considering abstracting this away
                //but this is literally the only place it's done
                self.stack.pop();
                self.pc = ret_addr;
                return Some(InterpretLogEntry::WalkOutOfLoop);
            } else {
                self.pc = 0;
                return Some(InterpretLogEntry::JumpBack);
            }
        }

        let item = match self.handle_token(
            stack_top
                .get(self.pc)
                .expect("Bounds checking performed in the if statement above"),
        ) {
            t @ InterpretLogEntry::WalkIntoLoop => {
                self.pc = 0;
                t
            }
            rest => {
                self.pc += 1;
                rest
            }
        };

        Some(item)
    }
}
