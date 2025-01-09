use crate::functions::*;
use crate::parser::Object;
use crate::tokenizer::Atom;

#[derive(Debug, Clone, PartialEq)]
pub struct Stack {
    stack: Vec<(Atom, Object)>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { stack: vec![] }
    }

    pub fn push(&mut self, atom: Atom, object: Object) {
        self.stack.push((atom, object));
    }

    pub fn find(&self, atom: &Atom) -> Option<Object> {
        self.stack.iter().rev().find(|(s, _)| s == atom).map(|(_, o)| o.clone())
    }
}

pub fn eval(expression: &Object, stack: &mut Stack) -> Object {
    match expression {
        Object::Atom(atom) => { eval_atom(atom, stack) }
        Object::List(list) => { eval_list(list, stack) }
        _ => panic!("Cannot eval improper list")
    }
}

fn eval_atom(atom: &Atom, stack: &mut Stack) -> Object {
    match atom {
        Atom::Nil => { Object::Atom(atom.clone()) }
        Atom::T => { Object::Atom(atom.clone()) }
        Atom::String(_) => { Object::Atom(atom.clone()) }
        Atom::Integer(_) => { Object::Atom(atom.clone()) }
        Atom::Float(_) => { Object::Atom(atom.clone()) }
        Atom::Symbol(_) => { stack.find(atom).unwrap() }
    }
}

fn eval_list(list: &Vec<Object>, stack: &mut Stack) -> Object {
    let mut list_iter = list.iter();
    if let Some(first) = list_iter.next() {
        let fn_symbol = match first {
            Object::Atom(atom) => {
                match atom {
                    Atom::Symbol(symbol) => { &symbol[..] }
                    _ => { panic!("First element of list must be a symbol") }
                }
            }
            _ => { panic!("First element of list must be an atom") }
        };

        match fn_symbol {
            "null" => { fn_null(&eval(list_iter.next().unwrap(), stack)) }
            "quote" => { list_iter.next().unwrap().clone() }
            "car" => { fn_car(&eval(list_iter.next().unwrap(), stack)) }
            "cdr" => { fn_cdr(&eval(list_iter.next().unwrap(), stack)) }
            "cons" => { fn_cons(&eval(list_iter.next().unwrap(), stack), &eval(list_iter.next().unwrap(), stack)) }
            "print" => { fn_print(&eval(list_iter.next().unwrap(), stack)) }
            "atom" => { fn_atom(&eval(list_iter.next().unwrap(), stack)) }
            "listp" => { fn_listp(&eval(list_iter.next().unwrap(), stack)) }
            "setq" => { fn_setq(list_iter.next().unwrap(), &eval(list_iter.next().unwrap(), stack), stack) }
            "defun" => { fn_defun(list_iter.next().unwrap(), list_iter.next().unwrap(), &Object::List(list_iter.cloned().collect()), stack) }
            "cond" => { fn_cond(&Object::List(list_iter.cloned().collect()), stack) }
            "eq" => { fn_eq(&eval(list_iter.next().unwrap(), stack), &eval(list_iter.next().unwrap(), stack)) }
            "eval" => { eval(&eval(list_iter.next().unwrap(), stack), stack) }
            "equal" => { fn_equal(&eval(list_iter.next().unwrap(), stack), &eval(list_iter.next().unwrap(), stack)) }
            "+" => { fn_add(&list_iter.map(|o| eval(o, stack)).collect()) }
            "-" => { fn_subtract(&list_iter.map(|o| eval(o, stack)).collect()) }
            "*" => { fn_multiply(&list_iter.map(|o| eval(o, stack)).collect()) }
            "/" => { fn_divide(&list_iter.map(|o| eval(o, stack)).collect()) }
            "mod" => { fn_mod(&eval(list_iter.next().unwrap(), stack), &eval(list_iter.next().unwrap(), stack)) }
            "floor" => { fn_floor(&eval(list_iter.next().unwrap(), stack), &eval(list_iter.next().unwrap_or(&Object::Atom(Atom::Integer(1))), stack)) }
            "apply" => { fn_apply(&eval(list_iter.next().unwrap(), stack), &eval(list_iter.next().unwrap(), stack), stack) }
            "load" => { fn_load(&eval(list_iter.next().unwrap(), stack), stack) }
            "and" => { fn_and(&Object::List(list_iter.cloned().collect()), stack) }
            "<=" => { fn_less_than_or_equal(&eval(list_iter.next().unwrap(), stack), &eval(list_iter.next().unwrap(), stack)) }
            ">=" => { fn_greater_than_or_equal(&eval(list_iter.next().unwrap(), stack), &eval(list_iter.next().unwrap(), stack)) }
            ">" => { fn_greater_than(&eval(list_iter.next().unwrap(), stack), &eval(list_iter.next().unwrap(), stack)) }
            "<" => { fn_less_than(&eval(list_iter.next().unwrap(), stack), &eval(list_iter.next().unwrap(), stack)) }
            _ => { fn_apply_user(first, &Object::List(list_iter.cloned().collect()), stack) }
        }
    } else {
        Object::Atom(Atom::Nil)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{ConsCell, parse};
    use crate::tokenizer::{MyFloat, tokenize_expr};

    use super::*;

    fn expr(s: &str) -> Object {
        parse(&mut tokenize_expr(s).iter().peekable())
    }

    #[test]
    fn test_eval_quote() {
        let expr = parse(&mut tokenize_expr("(quote (1 2))").iter().peekable());
        let result = eval(&expr, &mut Stack::new());
        let expected = Object::List(vec![
            Object::Atom(Atom::Integer(1)),
            Object::Atom(Atom::Integer(2)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_eval_null() {
        let expr = parse(&mut tokenize_expr("(null '(1 2))").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::Nil));

        let expr = parse(&mut tokenize_expr("(null nil)").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::T));
    }

    #[test]
    fn test_eval_atom() {
        let expr = parse(&mut tokenize_expr("5").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::Integer(5)));
    }

    #[test]
    fn test_eval_car() {
        let expr = parse(&mut tokenize_expr("(car nil)").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::Nil));

        let expr = parse(&mut tokenize_expr("(car ())").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::Nil));

        let expr = parse(&mut tokenize_expr("(car '(5))").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::Integer(5)));
    }

    #[test]
    fn test_eval_cdr() {
        let expr = parse(&mut tokenize_expr("(cdr nil)").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::Nil));

        let expr = parse(&mut tokenize_expr("(cdr '(1 2))").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::List(vec![Object::Atom(Atom::Integer(2))]));

        let expr = parse(&mut tokenize_expr("(null (cdr '(1)))").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::T));
    }

    #[test]
    fn test_eval_print() {
        let expr = parse(&mut tokenize_expr("(print '(1 2))").iter().peekable());
        let result = eval(&expr, &mut Stack::new());
        if let Object::Atom(Atom::String(s)) = result {
            assert_eq!(s, "(1 2)");
        } else {
            panic!("Expected string");
        }

        let expr = parse(&mut tokenize_expr("(print 'foo)").iter().peekable());
        let result = eval(&expr, &mut Stack::new());
        if let Object::Atom(Atom::String(s)) = result {
            assert_eq!(s, "foo");
        } else {
            panic!("Expected string");
        }
    }

    #[test]
    fn test_eval_fn_atom() {
        let expr = parse(&mut tokenize_expr("(atom '(1 2))").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::Nil));

        let expr = parse(&mut tokenize_expr("(atom 100)").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::T));
    }

    #[test]
    fn test_eval_listp() {
        let expr = parse(&mut tokenize_expr("(listp '(1 2))").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::T));

        let expr = parse(&mut tokenize_expr("(listp 100)").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::Nil));
    }

    #[test]
    fn test_setq() {
        let mut stack = Stack::new();
        let expr = parse(&mut tokenize_expr("(setq x 5)").iter().peekable());
        assert_eq!(eval(&expr, &mut stack), Object::Atom(Atom::Integer(5)));
        let expr = parse(&mut tokenize_expr("x").iter().peekable());
        assert_eq!(eval(&expr, &mut stack), Object::Atom(Atom::Integer(5)));
    }

    #[test]
    fn test_defun() {
        let mut stack = Stack::new();
        let expr = parse(&mut tokenize_expr("(defun join (x y) (print y) (cons x y))").iter().peekable());
        assert_eq!(eval(&expr, &mut stack), Object::Atom(Atom::Symbol(String::from("join"))));
        let expr = parse(&mut tokenize_expr("(join (quote a) 5)").iter().peekable());
        assert_eq!(eval(&expr, &mut stack), Object::ConsCell(Box::new(ConsCell::new(Object::Atom(Atom::Symbol(String::from("a"))), Object::Atom(Atom::Integer(5))))));
    }

    #[test]
    fn test_cond() {
        let expr = parse(&mut tokenize_expr("(cond ((null 5) T) (T Nil))").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::Nil));
        let expr = parse(&mut tokenize_expr("(cond ((null ()) T) (T Nil))").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::T));
    }

    #[test]
    fn test_eq() {
        let expr = parse(&mut tokenize_expr("(eq 'a 'b)").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::Nil));
        let expr = parse(&mut tokenize_expr("(eq 'a 'a)").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::T));
        let expr = parse(&mut tokenize_expr("(eq nil nil)").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::T));
        let expr = parse(&mut tokenize_expr("(eq T T)").iter().peekable());
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::T));
    }

    #[test]
    fn test_eval() {
        let expr = expr("(eval '(car '(1 2)))");
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::Integer(1)));
    }

    #[test]
    fn test_equal() {
        let expr = expr("(equal '(1 2) '(1 2))");
        assert_eq!(eval(&expr, &mut Stack::new()), Object::Atom(Atom::T));
    }

    #[test]
    fn test_greater_than() {
        let test_expr = expr("(> 5 4)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::T));
        let test_expr = expr("(> 4 5)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Nil));
    }

    #[test]
    fn test_less_than() {
        let test_expr = expr("(< 4 5)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::T));
        let test_expr = expr("(< 5 4)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Nil));
    }

    #[test]
    fn test_less_than_or_equal() {
        let test_expr = expr("(<= 3 4)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::T));
        let test_expr = expr("(<= 4 4)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::T));
        let test_expr = expr("(<= 5 4)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Nil));
    }

    #[test]
    fn test_greater_than_or_equal() {
        let test_expr = expr("(>= 5 4)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::T));
        let test_expr = expr("(>= 4 4)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::T));
        let test_expr = expr("(>= 3 4)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Nil));
    }

    #[test]
    fn test_add() {
        let test_expr = expr("(+ 1 2)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Float(MyFloat(3.0))));
        let test_expr = expr("(+ 1 2.5)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Float(MyFloat(3.5))));
    }

    #[test]
    fn test_sub() {
        let test_expr = expr("(- 1 2)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Float(MyFloat(-1.0))));
        let test_expr = expr("(- 1 0.5 0.25)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Float(MyFloat(0.25))));
    }

    #[test]
    fn test_multiply() {
        let test_expr = expr("(* 1 2)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Float(MyFloat(2.0))));
    }

    #[test]
    fn test_divide() {
        let test_expr = expr("(/ 1 2)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Float(MyFloat(0.5))));
    }

    #[test]
    fn test_mod() {
        let test_expr = expr("(mod 1 2)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Integer(1)));
        let test_expr = expr("(mod 5 2)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Integer(1)));
    }

    #[test]
    fn test_floor() {
        let test_expr = expr("(floor 1 2)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Integer(0)));
        let test_expr = expr("(floor 3.2)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Integer(3)));
        let test_expr = expr("(floor 5 3)");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Integer(1)));
    }

    #[test]
    fn test_apply() {
        let test_expr = expr("(apply '+ '(1 2))");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::Float(MyFloat(3.0))));
    }

    #[test]
    fn test_load() {
        let test_expr = expr("(load \"test.l\")");
        assert_eq!(eval(&test_expr, &mut Stack::new()), Object::Atom(Atom::T));
    }
}