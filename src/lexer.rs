use std::char;

use crate::tokens::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).cloned()
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.peek() {
            match ch {
                //Skip whitespaces
                ' ' | '\t' | '\n' => self.advance(),
                '+' => { self.advance(); return Token::Plus}
                '-' => { self.advance(); return Token::Minus}
                '*' => { self.advance(); return Token::Star}
                '/' => { self.advance(); return Token::Slash}
                '=' => { self.advance(); return Token::Equals}
                '(' => { self.advance(); return Token::OpenParen}
                ')' => { self.advance(); return Token::CloseParen}
                '{' => { self.advance(); return Token::OpenBrace}
                '}' => { self.advance(); return Token::CloseBrace}
                _ =>  {
                    if ch.is_numeric() {
                        return self.read_number();
                    } else if ch.is_alphabetic() {
                        return self.read_identifier();
                    } else {
                        panic!("Unexpected character: {}", ch);
                    }
                }
            }
        }
        Token::EOF
    }

    fn read_number(&mut self) -> Token {
        let mut num_str = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_numeric() || ch == '.' {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(num_str.parse().unwrap())
    }

    fn read_identifier(&mut self) -> Token {
        let mut id_str = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() {
                id_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        match id_str.as_str() {
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "fn" => Token::Fn,
            _ => Token::Ident(id_str),
        }
    }
}