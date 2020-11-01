use crate::ast::*;

pub fn pretty_print(expr: &Expr) -> String {
    match expr {
        Expr::Literal(literal) => match literal {
            Literal::NilLiteral => "nil".to_string(),
            Literal::BoolLiteral(b) => b.to_string(),
            Literal::StringLiteral(s) => s.clone(),
            Literal::NumberLiteral(n) => n.to_string(),
        },
        Expr::Binary(Binary {
            operator,
            left,
            right,
        }) => parenthize(operator.lexeme.as_str(), vec![left, right]),
        Expr::Grouping(Grouping { expr }) => parenthize("group", vec![expr]),
        Expr::Unary(Unary { operator, right }) => parenthize(operator.lexeme.as_str(), vec![right]),
        Expr::Logical(Logical {
            operator,
            left,
            right,
        }) => parenthize(operator.lexeme.as_str(), vec![left, right]),
    }
}

fn parenthize(name: &str, exprs: Vec<&Expr>) -> String {
    let mut rv = String::new();
    rv.push('(');
    rv.push_str(name);
    for expr in exprs.iter() {
        rv.push(' ');
        rv.push_str(pretty_print(expr).as_str());
    }
    rv.push(')');
    rv
}
