mod interpreter;

use interpreter::lexer::Lexer;
use interpreter::token::{Token, TokenType};

fn main() {
    let tt = Token {
        token_type: TokenType::COMMA,
        literal: "let x = 3;".to_string(),
    };

    println!("{:?}", tt);

    let mut lexer = Lexer::new("x=3+4;");
    // println!("{}", lexer.input);
    // println!("{}", lexer.ch as char);

    println!("{:?}", lexer.next_token().token_type);
    println!("{:?}", lexer.next_token().token_type);
    println!("{:?}", lexer.next_token().token_type);
    println!("{:?}", lexer.next_token().token_type);
    println!("{:?}", lexer.next_token().token_type);
    println!("{:?}", lexer.next_token().token_type);
    println!("{:?}", lexer.next_token().token_type);
}
