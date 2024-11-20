use anyhow::{anyhow, Context};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Literal(f64),
    Plus,
    Minus,
    Star,
    ForwardSlash,
    Caret,
    LeftParentheses,
    RightParentheses,
}

pub struct Lexer<'a> {
    input: &'a str,
    offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, offset: 0 }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = anyhow::Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.offset >= self.input.len() {
                return None;
            }

            let next_char = self.input[self.offset..].chars().next()?;
            self.offset += 1;
            match next_char {
                '+' => return Some(Ok(Token::Plus)),
                '*' => return Some(Ok(Token::Star)),
                '/' => return Some(Ok(Token::ForwardSlash)),
                '-' => return Some(Ok(Token::Minus)),
                '^' => return Some(Ok(Token::Caret)),
                '(' => return Some(Ok(Token::LeftParentheses)),
                ')' => return Some(Ok(Token::RightParentheses)),
                c if c.is_ascii_digit() => {
                    let mut rest = self.input[self.offset..]
                        .chars()
                        .take_while(|&c| c.is_ascii_digit() || c == '.')
                        .collect::<String>();

                    self.offset += rest.len();

                    rest.insert(0, c);

                    let out = rest.parse::<f64>().map(|float| Token::Literal(float));

                    return Some(out.context("failed to parse float"));
                }
                ' ' | '\t' | '\n' => {}
                c => return Some(Err(anyhow!("invalid character in input: {c}"))),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let lexer = Lexer::new("5 + 5 * 3");
        let tokens = lexer.collect::<anyhow::Result<Vec<Token>>>().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Literal(5f64),
                Token::Plus,
                Token::Literal(5f64),
                Token::Star,
                Token::Literal(3f64)
            ]
        )
    }
}
