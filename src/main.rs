mod print;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct Name(usize);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct Expr(usize);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub(crate) enum ExprKind {
    Let(Name, Expr, Expr),
    Var(Name),
    Tuple(Vec<Expr>),
    Project(Expr, usize),
    I32(i32),    // Non-linear
    Str(String), // Linear
}

// Expression which is a Path to something
pub(crate) type PathExpr = Expr;

use crepe::crepe;

crepe! {
    @input
    #[derive(Debug)]
    struct ExprOf<'i>(Expr, &'i ExprKind);

    // Checks if Expr is a PathExpr
    struct IsPathExpr(Expr);

    // An expression is a path-expression if it has an origin.
    IsPathExpr(e0) <-
        Origin(_, e0);

    // Name is the start of PathExpr, e.g. Origin(a, a.b.c)
    @output
    #[derive(Debug)]
    struct Origin(Name, PathExpr);

    // The origin of a path-variable is itself.
    Origin(x, e0) <-
        ExprOf(e0, ek0),
        let &ExprKind::Var(x) = ek0;

    // The origin of a path is the origin of its ancestor.
    Origin(x, e1) <-
        Ancestor(e0, e1),
        Origin(x, e0);

    @output
    // PathExpr is an ancestor of PathExpr
    struct Ancestor(PathExpr, PathExpr);

    Ancestor(e0, e1) <-
        ExprOf(e1, ek1),
        let &ExprKind::Project(e0, _) = ek1,
        IsPathExpr(e0);

    // PathExpr is used in Expr
    @output
    struct Used(PathExpr, Expr);

    // PathExpr is used (moved) if it occurs at the right-hand-side
    // of a let-expression (given that it is a PathExpr).
    Used(e0, e1) <-
        ExprOf(e1, ek2),
        let &ExprKind::Let(_, e0, _) = ek2,
        IsPathExpr(e0);

    // Checks if PathExpr is used
    struct IsUsed(PathExpr);

    IsUsed(e0) <- Used(e0, _);

    @output
    struct Violation(PathExpr, PathExpr);

//     Violation(e0, e1) <-
//         Origin(x, e0),
//         Origin(x, e1),
//         Used(e0, _),
//         Used(e1, _),
//         (e0 != e1);

    // A ancestor-path of a child-path cannot be used.
    Violation(e0, e1) <-
        IsUsed(e0),
        Ancestor(e0, e1);
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
            .map(|(i, e)| ExprOf(Expr(i), e)),
    );

    let (paths, ancestors, used, violations) = runtime.run();
    let inline = true;
    println!("Origins:");
    for Origin(x, e) in paths {
        print!("  x_{} [is origin of] ", x.0);
        exprs.print(e, inline);
        println!();
    }
    println!("Ancestors:");
    for Ancestor(e0, e1) in ancestors {
        print!("  ");
        exprs.print(e0, inline);
        print!(" [is ancestor of] ");
        exprs.print(e1, inline);
        println!();
    }
    println!("Used:");
    for Used(e0, e1) in used {
        print!("  ");
        exprs.print(e0, inline);
        print!(" [used in] ");
        exprs.print(e1, inline);
        println!();
    }
    println!("Violations");
    for Violation(e0, e1) in violations {
        print!("  ");
        exprs.print(e0, inline);
        print!(" [] ");
        exprs.print(e1, inline);
        println!();
    }
}

fn test0() {
    let mut exprs = ExprMap::default();
    let mut names = NameMap::default();

    // Tests the following expr:
    // -------------------------
    // let a = "foo" in  // {x}
    // let b = "bar" in  // {x, y}
    // let c = a in      // {c, b}
    // let d = a in      // error, a ∉ {c, b}
    // 1
    // -------------------------
    // Expected result: FAIL

    let a = names.new();
    let b = names.new();
    let c = names.new();
    let d = names.new();

    let foo = exprs.new(ExprKind::Str("foo".into()));
    let bar = exprs.new(ExprKind::Str("bar".into()));
    let a_ref0 = exprs.new(ExprKind::Var(a));
    let a_ref1 = exprs.new(ExprKind::Var(a));
    let one = exprs.new(ExprKind::I32(1));

    let let_d = exprs.new(ExprKind::Let(d, a_ref0, one));
    let let_c = exprs.new(ExprKind::Let(c, a_ref1, let_d));
    let let_b = exprs.new(ExprKind::Let(b, bar, let_c));
    let let_a = exprs.new(ExprKind::Let(a, foo, let_b));

    let inline = false;
    exprs.println(let_a, inline);

    typecheck(exprs);
}

fn test1() {
    let mut exprs = ExprMap::default();
    let mut names = NameMap::default();

    // Tests the following expr:
    // -------------------------
    // let a = (("foo", 5), "bar") in
    // let b = a.0.0 in
    // let c = a.0.1 in
    // let d = a.1 in
    // 1
    // -------------------------
    // Expected result: OK

    let a = names.new();
    let b = names.new();
    let c = names.new();
    let d = names.new();

    let foo = exprs.new(ExprKind::Str("foo".into()));
    let five = exprs.new(ExprKind::I32(5));
    let bar = exprs.new(ExprKind::Str("bar".into()));
    let a_ref0 = exprs.new(ExprKind::Var(a));
    let a_ref1 = exprs.new(ExprKind::Var(a));
    let one = exprs.new(ExprKind::I32(1));

    let tuple1 = exprs.new(ExprKind::Tuple(vec![foo, five]));
    let tuple2 = exprs.new(ExprKind::Tuple(vec![tuple1, bar]));

    let a_0_ref = exprs.new(ExprKind::Project(a_ref0, 0));
    let a_0_0_ref = exprs.new(ExprKind::Project(a_0_ref, 0));
    let a_0_1_ref = exprs.new(ExprKind::Project(a_0_ref, 1));
    let a_1_ref = exprs.new(ExprKind::Project(a_ref1, 1));

    let let_d = exprs.new(ExprKind::Let(d, a_1_ref, one));
    let let_c = exprs.new(ExprKind::Let(c, a_0_1_ref, let_d));
    let let_b = exprs.new(ExprKind::Let(b, a_0_0_ref, let_c));
    let let_a = exprs.new(ExprKind::Let(a, tuple2, let_b));

    let inline = false;
    exprs.println(let_a, inline);

    typecheck(exprs);
}

fn main() {
    for (i, test) in [test0, test1].iter().enumerate() {
        println!("===============[Test {}]===============", i);
        test();
        println!();
    }
}
