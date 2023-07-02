// A variable can be of type Token::Identifier
// and hold a value of type Vec<Char>

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {

    // Single character tokens.
    PLUS(char),
    MINUS(char),
    SLASH(char),
    ASTERISK(char),
    COMMA(char),
    SEMICOLON(char),
    LPAREN(char),
    RPAREN(char),
    LBRACE(char),
    RBRACE(char),

    // One or two character tokens.
    ASSIGN(char),
    EQ(Vec<char>),
    NOT_EQ(Vec<char>),
    BANG(char),
    LT(char),
    GT(char),

    // Literals
    IDENT(Vec<char>),
    INT(Vec<char>),

    // Keywords
    FUNCTION(Vec<char>),
    LET(Vec<char>),
    TRUE(Vec<char>),
    FALSE(Vec<char>),
    IF(Vec<char>),
    ELSE(Vec<char>),
    RETURN(Vec<char>),

    ILLEGAL,
    NOTIMPLEMENTED,
    EOF
}
