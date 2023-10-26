use ast::Expr;
use emitter::Emitter;
use opcodes::*;

struct TypedValue {
    reg: u16,
}

fn expr_command(emitter: &mut Emitter, trees: &Vec<Expr>) -> TypedValue {
    emitter.write_2(OP_COMMAND, trees.len() as u16);

    for t in trees {
        let value = eval_expr_for_value(emitter, t);

        emitter.write_1(value.reg);
    }

    TypedValue { reg: 0 }
}

fn expr_string(emitter: &mut Emitter, s: &String) -> TypedValue {
    let pos = emitter.write_string(s);

    TypedValue { reg: pos }
}

fn eval_expr_for_value(emitter: &mut Emitter, expr: &Expr) -> TypedValue {
    match expr {
        Expr::Command(ref trees) => expr_command(emitter, trees),
        Expr::String(ref s) => expr_string(emitter, s),
    }
}

pub fn eval_expr(emitter: &mut Emitter, expr: &Expr) {
    eval_expr_for_value(emitter, expr);
}
