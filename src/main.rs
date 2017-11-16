use std::collections::HashMap;
use std::clone::Clone;
use std::result::Result;

type Env = HashMap<String, Value>;

#[derive(PartialEq, Debug, Clone)]
enum Expr {
    Bool(bool),
    Num(i64),
    //Float(f64),
    Binop { op: String, l: Box<Expr>, r: Box<Expr> },
    If { c: Box<Expr>, t: Box<Expr>, f: Box<Expr> },
    Varref { name: String },
    App { fun: Box<Expr>, args: Vec<Expr> },
    Fundef { params: Vec<String>, body: Box<Expr> },
}

#[derive(PartialEq, Debug, Clone)]
enum Value {
    Bool(bool),
    Num(i64),
    Closure { params: Vec<String>, body: Expr, env: Env },
}

#[test]
fn sample_ast() {
    // This is the AST for
    // {{lam {x}
    //        {if {<= x 5}
    //            true
    //            1}}
    //  4}
    let prog: Expr = Expr::App {
        fun: Box::new(Expr::Fundef {
            params: vec!(String::from("x")),
            body: Box::new(Expr::If {
                c: Box::new(Expr::Binop{
                    op: String::from("<="),
                    l: Box::new(Expr::Varref{
                        name: String::from("x"),
                    }),
                    r: Box::new(Expr::Num(5)),
                }),
                t: Box::new(Expr::Bool(true)),
                f: Box::new(Expr::Num(1)),
            }),
        }),
        args: vec!(Expr::Num(4)),
    };
    assert_eq!(interp(prog, &Env::new()), Value::Bool(true));
}



fn eval_binop(op: String, l: Value, r: Value) -> Value {

    let left =     
        match l {
            Value::Num(n) => n,
            _ => 99999,
        };

    let right =     
        match r {
            Value::Num(n) => n,
            _ => 99999,
        };

    // do something with the 99999 error

    match op.as_ref() {
        "+" => Value::Num(left + right),
        "-" => Value::Num(left - right),
        "/" => Value::Num(left / right),
        "*" => Value::Num(left * right),
        "<=" => Value::Bool(left <= right),
        //not implemented
        _ => Value::Num(-1),
    }
}

// Interpret a UIRE expression
fn interp(exp: Expr, env: &Env) -> Result<Value, &'static str> {
    match exp {
        Expr::Num(n) => Ok(Value::Num(n)),
        Expr::Bool(b) => Ok(Value::Bool(b)),
        //Expr::Float(f) => Value::Float(f),
        Expr::Binop{op,l,r} =>
            Ok(eval_binop(op, try!(interp(*l, env)), try!(interp(*r, env)))),
        Expr::If{c,t,f} => match try!(interp(*c, env)) {
            Value::Bool(true) => interp(*t, env),
            Value::Bool(false) => interp(*f, env),
            _ => Err("If conditions must be booleans"),
        },
        Expr::Varref{name} => match env.get(&name) {
            Some(val) => Ok((*val).clone()),
            None => Err("Undefined variable reference"),
        },

        Expr::Fundef{params, body} => Ok(Value::Closure {
            params: params,
            body: (*body).clone(),
            env: env.clone(),
        }),
        Expr::App{fun, args} => match try!(interp(*fun, env)) {
            Value::Closure {params, body, env: c_env} => {
                if args.len() != params.len() {
                    return Err("Incorrect number of arguments");
                }
                let mut new_env = c_env.clone();
                for i in 0..args.len() {
                    new_env.insert(params[i].clone(),
                                   try!(interp(args[i].clone(), env)));
                }
                interp(body, &new_env)
            },
            _ => Err("Tried to call a non-function as a function"),
        },

    }
}

#[test]
fn test_prims() {
    assert_eq!(interp(Expr::Num(5), &(Env::new())), Value::Num(5));
    assert_eq!(interp(Expr::Bool(true), &(Env::new())), Value::Bool(true));
    // assert_eq!(interp(Expr::Float(3.14)), Value::Float(3.14));

}


fn serialize(val: Result<Value, &'static str>) -> String {
    match val {
        Ok(Value::Num(n)) => n.to_string(),
        Ok(Value::Bool(b)) => b.to_string(),
        Ok(_) => String::from("#<procedure>"),
        Err(err) => format!("UIRE: {}", err),
    }
}

#[test]
fn test_serialize() {
    assert_eq!(serialize(Value::Num(-7)), "-7");
    //  assert_eq!(serialize(Value::Float(1.1)), "1.1");
    assert_eq!(serialize(Value::Bool(true)), "true");
    assert_eq!(serialize(Value::Bool(false)), "false");
    assert_eq!(serialize(interp(Expr::Binop{
        op: String::from("*"),
        l: Box::new(Expr::Num(20),),
        r: Box::new(Expr::Num(5)), }, &(Env::new()))), "100");
    assert_eq!(serialize(interp(Expr::Binop{
        op: String::from("+"),
        l: Box::new(Expr::Num(11),),
        r: Box::new(Expr::Num(7)),}, &(Env::new()))), "18");

    assert_eq!(serialize(interp(Expr::Binop{
        op: String::from("-"),
        l: Box::new(Expr::Num(11),),
        r: Box::new(Expr::Binop{
            op: String::from("/"),
            l: Box::new(Expr::Num(20),),
            r: Box::new(Expr::Num(4)),}),}, &(Env::new()))), "6");

    assert_eq!(serialize(interp(Expr::Binop{
        op: String::from("<="),
        l: Box::new(Expr::Num(200),),
        r: Box::new(Expr::Num(10)),}, &(Env::new()))), "false");

    assert_eq!(serialize(interp(Expr::If{
        c: Box::new(Expr::Binop{
            op: String::from("<="),
            l: Box::new(Expr::Num(200),),
            r: Box::new(Expr::Num(10)),}),
            t: Box::new(Expr::Num(200),),
            f: Box::new(Expr::Num(10)),}, &(Env::new()))), "10");

    assert_eq!(serialize(interp(Expr::If{
        c: Box::new(Expr::Binop{
            op: String::from("<="),
            l: Box::new(Expr::Num(10),),
            r: Box::new(Expr::Num(100)),}),
            t: Box::new(Expr::Binop{
                op: String::from("/"),
                l: Box::new(Expr::Num(20),),
                r: Box::new(Expr::Num(4)),},),
                f: Box::new(Expr::Num(10)),}, &(Env::new()))), "5");

}


fn main() {
    println!("{}", serialize(interp(Expr::Num(5), &(Env::new()))));

    let test_bin : Expr = Expr::Binop{
        op: String::from("+"),
        l: Box::new(Expr::Num(100),),
        r: Box::new(Expr::Num(5)),};

    println!("{}", serialize(interp(test_bin, &(Env::new()))));

    println!("{}", serialize(interp(Expr::Varref {
        name: String::from("x"),
    }, &Env::new())))
}
