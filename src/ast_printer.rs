use crate::ast::*;

pub fn pretty_print(expr: &Expr) -> String {
    match expr {
        Expr::Literal(literal) => match literal {
            Literal::NilLiteral => "nil".to_string(),
            Literal::BoolLiteral(b) => b.to_string(),
            Literal::StringLiteral(s) => s.clone(),
            Literal::NumberLiteral(n) => n.to_string(),
        },
        Expr::Binary(b) => parenthize(&b.operator.to_string(), vec![&b.left, &b.right]),
        Expr::Grouping(g) => parenthize("group", vec![&g.expr]),
        Expr::Unary(u) => parenthize(&u.operator.to_string(), vec![&u.right]),
        Expr::Logical(l) => parenthize(&l.operator.to_string(), vec![&l.left, &l.right]),
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
