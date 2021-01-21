use crate::data::*;
use std::borrow::Borrow;

impl ExprInterner {
    pub(crate) fn resolve_paths(&self, id: impl Borrow<ExprId>) -> PathInterner {
        let mut interner = PathInterner::default();
        self.r(id.borrow(), &mut interner);
        interner
    }

    fn r(&self, e: &ExprId, paths: &mut PathInterner) -> Option<PathId> {
        match self.resolve(e) {
            Expr::Let(_, e0, e1) => {
                if let Some(path_id) = self.r(e0, paths) {
                    paths.add_use(path_id, *e0);
                }
                if let Some(path_id) = self.r(e1, paths) {
                    paths.add_use(path_id, *e0);
                }
                None
            }
            Expr::Var(x) => {
                let id = paths.intern(Path::Var(*x));
                paths.add_root(id, *x);
                Some(id)
            }
            Expr::Tuple(es) => {
                for e in es {
                    if let Some(path_id) = self.r(e, paths) {
                        paths.add_use(path_id, *e);
                    }
                }
                None
            }
            Expr::Project(e, i) => {
                if let Some(path_id) = self.r(e, paths) {
                    let id = paths.intern(Path::Project(path_id, *i));
                    paths.add_parent(path_id, id);
                    Some(id)
                } else {
                    None
                }
            }
            Expr::I32(_) => None,
            Expr::Str(_) => None,
        }
    }
}
