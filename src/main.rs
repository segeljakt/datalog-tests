mod data;
mod print;
mod resolve;

use data::*;

use crepe::crepe;

crepe! {

    @input
    // NameId is the direct root of PathId, e.g. Root(a, a.b)
    struct Root(NameId, PathId);

    @input
    // PathId is a direct parent of PathId, e.g. Parent(a.b, a.b.c)
    struct Parent(PathId, PathId);

    // PathId is used in ExprId, e.g. Use(a.b, <id>) where let x = <id> in e
    @input
    struct Used(PathId, ExprId);

    // NameId is the transitive root of PathId, e.g. Origin(a, a.b.c)
    @output // TODO: Remove
    struct Origin(NameId, PathId);

    // PathId is a transitive ancestor of PathId, e.g. Ancestor(a.b, a.b.c.d)
    @output // TODO: Remove
    struct Ancestor(PathId, PathId);

    // Origin: Direct-case
    Origin(x, p0) <-
        Root(x, p0);

    // Origin: Transitive case
    Origin(x, p1) <-
        Root(x, p0),
        Ancestor(p0, p1);

    // Ancestor: Direct case
    Ancestor(p0, p1) <-
        Parent(p0, p1);

    // Ancestor: Transitive case
    Ancestor(p0, p2) <-
        Parent(p0, p1),
        Ancestor(p1, p2);

    // Error when PathId is an Ancestor of PathId and is used in ExprId.
    @output
    struct AncestorUsed(PathId, PathId, ExprId);

    // Error when PathId is used in both ExprId and ExprId.
    @output
    struct DoubleUse(PathId, ExprId, ExprId);

    // An ancestor of a child cannot be used.
    AncestorUsed(p0, p1, e0) <-
        Used(p0, e0),
        Ancestor(p0, p1);

    // A path cannot be used twice.
    DoubleUse(p0, e0, e1) <-
        Used(p0, e0),
        Used(p0, e1),
        (e0 != e1);
}

fn typecheck(exprs: ExprInterner, e: ExprId) {
    let inline = false;
    exprs.println(e, inline);
    let mut paths = exprs.resolve_paths(e);

    let mut runtime = Crepe::new();
    let roots = paths
        .roots
        .drain(..)
        .map(|(x0, p0)| Root(p0, x0))
        .collect::<Vec<_>>();
    let parents = paths
        .parents
        .drain(..)
        .map(|(p0, p1)| Parent(p0, p1))
        .collect::<Vec<_>>();
    let uses = paths
        .uses
        .drain(..)
        .map(|(p0, e0)| Used(p0, e0))
        .collect::<Vec<_>>();

    println!("Used:");
    for Used(p0, p1) in uses.iter() {
        print!("  ");
        paths.print(p0);
        print!(" [used in] ");
        exprs.print(p1, true);
        println!();
    }

    runtime.extend(roots.into_iter());
    runtime.extend(parents.into_iter());
    runtime.extend(uses.into_iter());

    let (origins, ancestors, ancestor_uses, double_uses) = runtime.run();
    let inline = true;
    println!("Origins:");
    for Origin(x, e) in origins {
        print!("  x{} [is origin of] ", x.0);
        paths.print(e);
        println!();
    }
    println!("Ancestors:");
    for Ancestor(p0, p1) in ancestors {
        print!("  ");
        paths.print(p0);
        print!(" [is ancestor of] ");
        paths.print(p1);
        println!();
    }
    println!("AncestorUses");
    for AncestorUsed(p0, p1, e0) in ancestor_uses {
        print!("  ");
        paths.print(p0);
        print!(" [is ancestor of] ");
        paths.print(p1);
        print!(" [and was used here] ");
        exprs.print(e0, inline);
        println!();
    }
    println!("DoubleUses");
    for DoubleUse(p0, e0, e1) in double_uses {
        print!("  ");
        paths.print(p0);
        print!(" [used in] ");
        exprs.print(e0, inline);
        print!(" [and also] ");
        exprs.print(e1, inline);
        println!();
    }
}

fn test0() {
    let mut exprs = ExprInterner::default();
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

    let a = names.fresh();
    let b = names.fresh();
    let c = names.fresh();
    let d = names.fresh();

    let foo = exprs.intern(Expr::Str("foo".into()));
    let bar = exprs.intern(Expr::Str("bar".into()));
    let a_ref0 = exprs.intern(Expr::Var(a));
    let a_ref1 = exprs.intern(Expr::Var(a));
    let one = exprs.intern(Expr::I32(1));

    let let_d = exprs.intern(Expr::Let(d, a_ref0, one));
    let let_c = exprs.intern(Expr::Let(c, a_ref1, let_d));
    let let_b = exprs.intern(Expr::Let(b, bar, let_c));
    let let_a = exprs.intern(Expr::Let(a, foo, let_b));

    typecheck(exprs, let_a);
}

fn test1() {
    let mut exprs = ExprInterner::default();
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

    let a = names.fresh();
    let b = names.fresh();
    let c = names.fresh();
    let d = names.fresh();

    let foo = exprs.intern(Expr::Str("foo".into()));
    let five = exprs.intern(Expr::I32(5));
    let bar = exprs.intern(Expr::Str("bar".into()));
    let a_ref0 = exprs.intern(Expr::Var(a));
    let a_ref1 = exprs.intern(Expr::Var(a));
    let one = exprs.intern(Expr::I32(1));

    let tuple1 = exprs.intern(Expr::Tuple(vec![foo, five]));
    let tuple2 = exprs.intern(Expr::Tuple(vec![tuple1, bar]));

    let a_0_ref = exprs.intern(Expr::Project(a_ref0, Index(0)));
    let a_0_0_ref = exprs.intern(Expr::Project(a_0_ref, Index(0)));
    let a_0_1_ref = exprs.intern(Expr::Project(a_0_ref, Index(1)));
    let a_1_ref = exprs.intern(Expr::Project(a_ref1, Index(1)));

    let let_d = exprs.intern(Expr::Let(d, a_1_ref, one));
    let let_c = exprs.intern(Expr::Let(c, a_0_1_ref, let_d));
    let let_b = exprs.intern(Expr::Let(b, a_0_0_ref, let_c));
    let let_a = exprs.intern(Expr::Let(a, tuple2, let_b));

    let inline = false;
    exprs.println(&let_a, inline);

    typecheck(exprs, let_a);
}

fn main() {
    for (i, test) in [test0, test1].iter().enumerate() {
        println!("===============[Test {}]===============", i);
        test();
        println!();
    }
}
