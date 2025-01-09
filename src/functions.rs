use crate::interpreter::{eval, Stack};
use crate::parser::{ConsCell, Object};
use crate::tokenizer;
use crate::tokenizer::{Atom, MyFloat, Token};

pub fn fn_less_than(a: &Object, b: &Object) -> Object {
    let a = match a {
        Object::Atom(Atom::Integer(number)) => *number as f64,
        Object::Atom(Atom::Float(MyFloat(number))) => *number,
        _ => panic!("Cannot compare non-number")
    };

    let b = match b {
        Object::Atom(Atom::Integer(number)) => *number as f64,
        Object::Atom(Atom::Float(MyFloat(number))) => *number,
        _ => panic!("Cannot compare non-number")
    };

    if a < b {
        Object::Atom(Atom::T)
    } else {
        Object::Atom(Atom::Nil)
    }
}

pub fn fn_greater_than(a: &Object, b: &Object) -> Object {
    let a = match a {
        Object::Atom(Atom::Integer(number)) => *number as f64,
        Object::Atom(Atom::Float(MyFloat(number))) => *number,
        _ => panic!("Cannot compare non-number")
    };

    let b = match b {
        Object::Atom(Atom::Integer(number)) => *number as f64,
        Object::Atom(Atom::Float(MyFloat(number))) => *number,
        _ => panic!("Cannot compare non-number")
    };

    if a > b {
        Object::Atom(Atom::T)
    } else {
        Object::Atom(Atom::Nil)
    }
}

pub fn fn_and(expressions: &Object, stack: &mut Stack) -> Object {
    let expressions_list = match expressions {
        Object::List(list) => list,
        _ => panic!("AND requires a list")
    };

    for expr in expressions_list {
        if eval(expr, stack) == Object::Atom(Atom::Nil) {
            return Object::Atom(Atom::Nil);
        }
    }
    Object::Atom(Atom::T)
}

pub fn fn_load(filename: &Object, stack: &mut Stack) -> Object {
    if let Object::Atom(Atom::String(filename)) = filename {
        let contents = std::fs::read_to_string(filename).unwrap();
        let tokens = tokenizer::tokenize_expr(&contents);
        let tokens = vec![]
            .into_iter()
            .chain(vec![Token::LParen])
            .chain(tokens.into_iter())
            .chain(vec![Token::RParen].into_iter())
            .collect::<Vec<_>>();
        let parsed = crate::parser::parse(&mut tokens.iter().peekable());
        fn_eval_multiple(&parsed, stack);

        Object::Atom(Atom::T)
    } else {
        panic!("Load requires filename as string")
    }
}

fn int_from_obj(obj: &Object) -> i32 {
    match obj {
        Object::Atom(Atom::Integer(number)) => *number,
        Object::Atom(Atom::Float(MyFloat(number))) => *number as i32,
        _ => panic!("Cannot convert non-number to int")
    }
}

pub fn fn_floor(num: &Object, div: &Object) -> Object {
    let num = int_from_obj(num);
    let div = int_from_obj(div);
    Object::Atom(Atom::Integer(num / div))
}

pub fn fn_mod(num: &Object, m: &Object) -> Object {
    let num = int_from_obj(num);
    let m = int_from_obj(m);

    Object::Atom(Atom::Integer(num % m))
}

pub fn fn_subtract(vec: &Vec<Object>) -> Object {
    let mut result = match vec.first().unwrap() {
        Object::Atom(Atom::Integer(number)) => *number as f64,
        Object::Atom(Atom::Float(MyFloat(number))) => *number,
        _ => panic!("Cannot subtract non-number")
    };

    for item in vec.iter().skip(1) {
        match item {
            Object::Atom(Atom::Integer(number)) => result -= *number as f64,
            Object::Atom(Atom::Float(MyFloat(number))) => result -= number,
            _ => panic!("Cannot subtract non-number")
        }
    }
    Object::Atom(Atom::Float(MyFloat(result)))
}

pub fn fn_add(vec: &Vec<Object>) -> Object {
    let mut sum = 0.0;
    for item in vec {
        match item {
            Object::Atom(Atom::Integer(number)) => sum += *number as f64,
            Object::Atom(Atom::Float(MyFloat(number))) => sum += number,
            _ => panic!("Cannot add non-number")
        }
    }
    Object::Atom(Atom::Float(MyFloat(sum)))
}

pub fn fn_equal(a: &Object, b: &Object) -> Object {
    if a == b {
        Object::Atom(Atom::T)
    } else {
        Object::Atom(Atom::Nil)
    }
}

pub fn fn_eq(a: &Object, b: &Object) -> Object {
    match a {
        Object::Atom(Atom::Symbol(a_name)) => {
            match b {
                Object::Atom(Atom::Symbol(b_name)) => {
                    if a_name == b_name {
                        Object::Atom(Atom::T)
                    } else {
                        Object::Atom(Atom::Nil)
                    }
                }
                _ => Object::Atom(Atom::Nil)
            }
        }
        Object::Atom(Atom::Nil) => {
            match b {
                Object::Atom(Atom::Nil) => Object::Atom(Atom::T),
                _ => Object::Atom(Atom::Nil)
            }
        }
        Object::Atom(Atom::T) => {
            match b {
                Object::Atom(Atom::T) => Object::Atom(Atom::T),
                _ => Object::Atom(Atom::Nil)
            }
        }
        _ => Object::Atom(Atom::Nil)
    }
}

pub fn fn_cond(clauses: &Object, stack: &mut Stack) -> Object {
    let clause_list = match clauses {
        Object::List(list) => list,
        _ => panic!("Clauses is not a list")
    };
    for clause in clause_list {
        let clause_vec = match clause {
            Object::List(list) => list,
            _ => panic!("Clause is not a list")
        };

        if eval(&clause_vec[0], stack) != Object::Atom(Atom::Nil) {
            return fn_eval_multiple(&clause, stack);
        }
    }
    Object::Atom(Atom::Nil)
}

fn bind_actuals(formals: &Object, actuals: &Object, stack: &mut Stack) -> Stack {
    let formals_list = match formals {
        Object::List(list) => list,
        _ => panic!("Formals is not a list")
    };

    let actuals_list = match actuals {
        Object::List(list) => list,
        _ => panic!("Actuals is not a list")
    };

    let mut fn_stack = stack.clone();
    for (i, formal) in formals_list.iter().enumerate() {
        if let Object::Atom(Atom::Symbol(symbol)) = formal {
            let actual = &actuals_list[i];
            let evaluated_actual = eval(actual, stack);
            fn_stack.push(Atom::Symbol(symbol.clone()), evaluated_actual);
        } else {
            panic!("Non-symbol found in formals")
        }
    }
    fn_stack
}

pub fn fn_eval_multiple(expressions: &Object, stack: &mut Stack) -> Object {
    if let Object::List(list) = expressions {
        let mut last = Object::Atom(Atom::Nil);
        for expr in list {
            last = eval(expr, stack);
        }
        last
    } else {
        panic!("Eval multiple requires a list of expressions");
    }
}

pub fn fn_apply(fn_name: &Object, actuals: &Object, stack: &mut Stack) -> Object {
    if let Object::Atom(Atom::Symbol(symbol)) = fn_name {
        let mut eval_list = vec![Object::Atom(Atom::Symbol(symbol.clone()))];
        if let Object::List(list) = actuals {
            eval_list.extend(list.iter().cloned());
            eval(&Object::List(eval_list), stack)
        } else {
            panic!("Function object is not a list")
        }
    } else {
        panic!("Cannot apply a non-symbol")
    }
}

pub fn fn_apply_user(fn_name: &Object, actuals: &Object, stack: &mut Stack) -> Object {
    if let Object::Atom(Atom::Symbol(symbol)) = fn_name {
        let fn_object = stack.find(&Atom::Symbol(symbol.clone())).unwrap();
        if let Object::List(list) = fn_object {
            let mut list_iter = list.iter();
            let formals = list_iter.next().unwrap();
            let body = list_iter.next().unwrap();
            fn_eval_multiple(body, &mut bind_actuals(formals, actuals, stack))
        } else {
            panic!("Function object is not a list")
        }
    } else {
        panic!("Cannot apply a non-symbol")
    }
}

pub fn fn_defun(name: &Object, formals: &Object, body: &Object, stack: &mut Stack) -> Object {
    if let Object::Atom(Atom::Symbol(symbol)) = name {
        stack.push(Atom::Symbol(symbol.clone()), Object::List(vec![
            formals.clone(),
            body.clone(),
        ]));
        Object::Atom(Atom::Symbol(symbol.clone()))
    } else {
        panic!("Cannot defun a non-symbol")
    }
}

pub fn fn_setq(name: &Object, value: &Object, stack: &mut Stack) -> Object {
    if let Object::Atom(Atom::Symbol(symbol)) = name {
        stack.push(Atom::Symbol(symbol.clone()), value.clone());
        value.clone()
    } else {
        panic!("Cannot setq to a non-symbol")
    }
}

pub fn fn_listp(object: &Object) -> Object {
    match object {
        Object::List(_) => Object::Atom(Atom::T),
        Object::ConsCell(_) => Object::Atom(Atom::T),
        _ => Object::Atom(Atom::Nil)
    }
}

pub fn fn_atom(object: &Object) -> Object {
    match object {
        Object::Atom(_) => Object::Atom(Atom::T),
        _ => Object::Atom(Atom::Nil)
    }
}

pub fn fn_null(object: &Object) -> Object {
    match object {
        Object::List(list) => {
            if list.len() == 0 {
                Object::Atom(Atom::T)
            } else {
                Object::Atom(Atom::Nil)
            }
        }
        Object::Atom(atom) => {
            match atom {
                Atom::Nil => { Object::Atom(Atom::T) }
                _ => { Object::Atom(Atom::Nil) }
            }
        }
        Object::ConsCell(_) => Object::Atom(Atom::Nil)
    }
}

pub fn fn_car(object: &Object) -> Object {
    match object {
        Object::List(list) => {
            if list.len() == 0 {
                Object::Atom(Atom::Nil)
            } else {
                list[0].clone()
            }
        }
        Object::ConsCell(cell) => { cell.car.clone() }
        Object::Atom(Atom::Nil) => { Object::Atom(Atom::Nil) }
        Object::Atom(_) => panic!("Cannot call CAR on an atom")
    }
}

pub fn fn_cdr(object: &Object) -> Object {
    match object {
        Object::List(list) => {
            if list.len() == 0 {
                Object::Atom(Atom::Nil)
            } else {
                Object::List(list[1..].to_vec())
            }
        }
        Object::Atom(Atom::Nil) => { Object::Atom(Atom::Nil) }
        Object::ConsCell(cell) => { cell.cdr.clone() }
        _ => panic!("Invalid CDR")
    }
}

pub fn fn_cons(car: &Object, cdr: &Object) -> Object {
    match cdr {
        Object::List(list) => {
            let mut new_list = vec![car.clone()];
            new_list.extend(list.iter().cloned());
            Object::List(new_list)
        }
        Object::Atom(Atom::Nil) => {
            Object::List(vec![car.clone()])
        }
        _ => Object::ConsCell(Box::new(ConsCell::new(car.clone(), cdr.clone())))
    }
}

pub fn fn_print(object: &Object) -> Object {
    let str = fn_print_helper(object);
    println!("{}", str);
    Object::Atom(Atom::String(str))
}

fn fn_print_helper(object: &Object) -> String {
    let mut str = String::new();
    match object {
        Object::Atom(atom) => {
            match atom {
                Atom::Integer(number) => str.push_str(&number.to_string()),
                Atom::Float(number) => str.push_str(&number.to_string()),
                Atom::Symbol(name) => str.push_str(name),
                Atom::String(val) => str.push_str(val),
                Atom::T => str.push('T'),
                Atom::Nil => str.push_str("NIL")
            }
        }
        Object::List(list) => {
            str.push('(');
            for (i, item) in list.iter().enumerate() {
                str.push_str(&fn_print_helper(item));
                if i < list.len() - 1 {
                    str.push(' ');
                }
            }
            str.push(')');
        }
        Object::ConsCell(cell) => {
            str.push('(');
            str.push_str(&fn_print_helper(&cell.car));
            str.push('.');
            str.push_str(&fn_print_helper(&cell.cdr));
            str.push(')');
        }
    }
    str
}