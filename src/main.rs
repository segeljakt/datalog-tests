#![allow(unused)]

mod print;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct Name(usize);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct Expr(usize);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct Type(usize);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum ExprKind {
    Let(Name, Expr, Expr),
    Var(Name),
    Add(Expr, Expr),
    Equ(Expr, Expr),
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

    struct Bind(Name, Expr);

    @output
    #[derive(Debug)]
    struct TypeOf(Expr, TypeKind);

    @output
    #[derive(Debug)]
    struct TypeError(Expr);

    // Bind n0 to e0
    Bind(x0, e0) <-
        ExprOf(e1, ek1),
        let ExprKind::Let(x0, e0, _) = ek1;

    // It's an error if an expression has no type.
    TypeError(e0) <-
        ExprOf(e0, _),
        !TypeOf(e0, _);

    // typeof(let x = e0 in e1)
    TypeOf(e0, tk0) <-
        ExprOf(e0, ek0),
        let ExprKind::Let(_, _, e1) = ek0,
        TypeOf(e1, tk0);

    // typeof(x)
    TypeOf(e0, tk0) <-
        ExprOf(e0, ek0),
        let ExprKind::Var(x0) = ek0,
        Bind(x0, e1),
        TypeOf(e1, tk0);

    // typeof(a == b)
    TypeOf(e0, TypeKind::Bool) <-
        ExprOf(e0, ek0),
        let ExprKind::Equ(e1, e2) = ek0,
        TypeOf(e1, tk0),
        TypeOf(e2, tk0);

    // typeof(a + b)
    TypeOf(e0, tk0) <-
        ExprOf(e0, ek0),
        let ExprKind::Add(e1, e2) = ek0,
        TypeOf(e1, tk0),
        TypeOf(e2, tk0);

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

#[derive(Default, Debug)]
struct NameMap {
    pub counter: usize,
}

impl NameMap {
    fn new(&mut self) -> Name {
        let id = Name(self.counter);
        self.counter += 1;
        id
    }
}

fn typecheck(exprs: ExprMap) {
    let mut runtime = Crepe::new();

    runtime.extend(
        exprs
            .vec
            .iter()
            .enumerate()
            .map(|(i, &e)| ExprOf(Expr(i), e)),
    );

    let (typings, errors) = runtime.run();

    println!();
    println!("## Typings:");
    for TypeOf(e, tk) in typings {
        print!("  (");
        exprs.print(e);
        println!("): {:?}", tk);
    }
    println!("## Errors:");
    for TypeError(e) in errors {
        print!("  (");
        exprs.print(e);
        println!("): ???");
    }
}

fn test0() {
    let mut exprs = ExprMap::default();
    let mut names = NameMap::default();

    // let x = 50i32 in x

    let e0 = exprs.new(ExprKind::I32(50));
    let n0 = names.new();
    let e1 = exprs.new(ExprKind::Var(n0));
    let e2 = exprs.new(ExprKind::Let(n0, e0, e1));

    exprs.println(e2);

    typecheck(exprs);
}

fn test1() {
    let mut exprs = ExprMap::default();
    let mut names = NameMap::default();

    // let x = 50i32 + 100i32 in x

    let e0 = exprs.new(ExprKind::I32(50));
    let e1 = exprs.new(ExprKind::I32(100));
    let e2 = exprs.new(ExprKind::Add(e0, e1));
    let n0 = names.new();
    let e3 = exprs.new(ExprKind::Var(n0));
    let e4 = exprs.new(ExprKind::Let(n0, e2, e3));

    exprs.println(e4);

    typecheck(exprs);
}

fn test2() {
    let mut exprs = ExprMap::default();
    let mut names = NameMap::default();

    // let x = 50i32 in x == 150i32

    let e0 = exprs.new(ExprKind::I32(50));
    let n0 = names.new();
    let e1 = exprs.new(ExprKind::I32(150));
    let e2 = exprs.new(ExprKind::Var(n0));
    let e3 = exprs.new(ExprKind::Equ(e1, e2));
    let e4 = exprs.new(ExprKind::Let(n0, e0, e3));

    exprs.println(e4);

    typecheck(exprs);
}

fn test3() {
    let mut exprs = ExprMap::default();
    let mut names = NameMap::default();

    // let x = 50i32 + 100i32 in x == 150i32

    let e0 = exprs.new(ExprKind::I32(50));
    let e1 = exprs.new(ExprKind::I32(100));
    let e2 = exprs.new(ExprKind::Add(e0, e1));
    let n0 = names.new();
    let e3 = exprs.new(ExprKind::I32(150));
    let e4 = exprs.new(ExprKind::Var(n0));
    let e5 = exprs.new(ExprKind::Equ(e3, e4));
    let e6 = exprs.new(ExprKind::Let(n0, e2, e5));

    exprs.println(e6);

    typecheck(exprs);
}

fn test4() {
    let mut exprs = ExprMap::default();
    let mut names = NameMap::default();

    // let x = 50i32 in 150u32

    let n0 = names.new();
    let e0 = exprs.new(ExprKind::I32(50));
    let e1 = exprs.new(ExprKind::U32(150));
    let e6 = exprs.new(ExprKind::Let(n0, e0, e1));

    exprs.println(e6);

    typecheck(exprs);
}

fn test5() {
    let mut exprs = ExprMap::default();
    let mut names = NameMap::default();

    // let x = 50 in x == 150

    let e0 = exprs.new(ExprKind::I32(50));
    let e1 = exprs.new(ExprKind::U32(150));
    let e2 = exprs.new(ExprKind::Equ(e0, e1));

    exprs.println(e2);

    typecheck(exprs);
}

fn main() {
    for (i, test) in [test0, test1, test2, test3, test4, test5]
        .iter()
        .enumerate()
    {
        println!("===============[Test {}]===============", i);
        test();
        println!();
    }
}
