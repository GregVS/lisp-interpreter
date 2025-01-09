use crate::functions::fn_print;
use crate::functions::fn_eval_multiple;
use crate::interpreter::eval;
use crate::tokenizer::{Token};

mod parser;
mod tokenizer;
mod interpreter;
mod functions;

fn main() {
    let mut stack = interpreter::Stack::new();

    if std::env::args().len() > 1 {
        // Run using file
        let file_path = std::env::args().nth(1).unwrap();
        let file_content = std::fs::read_to_string(file_path).unwrap();
        let tokens = tokenizer::tokenize_expr(&file_content);
        let tokens = vec![]
            .into_iter()
            .chain(vec![Token::LParen])
            .chain(tokens.into_iter())
            .chain(vec![Token::RParen].into_iter())
            .collect::<Vec<_>>();
        let parsed = parser::parse(&mut tokens.iter().peekable());
        fn_eval_multiple(&parsed, &mut stack);
    } else {
        // Run interactive mode
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let tokens = tokenizer::tokenize_expr(&input);
            let parsed = parser::parse(&mut tokens.iter().peekable());
            fn_print(&eval(&parsed, &mut stack));
        }
    }
}
