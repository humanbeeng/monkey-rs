mod lexer;

use std::io::{self, Write};

use crate::lexer::TokenType;

fn main() {
    println!("Monkey Lang");
    if let Err(e) = io::stdout().flush() {
        panic!("Unable to flush stdout, {}", e);
    }

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        let mut l = lexer::Lexer::new(&input);

        loop {
            let tok = l.next_token();
            if tok.token_type == TokenType::EOF {
                break;
            } else {
                println!("{:?}", tok);
            }
        }
    }
}
