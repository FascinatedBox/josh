use crate::ast::Expr;
use crate::compiler::Compiler;
use crate::opcode::*;

fn eval_command_string(compiler: &mut Compiler, s: &String) {
    let string_id = compiler.intern_string(s);

    crate::emit_n!(compiler, C_LOAD_STRING, string_id);
}

fn eval_command_expr(compiler: &mut Compiler, expr: &Expr) {
    match expr {
        Expr::String(ref s) => eval_command_string(compiler, s),
        _ => panic!("Invalid tree for eval_command_expr."),
    }
}

fn expr_command(compiler: &mut Compiler, expr_list: &Vec<Expr>) {
    crate::emit_n!(compiler, O_COMMAND);

    for e in expr_list {
        eval_command_expr(compiler, e);
    }

    crate::emit_n!(compiler, C_EXEC);
}

pub fn eval_expr(compiler: &mut Compiler, expr: &Expr) {
    match expr {
        Expr::Command(ref trees) => expr_command(compiler, trees),
        Expr::String(_) => (),
    }
}
