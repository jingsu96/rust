use std::{fmt::Display, iter::Peekable, str::Chars};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Divide,
    Multiply,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExprError {
    Parse(String),
}

impl std::error::Error for ExprError {}

impl Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(s) => write!(f, "{}", s),
        }
    }
}

impl Token {
    fn is_operator(&self) -> bool {
        match self {
            Token::Plus | Token::Minus | Token::Divide | Token::Multiply => true,
            _ => false,
        }
    }

    fn precedence(op: &Token) -> i32 {
        match op {
            Token::Multiply | Token::Divide => 2,
            Token::Plus | Token::Minus => 1,
            _ => 0,
        }
    }

    fn compute(&self, l: i32, r: i32) -> Option<i32> {
        match &self {
            Token::Plus => Some(l + r),
            Token::Minus => Some(l - r),
            Token::Multiply => Some(l * r),
            Token::Divide => {
                if r == 0 {
                    None
                } else {
                    Some(l / r)
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Tokenizer<'a> {
    tokens: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            tokens: src.chars().peekable(),
        }
    }

    fn consume_whitespace(&mut self) {
        while let Some(&c) = self.tokens.peek() {
            if c.is_whitespace() {
                self.tokens.next();
            } else {
                break;
            }
        }
    }

    fn scan_number(&mut self) -> Option<Token> {
        let mut num = 0;

        while let Some(&c) = self.tokens.peek() {
            if c.is_digit(10) {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
                self.tokens.next();
            } else {
                break;
            }
        }

        Some(Token::Number(num))
    }

    fn scan_operator(&mut self) -> Option<Token> {
        let op = match self.tokens.next() {
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Multiply,
            Some('/') => Token::Divide,
            _ => return None,
        };
        Some(op)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_whitespace();

        match self.tokens.peek() {
            Some(&c) if c.is_numeric() => self.scan_number(),
            Some(_) => self.scan_operator(),
            None => None,
        }
    }
}

struct Expr<'a> {
    tokens: Peekable<Tokenizer<'a>>,
}

impl<'a> Expr<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            tokens: Tokenizer::new(src).peekable(),
        }
    }

    pub fn eval(&mut self) -> Result<i32, String> {
        self.compute_expr()
    }

    fn apply_op(numbers: &mut Vec<i32>, op: Token) -> Result<(), String> {
        if numbers.len() < 2 {
            return Err(
                ExprError::Parse("Not enough numbers to apply operator".into()).to_string(),
            );
        }

        let b = numbers.pop().unwrap();
        let a = numbers.pop().unwrap();

        let result = op
            .compute(a, b)
            .ok_or_else(|| ExprError::Parse("Division by zero".into()).to_string())?;

        numbers.push(result);
        Ok(())
    }

    pub fn compute_expr(&mut self) -> Result<i32, String> {
        let mut numbers = Vec::new();
        let mut ops = Vec::new();

        while let Some(token) = self.tokens.next() {
            match token {
                Token::Number(num) => numbers.push(num),
                token if token.is_operator() => {
                    while let Some(&op) = ops.last() {
                        if Token::precedence(&op) >= Token::precedence(&token) {
                            Self::apply_op(&mut numbers, ops.pop().unwrap())?;
                        } else {
                            break;
                        }
                    }
                    ops.push(token);
                }
                _ => return Err(ExprError::Parse("Invalid token".into()).to_string()),
            }
        }

        while let Some(op) = ops.pop() {
            Self::apply_op(&mut numbers, op)?;
        }

        numbers
            .pop()
            .ok_or_else(|| ExprError::Parse("Invalid expression".into()).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            Tokenizer::new("1 + 2 - 3").collect::<Vec<_>>(),
            vec![
                Token::Number(1),
                Token::Plus,
                Token::Number(2),
                Token::Minus,
                Token::Number(3)
            ]
        );
    }

    #[test]
    fn test_eval_expr() {
        let mut expr = Expr::new("1 + 2 - 3");
        assert_eq!(expr.eval().unwrap(), 0);
    }

    #[test]
    fn test_eval_expr_with_precedence() {
        let mut expr = Expr::new("1 + 2 * 3");
        assert_eq!(expr.eval().unwrap(), 7);

        let mut expr = Expr::new("1 + 2 * 3 - 4");
        assert_eq!(expr.eval().unwrap(), 3);
    }

    #[test]
    fn test_parse_error() {
        let mut expr = Expr::new("1 + 2 *");
        assert_eq!(
            expr.eval().unwrap_err().to_string(),
            "Not enough numbers to apply operator"
        );

        let mut expr = Expr::new("1 + 2 / 0");
        assert_eq!(expr.eval().unwrap_err().to_string(), "Division by zero");

        let mut expr = Expr::new("1 + 2 * 3 -");
        assert_eq!(
            expr.eval().unwrap_err().to_string(),
            "Not enough numbers to apply operator"
        );

        let mut expr = Expr::new("1 + 2 * 3 - 4 / 0");
        assert_eq!(expr.eval().unwrap_err().to_string(), "Division by zero");
    }
}
