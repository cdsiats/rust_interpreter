#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    //Keywords
    Let,
    If,
    Else,
    While,
    Fn,

    //Identifiers and Literals
    Ident(String),
    Number(f64),

    //Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equals,

    //Special Characters
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,

    EOF,
}