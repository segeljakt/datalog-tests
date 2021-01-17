#![allow(unused)]

use std::collections::HashSet;

mod print;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct Expr(usize);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct Type(usize);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum ExprKind {
    Let(Expr, Expr),
    I32(i32),
    U32(u32),
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum TypeKind {
    I32,
    U32,
    Bool,
}

use crepe::crepe;

crepe! {
    @input
    #[derive(Debug)]
    struct ExprOf(Expr, ExprKind);

    @output
    #[derive(Debug)]
    struct TypeOf(Expr, TypeKind);

    // typeof(let x = e0 in e1)
    TypeOf(e0, tk0) <-
        ExprOf(e0, ek0),
        let ExprKind::Let(_, e1) = ek0,
        TypeOf(e1, tk0);

    // typeof(100i32)
    TypeOf(e0, TypeKind::I32) <-
        ExprOf(e0, ek0),
        let ExprKind::I32(_) = ek0;

    // typeof(100u32)
    TypeOf(e0, TypeKind::U32) <-
        ExprOf(e0, ek0),
        let ExprKind::U32(_) = ek0;
}

#[derive(Default, Debug)]
pub(crate) struct ExprMap {
    pub(crate) vec: Vec<ExprKind>,
}

impl ExprMap {
    pub(crate) fn new(&mut self, expr: ExprKind) -> Expr {
        let id = Expr(self.vec.len());
        self.vec.push(expr);
        id
    }
    pub(crate) fn get(&self, id: Expr) -> &ExprKind {
        self.vec.get(id.0).unwrap()
    }
}

fn typecheck(exprs: ExprMap) {
    let mut runtime = Crepe::new();

    let inputs = exprs
        .vec
        .iter()
        .enumerate()
        .map(|(i, &e)| ExprOf(Expr(i), e))
        .collect::<HashSet<_>>();

    println!("Inputs: {{");
    for x in &inputs {
        println!("    {:?}", x);
    }
    println!("}}");
    println!();

    runtime.extend(inputs);

    let (outputs,) = runtime.run();

    println!("Outputs: {{");
    for x in outputs {
        println!("    {:?}", x);
    }
    println!("}}");
}

fn test() {
    let mut exprs = ExprMap::default();

    // let x = 50 + 100 in x == 150

    let e0 = exprs.new(ExprKind::I32(50));
    let e1 = exprs.new(ExprKind::U32(150));
    let e6 = exprs.new(ExprKind::Let(e0, e1));

    exprs.print(e6);

    typecheck(exprs);
}

fn main() {
    for (i, test) in [test].iter().enumerate() {
        println!("===============[Test {}]===============", i);
        test();
        println!();
    }
}
