use crate::tokens::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(String, Expr),
    Expression(Expr),
}