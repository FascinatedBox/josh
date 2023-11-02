pub enum Expr {
    Command(Vec<Expr>),
    Number(i64),
    Plus(Box<Expr>, Box<Expr>),
    String(String),
}

pub enum Stmt {
    Expr(Expr),
    Var(String, Expr),
}
