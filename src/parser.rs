use std::iter::Peekable;

use crate::lexer::Token;
use anyhow::anyhow;

#[derive(Debug)]
pub enum Operator {
    Multiply,
    Divide,
    Add,
    Subtract,
    Exponentiate,
}

impl TryFrom<Token> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Literal(_) => Err(anyhow!("floats are not a valid operator")),
            Token::Plus => Ok(Operator::Add),
            Token::Minus => Ok(Operator::Subtract),
            Token::Star => Ok(Operator::Multiply),
            Token::ForwardSlash => Ok(Operator::Divide),
            Token::Caret => Ok(Operator::Exponentiate),
            Token::LeftParentheses => Err(anyhow!("left parentheses are not an operator")),
            Token::RightParentheses => Err(anyhow!("right parentheses are not an operator")),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(f64),
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    Unary {
        right: Box<Expression>,
        operator: Operator,
    },
    Grouping(Box<Expression>),
}
pub struct Parser<I: Iterator> {
    tokens: Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    fn expression(&mut self) -> anyhow::Result<Expression> {
        return self.exponentiation();
    }

    fn exponentiation(&mut self) -> anyhow::Result<Expression> {
        let expression = self.term()?;

        if let Some(Token::Caret) = self.tokens.peek() {
            // SAFE: Unwrap is guaranteed to success by the previous call to peek
            let operator = Operator::try_from(self.tokens.next().unwrap())?;

            let right = self.exponentiation()?;

            return Ok(Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expression)
    }

    fn term(&mut self) -> anyhow::Result<Expression> {
        // Recurse down to the next level (of higher presedence operators)
        let mut expression = self.factor()?;

        // As long as we can parse + or - tokens we continue creating binary expressions
        while let Some(Token::Plus | Token::Minus) = self.tokens.peek() {
            // SAFE: Unwrap is guaranteed to success by the previous call to peek
            let operator = Operator::try_from(self.tokens.next().unwrap())?;

            let right = self.factor()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expression)
    }

    fn factor(&mut self) -> anyhow::Result<Expression> {
        let mut expression = self.unary()?;

        while let Some(Token::Star | Token::ForwardSlash) = self.tokens.peek() {
            let operator = Operator::try_from(self.tokens.next().unwrap())?;

            let right = self.factor()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expression)
    }
    fn unary(&mut self) -> anyhow::Result<Expression> {
        if let Some(Token::Minus) = self.tokens.peek() {
            // SAFE: Unwrap is guaranteed to success by the previous call to peek
            let operator = Operator::try_from(self.tokens.next().unwrap())?;
            let right = self.unary()?;
            return Ok(Expression::Unary {
                right: Box::new(right),
                operator,
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> anyhow::Result<Expression> {
        match self.tokens.next() {
            Some(Token::Literal(lit)) => Ok(Expression::Literal(lit)),
            Some(Token::LeftParentheses) => {
                let expression = self.expression()?;
                if let Some(Token::RightParentheses) = self.tokens.next() {
                    return Ok(Expression::Grouping(Box::new(expression)));
                } else {
                    Err(anyhow!("expected ) after expression"))
                }
            }
            Some(token) => Err(anyhow!("unexpected token: {token:?}")),
            None => Err(anyhow!("unexpected end of input")),
        }
    }
}

impl<I> Iterator for Parser<I>
where
    I: Iterator<Item = Token>,
{
    type Item = anyhow::Result<Expression>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tokens.peek().is_some() {
            Some(self.expression())
        } else {
            None
        }
    }
}
