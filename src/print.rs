use crate::ExprMap;
use crate::Expr;
use crate::ExprKind;

impl ExprMap {
    pub(crate) fn print(&self, id: Expr) {
        self.print_rec(id);
        println!();
        println!();
    }
    pub(crate) fn print_rec(&self, id: Expr) {
        match self.get(id) {
            ExprKind::Let(ex0, ex1) => {
                print!("let _ = ");
                self.print_rec(*ex0);
                print!(" in ");
                self.print_rec(*ex1);
            }
            ExprKind::I32(v) => print!("{}i32", v),
            ExprKind::U32(v) => print!("{}u32", v),
        }
    }
}
