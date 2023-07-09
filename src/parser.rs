#![allow(unused_variables, dead_code)]

use crate::lexer::Lexer;
use crate::token::Token;

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        todo!()
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.copy;
        self.peek_token = self.lexer.next_token();
    }
}

