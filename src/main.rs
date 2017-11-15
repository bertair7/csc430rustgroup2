use std::collections::HashMap;

#[derive(PartialEq, Debug)]
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


#[derive(PartialEq, Debug)]
enum Value {
    Bool(bool),
    Num(i64),
    //Float(f64),
}

type Env = HashMap<String, Value>;

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
}



fn evalBinop(op: String, l: Value, r: Value) -> Value {

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
fn interp(exp: Expr, env: &Env) -> Value {
    match exp {
        Expr::Num(n) => Value::Num(n),
        Expr::Bool(b) => Value::Bool(b),
        //Expr::Float(f) => Value::Float(f),
        Expr::Binop{op,l,r} => evalBinop(op, (interp (*l)), (interp (*r))),
        Expr::If{c,t,f} => if (interp(*c) == Value::Bool(true)) { interp(*t)} else {interp(*f)},
  	Expr::Varref{name} => match env.get(name) {
		Some(val) => val,
		None => Value::Num(-1)}
        // TODO
        _ => Value::Num(-1),
    }
}

#[test]
fn test_prims() {
    assert_eq!(interp(Expr::Num(5), &(Env::new())), Value::Num(5));
    assert_eq!(interp(Expr::Bool(true), &(Env::new())), Value::Bool(true));
   // assert_eq!(interp(Expr::Float(3.14)), Value::Float(3.14));

}


fn serialize(val: Value) -> String {
    match val {
        Value::Num(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
      //  Value::Float(f) => f.to_string(),
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
                    r: Box::new(Expr::Num(5)), }), &(Env::new())), "00");
    assert_eq!(serialize(interp(Expr::Binop{
                    op: String::from("+"),
                    l: Box::new(Expr::Num(11),),
                    r: Box::new(Expr::Num(7)),}), &(Env::new())), "18");

    assert_eq!(serialize(interp(Expr::Binop{
                    op: String::from("-"),
                    l: Box::new(Expr::Num(11),),
                    r: Box::new(Expr::Binop{
                        op: String::from("/"),
                        l: Box::new(Expr::Num(20),),
                        r: Box::new(Expr::Num(4)),}),})), "6");

     assert_eq!(serialize(interp(Expr::Binop{
                    op: String::from("<="),
                    l: Box::new(Expr::Num(200),),
                    r: Box::new(Expr::Num(10)),})), "false");

    assert_eq!(serialize(interp(Expr::If{
                    c: Box::new(Expr::Binop{
                        op: String::from("<="),
                        l: Box::new(Expr::Num(200),),
                        r: Box::new(Expr::Num(10)),}),
                    t: Box::new(Expr::Num(200),),
                    f: Box::new(Expr::Num(10)),})), "10");

    assert_eq!(serialize(interp(Expr::If{
                    c: Box::new(Expr::Binop{
                        op: String::from("<="),
                        l: Box::new(Expr::Num(10),),
                        r: Box::new(Expr::Num(100)),}),
                    t: Box::new(Expr::Binop{
                        op: String::from("/"),
                        l: Box::new(Expr::Num(20),),
                        r: Box::new(Expr::Num(4)),},),
                    f: Box::new(Expr::Num(10)),})), "5");

}


fn main() {
    println!("{}", serialize(interp(Expr::Num(5))));

    let testBin : Expr = Expr::Binop{
                    op: String::from("+"),
                    l: Box::new(Expr::Num(100),),
                    r: Box::new(Expr::Num(5)),};

    println!("{}", serialize(interp(testBin)));
}
