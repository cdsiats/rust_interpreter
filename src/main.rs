use lexer::Lexer;
use tokens::Token;

mod tokens;
mod lexer;
fn main() {
    let input = "let x = 42 + 8";
    let mut lexer = Lexer::new(input.to_string());

    loop {
        let tok = lexer.next_token();
        println!("{:?}", tok);
        if tok == Token::EOF {
            break;
        }
    }
}
