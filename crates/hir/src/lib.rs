mod database;
pub use database::Database;

use la_arena::Idx;

type ExprIdx = Idx<Expr>;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    VariableDef { name: String, value: Expr },
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Missing,
    Binary {
        op: BinaryOp,
        lhs: ExprIdx,
        rhs: ExprIdx,
    },
    Literal {
        /// is `None` if the number is too big to fit in a u64
        n: Option<u64>,
    },
    Unary {
        op: UnaryOp,
        expr: ExprIdx,
    },
    VariableRef {
        var: String,
    },
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Neg,
}

pub fn lower(ast: ast::Root) -> (Database, Vec<Stmt>) {
    let mut db = Database::default();
    let stmts = ast.stmts().filter_map(|stmt| db.lower_stmt(stmt)).collect();

    (db, stmts)
}
