mod eval;
mod lexer;
mod parser;

use anyhow::Context;
use eval::evaluate;
use lexer::{Lexer, Token};
use parser::Parser;
use std::env;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();

    let program = args
        .get(1)
        .context("You must supply a program as argument")?;

    let lexer = Lexer::new(program);
    let tokens = lexer
        .collect::<anyhow::Result<Vec<Token>>>()
        .context("failed to lex program into a valid list of tokens")?;

    let parser = Parser::new(tokens.into_iter());
    let expressions = parser
        .collect::<anyhow::Result<Vec<_>>>()
        .context("failed to parse")?;

    for expression in expressions {
        println!("Result: {}", evaluate(expression)?);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::eval::evaluate;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    macro_rules! assert_program_eq {
        ($expect:expr, $program:expr) => {
            let lexer = Lexer::new($program);
            let tokens = lexer.collect::<anyhow::Result<Vec<_>>>().unwrap();

            let parser = Parser::new(tokens.into_iter());
            let mut expressions = parser.collect::<anyhow::Result<Vec<_>>>().unwrap();
            assert_eq!(1, expressions.len());
            assert_eq!($expect, evaluate(expressions.pop().unwrap()).unwrap());
        };
    }

    #[test]
    fn test_basic_programs() {
        // Ah yes, there is nothing better than comparing floats :crycat:
        assert_program_eq!(20f64, "5 + 5 * 3");
        assert_program_eq!(30f64, "(5 + 5) * 3");
    }
}
