use crate::{ast::{Expr, Statement}, tokens::Token};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.current).cloned();
        self.current += 1;
        token
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.peek()? {
            Token::Let => {
                self.advance();
                if let Some(Token::Ident(name)) = self.advance() {
                    if let Some(Token::Equals) = self.advance() {
                        let expr = self.parse_expression()?;
                        return Some(Statement::Let(name, expr));
                    }
                }
            }
            _ => {
                let expr = self.parse_expression()?;
                return Some(Statement::Expression(expr));
            }
        }
        None
    }

    fn parse_expression(&mut self) -> Option<Expr> {
        let mut left = self.parse_primary()?;
        while let Some(op) = self.peek() {
            match op {
                Token::Plus | Token::Minus | Token::Star | Token::Slash => {
                    let op = self.advance().unwrap();
                    let right = self.parse_primary()?;
                    left = Expr::BinaryOp { 
                        left: Box::new(left), 
                        op, right: 
                        Box::new(right), 
                    };
                }
                _ => break,
            }
        }
        Some(left)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.advance()? {
            Token::Number(n) => Some(Expr::Number(n)),
            Token::Ident(name) => Some(Expr::Identifier(name)),
            _ => None,
        }
    }
}