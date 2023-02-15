use std::env;

pub mod file_utils;
pub mod lexer;
pub mod lexer_result;
pub mod symbols;
pub mod token;
pub mod types;

use lexer::Lexer;

fn main() {
    let file_path = env::args().nth(1);

    if file_path == None {
        panic!("A path is required to parse");
    }

    let file_path = file_path.unwrap();

    let file_content = file_utils::get_file_content(file_path);

    let lexer_result_tokens = Lexer::new(file_content.unwrap()).lex().tokens;

    for token in lexer_result_tokens {
        println!("{:?}", token);
    }
}
