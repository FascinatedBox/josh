use ast::Expr;
use emitter::Emitter;
use opcodes::*;

pub struct TypedValue {
    pub reg: u16,
}

fn expr_command(emitter: &mut Emitter, trees: &Vec<Expr>) -> TypedValue {
    emitter.write_2(OP_COMMAND, trees.len() as u16);

    for t in trees {
        let value = eval_expr_for_value(emitter, t);

        emitter.write_1(value.reg);
    }

    TypedValue { reg: 0 }
}

fn expr_number(emitter: &mut Emitter, n: &i64) -> TypedValue {
    let pos = emitter.write_number(n);
    let storage_id = emitter.get_storage_id();

    emitter.write_3(OP_LOAD_INTEGER, pos, storage_id);

    TypedValue { reg: storage_id }
}

fn expr_plus(emitter: &mut Emitter, left: &Expr, right: &Expr) -> TypedValue {
    let left = eval_expr_for_value(emitter, left);
    let right = eval_expr_for_value(emitter, right);
    let storage_id = emitter.get_storage_id();

    emitter.write_4(OP_PLUS, left.reg, right.reg, storage_id);

    TypedValue { reg: storage_id }
}

fn expr_string(emitter: &mut Emitter, s: &String) -> TypedValue {
    let pos = emitter.write_string(s);

    TypedValue { reg: pos }
}

pub fn eval_expr_for_value(emitter: &mut Emitter, expr: &Expr) -> TypedValue {
    match expr {
        Expr::Command(ref trees) => expr_command(emitter, trees),
        Expr::Number(ref n) => expr_number(emitter, n),
        Expr::Plus(ref left, ref right) => expr_plus(emitter, left, right),
        Expr::String(ref s) => expr_string(emitter, s),
    }
}
