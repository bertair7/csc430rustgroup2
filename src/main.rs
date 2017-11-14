#[derive(PartialEq, Debug)]
enum Expr {
    Bool(bool),
    Num(i64),
    Float(f64),
    Binop { op: String, l: Box<Expr>, r: Box<Expr> },
    If { c: Box<Expr>, t: Box<Expr>, f: Box<Expr> },
    Varref { name: String },
    App { fun: Box<Expr>, args: Vec<Expr> },
    Fundef { params: Vec<String>, body: Box<Expr> },
}


#[derive(PartialEq, Debug)]
enum Value {
    Bool(bool),
    Num(i64),
    Float(f64),
}


#[test]
fn sample_ast() {
    // This is the AST for
    // {{lam {x}
    //        {if {<= x 5}
    //            true
    //            1.0}}
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
                f: Box::new(Expr::Float(1.0)),
            }),
        }),
        args: vec!(Expr::Num(4)),
    };
}


// Interpret a UIRE expression
fn interp(exp: Expr) -> Value {
    match exp {
        Expr::Num(n) => Value::Num(n),
        Expr::Bool(b) => Value::Bool(b),
        Expr::Float(f) => Value::Float(f),
        // TODO
        _ => Value::Num(-1),
    }
}

#[test]
fn test_prims() {
    assert_eq!(interp(Expr::Num(5)), Value::Num(5));
    assert_eq!(interp(Expr::Bool(true)), Value::Bool(true));
    assert_eq!(interp(Expr::Float(3.14)), Value::Float(3.14));
}


fn serialize(val: Value) -> String {
    match val {
        Value::Num(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Float(f) => f.to_string(),
    }
}

#[test]
fn test_serialize() {
    assert_eq!(serialize(Value::Num(-7)), "-7");
    assert_eq!(serialize(Value::Float(1.1)), "1.1");
    assert_eq!(serialize(Value::Bool(true)), "true");
    assert_eq!(serialize(Value::Bool(false)), "false");
}


fn main() {
    println!("{}", serialize(interp(Expr::Num(5))));
}
