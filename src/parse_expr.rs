use ast::Expr;
use parser::ParseState;
use token::Token;

fn infix_power(token: &Token) -> Option<(u8, u8)> {
    let result = match token {
        Token::Eq => (2, 1),
        Token::Minus | Token::Plus => (5, 6),
        Token::Divide | Token::Multiply => (7, 8),
        _ => return None,
    };
    Some(result)
}

fn expr_power(parser: &mut ParseState, min_power: u8) -> Expr {
    let spanned_left = parser.lexer.next();

    let mut left = match spanned_left.kind {
        Token::Number(n) => Expr::Number(n),
        t => panic!("Invalid token: {:?}", t),
    };

    loop {
        let op = parser.lexer.peek();
        let kind = &op.kind;

        if let Some((left_power, right_power)) = infix_power(kind) {
            if left_power < min_power {
                break;
            }

            parser.lexer.advance(&op);

            match kind {
                Token::Plus => {
                    let right = expr_power(parser, right_power);
                    left = Expr::Plus(Box::new(left), Box::new(right));
                }
                _ => {
                    panic!("token not handled here!");
                }
            }

            continue;
        }

        break;
    }

    left
}

pub fn parse_expr(parser: &mut ParseState) -> Expr {
    expr_power(parser, 0)
}
