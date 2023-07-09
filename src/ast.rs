#![allow(unused_variables, dead_code)]

use crate::token::Token;

trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
struct Program {
    statements: Vec<Statement>,
}

/// Noeud racine de l'AST que le parser produit. Chaque programme valide est une série de statement.
impl Program {
    fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }
}

impl Node for Program {
    //TODO: Use std::fmt
    fn token_literal(&self) -> String {
        let mut output = String::new();
        for statement in &self.statements {
            output.push_str(&statement.token_literal());
        }

        output
    }
}

/// Ne produit pas de valeur. ```let x = 5``` ne produit pas de valeur
enum Statement {
    LetStatement(letStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement(let_statement) => let_statement.token_literal()
        }
    }
}

struct LetStatement {
    token: Token,
    name: Identifier,
    value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        let mut output = String::new();

        output.push_str(
            match &self.token {
                Token(literal) => literal
            });

        output
    }
}

/// Produit une valeur. ```add(5, 5)``` produit la valeur 10
enum Expression {
    Identifier(identifier)
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        todo!()
    }
}

struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        let mut output = String::new();

        output.push_str(
            match &self.token {
                Token(literal) => literal
            });

        output
    }
}
