use crate::ExprId;
use crate::Expr;
use crate::ExprInterner;
use std::borrow::Borrow;

impl ExprInterner {
    pub(crate) fn println(&self, id: impl Borrow<ExprId>, inline: bool) {
        self.print(id.borrow(), inline);
        println!();
        println!();
    }
    pub(crate) fn print(&self, id: impl Borrow<ExprId>, inline: bool) {
        match self.resolve(id.borrow()) {
            Expr::Let(nx0, ex0, ex1) => {
                print!("let x_{} = ", nx0.0);
                self.print(ex0, inline);
                print!(" in");
                if inline {
                    print!(" ");
                } else {
                    println!();
                }
                self.print(ex1, inline);
            }
            Expr::Var(nx0) => print!("x_{}", nx0.0),
            Expr::I32(v) => print!("{}i32", v),
            Expr::Tuple(es) => {
                print!("(");
                let mut es = es.iter();
                self.print(es.next().unwrap(), inline);
                for e in es {
                    print!(", ");
                    self.print(e, inline);
                }
                print!(")");
            }
            Expr::Project(e, x) => {
                self.print(e, inline);
                print!(".{}", x.0);
            }
            Expr::Str(s) => {
                print!("{:?}", s);
            }
        }
    }
}
