// A variable can be of type Token::Identifier
// and hold a value of type Vec<Char>

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT(Vec<char>),
    INT(Vec<char>),
    ASSIGN(char),
    PLUS(char),
    COMMA(char),
    SEMICOLON(char),
    LPAREN(char),
    RPAREN(char),
    LBRACE(char),
    RBRACE(char),
    FUNCTION,
    LET
}
