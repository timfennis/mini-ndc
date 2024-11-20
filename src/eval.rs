use crate::parser::Expression;
use anyhow::anyhow;

pub fn evaluate(expression: Expression) -> anyhow::Result<f64> {
    match expression {
        Expression::Binary {
            left,
            operator,
            right,
        } => {
            let left = evaluate(*left)?;
            let right = evaluate(*right)?;

            let res = match operator {
                crate::parser::Operator::Multiply => left * right,
                crate::parser::Operator::Divide => left / right,
                crate::parser::Operator::Add => left + right,
                crate::parser::Operator::Subtract => left - right,
                crate::parser::Operator::Exponentiate => left.powf(right),
            };

            Ok(res)
        }
        Expression::Literal(f) => Ok(f),
        Expression::Unary { right, operator } => {
            let right = evaluate(*right)?;
            match operator {
                crate::parser::Operator::Subtract => Ok(-right),
                _ => Err(anyhow!("invalid operator in unary position")),
            }
        }
        Expression::Grouping(grouped) => evaluate(*grouped),
    }
}
