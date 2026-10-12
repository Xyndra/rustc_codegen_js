#![feature(register_tool)]
#![register_tool(js)]
#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Dot,
    Line { len: i32 },
    Rect(i32, i32),
    Named(&'static str),
}

#[derive(Debug)]
enum Op {
    Add,
    Neg,
}

impl Shape {
    fn area(&self) -> i32 {
        match self {
            Shape::Dot => 0,
            Shape::Line { len } => *len,
            Shape::Rect(w, h) => w * h,
            Shape::Named(s) => s.len() as i32,
        }
    }

    fn describe(&self) -> String {
        match self {
            Shape::Dot => "dot".to_string(),
            Shape::Line { len } => format!("line:{len}"),
            Shape::Rect(_, _) => "rect".to_string(),
            Shape::Named(n) if n.len() > 3 => format!("long:{n}"),
            Shape::Named(n) => format!("short:{n}"),
        }
    }
}

fn classify(s: &Shape) -> u8 {
    match s {
        Shape::Dot => 1,
        Shape::Line { .. } => 2,
        Shape::Rect(..) => 3,
        _ => 4,
    }
}

#[js::export]
pub fn run() -> String {
    let mut s = String::new();
    let shapes = [
        Shape::Dot,
        Shape::Line { len: 5 },
        Shape::Rect(3, 4),
        Shape::Named("square"),
        Shape::Named("hi"),
    ];
    for sh in shapes.iter() {
        s.push_str(&format!("{:?} {} {} ", sh, sh.area(), sh.describe()));
    }
    for sh in shapes.iter() {
        s.push_str(&format!("{} ", classify(sh)));
    }
    // match on Option/Result with bindings
    let opts = [Some(1), None, Some(3)];
    for o in opts.iter() {
        match o {
            Some(x) if *x > 2 => s.push_str(&format!("big{x} ")),
            Some(_) => s.push_str("small "),
            None => s.push_str("none "),
        }
    }
    let rs: Vec<Result<i32, &str>> = vec![Ok(7), Err("bad")];
    for r in rs.iter() {
        let t = match r {
            Ok(v) => format!("ok{v}"),
            Err(e) => format!("err{e}"),
        };
        s.push_str(&format!("{t} "));
    }
    // while-let / if-let / let-else
    let mut stack: Vec<Shape> = vec![Shape::Rect(2, 2), Shape::Dot];
    while let Some(sh) = stack.pop() {
        s.push_str(&format!("{} ", sh.area()));
    }
    if let Some(v) = opts[0] {
        s.push_str(&format!("iflet{v} "));
    }
    let Shape::Rect(w, h) = shapes[2] else {
        return s;
    };
    s.push_str(&format!("letelse,w{w}h{h} "));
    // nested match returning tuples, and matching on chars/bools/strings
    let pairs = [(Op::Add, 3), (Op::Neg, -3)];
    for (op, n) in pairs.iter() {
        let r = match op {
            Op::Add if *n < 0 => -(*n),
            Op::Add => *n,
            Op::Neg => -(*n),
        };
        s.push_str(&format!("{r} "));
    }
    for c in "aβz".chars() {
        s.push_str(&format!("{} ", c.is_lowercase()));
    }
    s.push_str(&format!("{:?} ", "hi".matches('i').count()));
    s
}
