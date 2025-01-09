use crate::functions::fn_print;
use crate::interpreter::eval;

mod parser;
mod tokenizer;
mod interpreter;
mod functions;

fn main() {
    let mut stack = interpreter::Stack::new();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let tokens = tokenizer::tokenize_expr(&input);
        let parsed = parser::parse(&mut tokens.iter().peekable());
        fn_print(&eval(&parsed, &mut stack));
    }
}