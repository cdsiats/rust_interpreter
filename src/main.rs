use lexer::Lexer;
use parser::Parser;
use tokens::Token;

mod tokens;
mod lexer;
mod ast;
mod parser;
fn main() {
    let input = "let x = 2 + 3 * 4";
    let mut lexer = Lexer::new(input.to_string());

    let mut tokens = vec![];
    while let token= lexer.next_token() {
        if token == Token::EOF {
            break;
        }
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    if let Some(statement) = parser.parse_statement() {
        println!("{:#?}", statement);
    }
}
