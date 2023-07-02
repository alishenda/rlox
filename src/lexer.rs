use std::iter::Iterator;
use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,

    // current position in input (points to current char)
    position: usize,

    // current reading position in input (after current char)
    read_position: usize,

    // current char under examination
    ch: char,
}

impl Lexer {

    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            ch: '0',
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = ' ';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    pub fn read_identifier(&mut self) -> Token {
        let position = self.position;
        while self.position < self.input.len() && is_letter(self.ch) {
            self.read_char();
        }
        let identifier: String = self.input[position..self.position].into_iter().collect();

        return if identifier.eq("let") {
            Token::LET("let".chars().collect())
        } else if identifier.eq("fn") {
            Token::FUNCTION("fn".chars().collect())
        } else if identifier.eq("true") {
            Token::TRUE("true".chars().collect())
        } else if identifier.eq("false") {
            Token::FALSE("false".chars().collect())
        } else if identifier.eq("if") {
            Token::IF("if".chars().collect())
        } else if identifier.eq("else") {
            Token::ELSE("else".chars().collect())
        } else if identifier.eq("return") {
            Token::RETURN("return".chars().collect())
        } else {
            Token::IDENT(identifier.chars().collect())
        }
    }

    pub fn read_number(&mut self) -> Token {
        let position = self.position;
        while self.position < self.input.len() && is_digit(self.ch) {
            self.read_char();
        }

        let identifier: String = self.input[position..self.position].into_iter().collect();

        return Token::INT(identifier.chars().collect());
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {

        if self.read_position > self.input.len() {
            return Token::EOF;
        }

        let new_token: Token;
        self.skip_whitespace();

        match self.ch {
            '=' => {
                new_token = Token::EQUAL(self.ch);
            }
            ';' => {
                new_token = Token::SEMICOLON(self.ch);
            }
            '(' => {
                new_token = Token::LPAREN(self.ch);
            }
            ')' => {
                new_token = Token::RPAREN(self.ch)
            }
            ',' => {
                new_token = Token::COMMA(self.ch)
            }
            '<' => {
                new_token = Token::LT(self.ch)
            }
            '>' => {
                new_token = Token::GT(self.ch)
            }
            '+' => {
                new_token = Token::PLUS(self.ch)
            }
            '-' => {
                new_token = Token::MINUS(self.ch)
            }
            '*' => {
                new_token = Token::ASTERISK(self.ch)
            }
            '/' => {
                new_token = Token::SLASH(self.ch)
            }
            '!' => {
                new_token = Token::BANG(self.ch)
            }
            '{' => {
                new_token = Token::LBRACE(self.ch)
            }
            '}' => {
                new_token = Token::RBRACE(self.ch)
            }
            _ => {
                if is_letter(self.ch) {
                    return self.read_identifier();
                } else if is_digit(self.ch) {
                    return self.read_number();
                } else {
                    new_token = Token::ILLEGAL;
                }
            }
        }

        self.read_char();
        return new_token;
    }


}

fn is_letter(ch: char) -> bool {
    return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
}

fn is_digit(ch: char) -> bool {
    return '0' <= ch && ch <= '9';
}

#[test]
fn test_next_token() {
    let input = String::from("=+(){},;");

    let expected_tokens = [
        Token::EQUAL('='),
        Token::PLUS('+'),
        Token::LPAREN('('),
        Token::RPAREN(')'),
        Token::LBRACE('{'),
        Token::RBRACE('}'),
        Token::COMMA(','),
        Token::SEMICOLON(';'),
    ];

    println!("========== TEST 1 =========");
    let mut l = Lexer::new(input.chars().collect());
    l.read_char();

    let mut actual: Token;
    for expected_token in expected_tokens.into_iter() {
        actual = l.next_token();

        assert_eq!(expected_token, actual);
    }
}

#[test]
fn test_next_token_extended() {
    let input = String::from("let five = 5;
                             let ten = 10;
                             
                             let add = fn(x, y) {
                               x + y;
                             };

                             let result = add(five, ten);
                             !-/*5;
                             5 < 10 > 5;

                             if (5 < 10) {
                                 return true;
                             } else {
                                 return false;
                             }");

    let expected_tokens = [
        Token::LET("let".chars().collect()),
        Token::IDENT("five".chars().collect()),
        Token::EQUAL('='),
        Token::INT("5".chars().collect()), 
        Token::SEMICOLON(';'),
        Token::LET("let".chars().collect()),
        Token::IDENT("ten".chars().collect()),
        Token::EQUAL('='),
        Token::INT("10".chars().collect()),
        Token::SEMICOLON(';'),
        Token::LET("let".chars().collect()),
        Token::IDENT("add".chars().collect()),
        Token::EQUAL('='),
        Token::FUNCTION("fn".chars().collect()),
        Token::LPAREN('('),
        Token::IDENT("x".chars().collect()),
        Token::COMMA(','),
        Token::IDENT("y".chars().collect()),
        Token::RPAREN(')'),
        Token::LBRACE('{'),
        Token::IDENT("x".chars().collect()),
        Token::PLUS('+'),
        Token::IDENT("y".chars().collect()),
        Token::SEMICOLON(';'),
        Token::RBRACE('}'),
        Token::SEMICOLON(';'),
        Token::LET("let".chars().collect()),
        Token::IDENT("result".chars().collect()),
        Token::EQUAL('='),
        Token::IDENT("add".chars().collect()),
        Token::LPAREN('('),
        Token::IDENT("five".chars().collect()),
        Token::COMMA(','),
        Token::IDENT("ten".chars().collect()),
        Token::RPAREN(')'),
        Token::SEMICOLON(';'),
        Token::BANG('!'),
        Token::MINUS('-'),
        Token::SLASH('/'),
        Token::ASTERISK('*'),
        Token::INT("5".chars().collect()),
        Token::SEMICOLON(';'),
        Token::INT("5".chars().collect()),
        Token::LT('<'),
        Token::INT("10".chars().collect()),
        Token::GT('>'),
        Token::INT("5".chars().collect()),
        Token::SEMICOLON(';'),
        Token::IF("if".chars().collect()),
        Token::LPAREN('('),
        Token::INT("5".chars().collect()),
        Token::LT('<'),
        Token::INT("10".chars().collect()),
        Token::RPAREN(')'),
        Token::LBRACE('{'),
        Token::RETURN("return".chars().collect()),
        Token::TRUE("true".chars().collect()),
        Token::SEMICOLON(';'),
        Token::RBRACE('}'),
        Token::ELSE("else".chars().collect()),
        Token::LBRACE('{'),
        Token::RETURN("return".chars().collect()),
        Token::FALSE("false".chars().collect()),
        Token::SEMICOLON(';'),
        Token::RBRACE('}'),
        Token::EOF,
    ];
    println!("========== TEST 2 =========");
    let mut l = Lexer::new(input.chars().collect());
    l.read_char();

    let mut actual: Token;
    for expected_token in expected_tokens.into_iter() {
        actual = l.next_token();
        assert_eq!(expected_token, actual);
    }
}

