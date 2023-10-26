use ast::Stmt;
use emitter::Emitter;
use eval_expr::eval_expr;

pub fn eval_stmt(emitter: &mut Emitter, stmt: Stmt) {
    match stmt {
        Stmt::Expr(ref expr) => {
            eval_expr(emitter, expr);
        }
    }
}
