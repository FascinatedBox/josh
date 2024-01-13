use crate::ast::Stmt;
use crate::compiler::Compiler;
use crate::eval_expr::eval_expr;

pub fn eval_stmt(compiler: &mut Compiler, stmt: Stmt) {
    match stmt {
        Stmt::Expr(ref expr) => eval_expr(compiler, expr),
        Stmt::Todo => (),
    }
}
