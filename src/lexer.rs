use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    // current position in input (points to current char)
    read_position: usize,
    // current reading position in input (after current char)
    ch: char, // current char under examination
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
            self.ch = '0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    pub fn next_token(&mut self) -> Token {
        let new_token: Token;
        match self.ch {
            '=' => {
                new_token = Token::ASSIGN(self.ch);
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
            '+' => {
                new_token = Token::PLUS(self.ch)
            }
            '{' => {
                new_token = Token::LBRACE(self.ch)
            }
            '}' => {
                new_token = Token::RBRACE(self.ch)
            }
            _ => {
                if is_letter(self.ch) {
                    //return (self, Token::new(TokenType::ILLEGAL, self.read_identifier()))
                    return Token::ILLEGAL;
                } else {
                    //new_token = Token::new(TokenType::ILLEGAL, String::from(self.ch))
                    return Token::ILLEGAL;
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

#[test]
fn test_next_token() {
    let input = String::from("=+(){},;");

    let expected_tokens = [
        Token::ASSIGN('='),
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

/*#[test]
fn test_next_token_extented() {
    let input = String::from("let five = 5;
                             let ten = 10;
                             
                             let add = fn(x, y) {
                               x + y;
                             };

                             let result = add(five, ten);
                             ");

    let expected_tokens = [
        Token::new(TokenType::LET, String::from("let")),
        Token::new(TokenType::IDENT, String::from("five")),
        Token::new(TokenType::ASSIGN, String::from("=")),
        Token::new(TokenType::INT, String::from("5")),
        Token::new(TokenType::SEMICOLON, String::from(";")),
        Token::new(TokenType::LET, String::from("let")),
        Token::new(TokenType::IDENT, String::from("ten")),
        Token::new(TokenType::ASSIGN, String::from("=")),
        Token::new(TokenType::INT, String::from("10")),
        Token::new(TokenType::SEMICOLON, String::from(";")),
        Token::new(TokenType::LET, String::from("let")),
        Token::new(TokenType::IDENT, String::from("add")),
        Token::new(TokenType::ASSIGN, String::from("=")),
        Token::new(TokenType::FUNCTION, String::from("fn")),
        Token::new(TokenType::LPAREN, String::from("(")),
        Token::new(TokenType::IDENT, String::from("x")),
        Token::new(TokenType::COMMA, String::from(",")),
        Token::new(TokenType::IDENT, String::from("y")),
        Token::new(TokenType::RPAREN, String::from(")")),
        Token::new(TokenType::LBRACE, String::from("{")),
        Token::new(TokenType::IDENT, String::from("x")),
        Token::new(TokenType::PLUS, String::from("+")),
        Token::new(TokenType::IDENT, String::from("y")),
        Token::new(TokenType::SEMICOLON, String::from(";")),
        Token::new(TokenType::RBRACE, String::from("}")),
        Token::new(TokenType::SEMICOLON, String::from(";")),
        Token::new(TokenType::LET, String::from("let")),
        Token::new(TokenType::IDENT, String::from("result")),
        Token::new(TokenType::ASSIGN, String::from("=")),
        Token::new(TokenType::IDENT, String::from("add")),
        Token::new(TokenType::LPAREN, String::from("(")),
        Token::new(TokenType::IDENT, String::from("five")),
        Token::new(TokenType::COMMA, String::from(",")),
        Token::new(TokenType::IDENT, String::from("ten")),
        Token::new(TokenType::RPAREN, String::from(")")),
        Token::new(TokenType::SEMICOLON, String::from(";")),
        Token::new(TokenType::EOF, String::from("")),
    ];
    println!("========== TEST 2 =========");
    let mut actual: (Lexer, Token) = (new(input), Token::new(EOF, String::from("")));

    for expected_token in expected_tokens.into_iter() {
        actual = actual.0.next_token();

        assert_eq!(expected_token.literal, actual.1.literal);
        assert_eq!(expected_token.fnop_type, actual.1.fnop_type);
    }*/
//}

