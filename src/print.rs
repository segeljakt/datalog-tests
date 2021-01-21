use crate::Expr;
use crate::ExprKind;
use crate::ExprMap;

impl ExprMap {
    pub(crate) fn println(&self, id: Expr, inline: bool) {
        self.print(id, inline);
        println!();
        println!();
    }
    pub(crate) fn print(&self, id: Expr, inline: bool) {
        match self.get(id) {
            ExprKind::Let(nx0, ex0, ex1) => {
                print!("let x_{} = ", nx0.0);
                self.print(*ex0, inline);
                print!(" in");
                if inline {
                    print!(" ");
                } else {
                    println!();
                }
                self.print(*ex1, inline);
            }
            ExprKind::Var(nx0) => print!("x_{}", nx0.0),
            ExprKind::I32(v) => print!("{}i32", v),
            ExprKind::Tuple(es) => {
                print!("(");
                let mut es = es.iter();
                self.print(*es.next().unwrap(), inline);
                for e in es {
                    print!(", ");
                    self.print(*e, inline);
                }
                print!(")");
            }
            ExprKind::Project(e, x) => {
                self.print(*e, inline);
                print!(".{}", x);
            }
            ExprKind::Str(s) => {
                print!("{:?}", s);
            }
        }
    }
}
