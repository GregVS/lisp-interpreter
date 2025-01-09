use std::ops::Deref;

#[derive(Debug, Copy, Clone)]
pub struct MyFloat(pub f64);

impl PartialEq for MyFloat {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 0.000000000000001
    }
}

impl Eq for MyFloat {}

impl Deref for MyFloat {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Atom {
    Integer(i32),
    Float(MyFloat),
    Symbol(String),
    String(String),
    T,
    Nil,
}


#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    LParen,
    RParen,
    Atom(Atom),
}

pub fn tokenize_expr(line: &str) -> Vec<Token> {
    let new_str = line.replace('(', " ( ").replace(')', " ) ").replace(";;", " ;; ");
    let mut word_iter = new_str.split_whitespace();

    let mut tokens: Vec<Token> = Vec::new();
    let mut quoted_depths = Vec::new();

    while let Some(w) = word_iter.next() {
        match w {
            ";;" => { while let Some(_) = word_iter.next() {} }
            "(" => {
                tokens.push(Token::LParen);
                if !quoted_depths.is_empty() {
                    *quoted_depths.last_mut().unwrap() += 1;
                }
            }
            ")" => {
                tokens.push(Token::RParen);
                if !quoted_depths.is_empty() {
                    *quoted_depths.last_mut().unwrap() -= 1;
                }
                if !quoted_depths.is_empty() && quoted_depths[quoted_depths.len() - 1] == 0 {
                    quoted_depths.pop();
                    tokens.push(Token::RParen);
                }
            }
            "T" | "t" => { tokens.push(Token::Atom(Atom::T)); }
            "NIL" | "Nil" | "nil" => { tokens.push(Token::Atom(Atom::Nil)); }
            "'" => {
                tokens.push(Token::LParen);
                tokens.push(Token::Atom(Atom::Symbol("quote".to_string())));
                quoted_depths.push(0);
            }
            _ => {
                if !quoted_depths.is_empty() && quoted_depths[quoted_depths.len() - 1] == 0 {
                    tokens.push(Token::Atom(Atom::Symbol(w.to_lowercase().to_string())));
                    tokens.push(Token::RParen);
                    quoted_depths.pop();
                } else if let Ok(n) = w.parse::<i32>() {
                    tokens.push(Token::Atom(Atom::Integer(n)));
                } else if let Ok(n) = w.parse::<f64>() {
                    tokens.push(Token::Atom(Atom::Float(MyFloat(n))));
                } else if w.starts_with("'") {
                    tokens.push(Token::LParen);
                    tokens.push(Token::Atom(Atom::Symbol("quote".to_string())));
                    tokens.push(Token::Atom(Atom::Symbol(w[1..].to_lowercase().to_string())));
                    tokens.push(Token::RParen);
                } else if w.starts_with('"') && w.ends_with('"') {
                    tokens.push(Token::Atom(Atom::String(w.trim_matches('"').to_string())));
                } else {
                    tokens.push(Token::Atom(Atom::Symbol(w.to_lowercase().to_string())));
                }
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_simple() {
        let tokens = tokenize_expr("(+ 1 2)");
        assert_eq!(tokens, vec![
            Token::LParen,
            Token::Atom(Atom::Symbol("+".to_string())),
            Token::Atom(Atom::Integer(1)),
            Token::Atom(Atom::Integer(2)),
            Token::RParen,
        ]);
    }

    #[test]
    fn tokenize_nested() {
        let tokens = tokenize_expr("(+ 1 (* 2 3)) ;;comment");
        assert_eq!(tokens, vec![
            Token::LParen,
            Token::Atom(Atom::Symbol("+".to_string())),
            Token::Atom(Atom::Integer(1)),
            Token::LParen,
            Token::Atom(Atom::Symbol("*".to_string())),
            Token::Atom(Atom::Integer(2)),
            Token::Atom(Atom::Integer(3)),
            Token::RParen,
            Token::RParen,
        ]);
    }

    #[test]
    fn tokenize_quoted() {
        let tokens = tokenize_expr("'a");
        assert_eq!(tokens, vec![
            Token::LParen,
            Token::Atom(Atom::Symbol(String::from("quote"))),
            Token::Atom(Atom::Symbol(String::from("a"))),
            Token::RParen,
        ]);

        let tokens = tokenize_expr("'(1 2.5)");
        assert_eq!(tokens, vec![
            Token::LParen,
            Token::Atom(Atom::Symbol(String::from("quote"))),
            Token::LParen,
            Token::Atom(Atom::Integer(1)),
            Token::Atom(Atom::Float(MyFloat(2.5))),
            Token::RParen,
            Token::RParen,
        ]);
    }

    #[test]
    fn tokenize_double_quote() {
        let tokens = tokenize_expr("'(car '(1))");
        assert_eq!(tokens, vec![
            Token::LParen,
            Token::Atom(Atom::Symbol(String::from("quote"))),
            Token::LParen,
            Token::Atom(Atom::Symbol(String::from("car"))),
            Token::LParen,
            Token::Atom(Atom::Symbol(String::from("quote"))),
            Token::LParen,
            Token::Atom(Atom::Integer(1)),
            Token::RParen,
            Token::RParen,
            Token::RParen,
            Token::RParen,
        ]);
    }
}