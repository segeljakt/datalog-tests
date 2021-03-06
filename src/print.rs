use crate::ExprMap;
use crate::Expr;
use crate::ExprKind;

impl ExprMap {
    pub(crate) fn println(&self, id: Expr) {
        self.print(id);
        println!();
        println!();
    }
    pub(crate) fn print(&self, id: Expr) {
        match self.get(id) {
            ExprKind::Let(nx0, ex0, ex1) => {
                print!("let x_{} = ", nx0.0);
                self.print(*ex0);
                print!(" in ");
                self.print(*ex1);
            }
            ExprKind::Var(nx0) => print!("x_{}", nx0.0),
            ExprKind::Add(ex0, ex1) => {
                self.print(*ex0);
                print!(" + ");
                self.print(*ex1);
            }
            ExprKind::Equ(ex0, ex1) => {
                self.print(*ex0);
                print!(" == ");
                self.print(*ex1);
            }
            ExprKind::I32(v) => print!("{}i32", v),
            ExprKind::U32(v) => print!("{}u32", v),
        }
    }
}
