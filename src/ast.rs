pub enum Expr {
    Command(Vec<Expr>),
    String(String),
}

pub enum Stmt {
    Expr(Expr),
    Todo,
}
