// A variable can be of type Token::Identifier
// and hold a value of type Vec<Char>

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {

    // Single character tokens.
    PLUS(char),
    COMMA(char),
    SEMICOLON(char),
    LPAREN(char),
    RPAREN(char),
    LBRACE(char),
    RBRACE(char),

    // One or two character tokens.
    EQUAL(char),

    // Literals
    IDENT(Vec<char>),
    INT(Vec<char>),

    // Keywords
    FUNCTION(Vec<char>),
    LET(Vec<char>),
    ILLEGAL,
    EOF,
    NOTIMPLEMENTED
}
