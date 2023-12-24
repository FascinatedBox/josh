use ast::{Expr, Stmt};
use emitter::Emitter;
use opcodes::*;

fn eval_var(emitter: &mut Emitter, name: String, expr: Expr) {
    let var_id = emitter.declare_local(name);
    let expr_value = emitter.eval_expr(&expr);

    ::emit_n!(emitter, OP_ASSIGN, var_id, expr_value);
}

pub fn eval_stmt(emitter: &mut Emitter, stmt: Stmt) {
    match stmt {
        Stmt::Expr(ref expr) => {
            emitter.eval_expr(expr);
        }
        Stmt::Var(name, expr) => eval_var(emitter, name, expr),
    }
}
