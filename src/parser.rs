use std::iter::Peekable;
use std::slice::Iter;
use crate::tokenizer::{Atom, Token};

#[derive(Debug, PartialEq, Clone)]
pub struct ConsCell {
    pub car: Object,
    pub cdr: Object,
}

impl ConsCell {
    pub fn new(car: Object, cdr: Object) -> Self {
        Self {
            car,
            cdr,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Atom(Atom),
    List(Vec<Object>),
    ConsCell(Box<ConsCell>),
}


/// Parses a single expression
pub fn parse(token_iter: &mut Peekable<Iter<Token>>) -> Object {
    if let Some(token) = token_iter.next() {
        match token {
            Token::LParen => {
                let mut list = Vec::new();
                while let Some(token) = token_iter.peek() {
                    match token {
                        Token::LParen => {
                            let inner_list = parse(token_iter);
                            list.push(inner_list);
                        }
                        Token::RParen => {
                            token_iter.next();
                            return Object::List(list);
                        }
                        Token::Atom(atom) => {
                            list.push(Object::Atom(atom.clone()));
                            token_iter.next();
                        }
                    }
                }
            }
            Token::Atom(atom) => {
                return Object::Atom(atom.clone());
            }
            Token::RParen => {
                panic!("Unexpected right paren");
            }
        }
    }

    Object::Atom(Atom::Nil)
}

#[cfg(test)]
mod test {
    use crate::parser::{Object, parse};
    use crate::tokenizer::{Atom, tokenize_expr};

    #[test]
    fn parse_simple() {
        let tokens = tokenize_expr("(+ 1 2)");
        let parsed = parse(&mut tokens.iter().peekable());
        assert_eq!(parsed, Object::List(vec![
            Object::Atom(Atom::Symbol("+".to_string())),
            Object::Atom(Atom::Integer(1)),
            Object::Atom(Atom::Integer(2)),
        ]));
    }

    #[test]
    fn parse_nested() {
        let tokens = tokenize_expr("(+ 1 (* 2 3))");
        let parsed = parse(&mut tokens.iter().peekable());
        assert_eq!(parsed, Object::List(vec![
            Object::Atom(Atom::Symbol("+".to_string())),
            Object::Atom(Atom::Integer(1)),
            Object::List(vec![
                Object::Atom(Atom::Symbol("*".to_string())),
                Object::Atom(Atom::Integer(2)),
                Object::Atom(Atom::Integer(3)),
            ]),
        ]));
    }
}