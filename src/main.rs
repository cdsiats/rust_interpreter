use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;
use tokens::Token;

mod tokens;
mod lexer;
mod ast;
mod parser;
mod evaluator;
fn main() {
    let input = "let x = 255 + 300 * 3";
    let mut lexer = Lexer::new(input.to_string());

    let mut tokens = vec![];
    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());
        if token == Token::EOF {
            break;
        }
    }

    let mut parser = Parser::new(tokens);
    let statement = parser.parse_statement().unwrap();

    let mut evaluator = Evaluator::new();
    evaluator.eval_statement(&statement);

    println!("Final environment: {:?}", evaluator.env);
}
