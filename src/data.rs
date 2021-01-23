#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct NameId(pub(crate) usize);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct ExprId(pub(crate) usize);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub(crate) enum Expr {
    Let(NameId, ExprId, ExprId),
    Var(NameId),
    Tuple(Vec<ExprId>),
    Project(ExprId, Index),
    I32(i32),    // Non-linear
    Str(String), // Linear
}

#[derive(Default, Debug)]
pub(crate) struct ExprInterner {
    pub(crate) vec: Vec<Expr>,
}

impl ExprInterner {
    pub(crate) fn intern(&mut self, expr: Expr) -> ExprId {
        let id = ExprId(self.vec.len());
        self.vec.push(expr);
        id
    }
    pub(crate) fn resolve(&self, id: &ExprId) -> &Expr {
        self.vec.get(id.0).unwrap()
    }
}

#[derive(Default, Debug)]
pub(crate) struct NameInterner {
    pub counter: usize,
}

impl NameInterner {
    pub(crate) fn fresh(&mut self) -> NameId {
        let id = NameId(self.counter);
        self.counter += 1;
        id
    }
}

use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct Index(pub(crate) usize);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct PathId(pub(crate) usize);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum Path {
    Var(NameId),
    Project(PathId, Index),
}

#[derive(Default)]
pub(crate) struct PathInterner {
    counter: usize,
    path_to_id: HashMap<Path, PathId>,
    id_to_path: HashMap<PathId, Path>,
    pub(crate) roots: Vec<(PathId, NameId)>,
    pub(crate) parents: Vec<(PathId, PathId)>,
    pub(crate) uses: Vec<(PathId, ExprId)>,
}

use std::collections::hash_map::Entry;
use std::borrow::Borrow;

impl PathInterner {
    pub(crate) fn intern(&mut self, path: Path) -> PathId {
        match self.path_to_id.entry(path) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let id = PathId(self.counter);
                e.insert(id);
                self.id_to_path.insert(id, path);
                self.counter += 1;
                id
            }
        }
    }
    pub(crate) fn resolve(&self, id: impl Borrow<PathId>) -> &Path {
        self.id_to_path.get(id.borrow()).unwrap()
    }
    pub(crate) fn add_use(&mut self, p0: PathId, e0: ExprId) {
        self.uses.push((p0, e0))
    }
    pub(crate) fn add_root(&mut self, p0: PathId, x0: NameId) {
        self.roots.push((p0, x0))
    }
    pub(crate) fn add_parent(&mut self, p0: PathId, p1: PathId) {
        self.parents.push((p0, p1))
    }
    pub(crate) fn print(&self, id: impl Borrow<PathId>) {
        match self.resolve(id.borrow()) {
            Path::Var(x) => {
                print!("x{}", x.0);
            }
            Path::Project(p0, i) => {
                self.print(p0);
                print!(".{}", i.0);
            }
        }
    }
}
