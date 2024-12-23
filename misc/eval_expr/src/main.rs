use std::{fmt::Display, iter::Peekable, str::Chars};

pub type Result<T> = std::result::Result<T, ExprError>;

const ASSOC_LEFT: i32 = 0;
const ASSOC_RIGHT: i32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Divide,
    Multiply,
    Power,
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExprError {
    Parse(String),
    DivisionByZero,
    InvalidNumber,
}

impl std::error::Error for ExprError {}

impl Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(s) => write!(f, "Parse error: {}", s),
            Self::DivisionByZero => write!(f, "Division by zero"),
            Self::InvalidNumber => write!(f, "Invalid number format"),
        }
    }
}

impl Token {
    fn is_operator(&self) -> bool {
        matches!(
            self,
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Power
        )
    }

    fn precedence(op: &Token) -> i32 {
        match op {
            Token::Multiply | Token::Divide => 2,
            Token::Plus | Token::Minus => 1,
            Token::Power => 3,
            _ => 0,
        }
    }

    fn assoc(&self) -> i32 {
        match self {
            Token::Power => ASSOC_RIGHT,
            _ => ASSOC_LEFT,
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
            Token::Power => Some(l.pow(r as u32)),
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
            Some('^') => Token::Power,
            Some('(') => Token::LeftParen,
            Some(')') => Token::RightParen,
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
    iter: Peekable<Tokenizer<'a>>,
}

impl<'a> Expr<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            iter: Tokenizer::new(src).peekable(),
        }
    }

    pub fn eval(&mut self) -> Result<i32> {
        let result = self.compute_expr(1)?;

        if self.iter.peek().is_some() {
            return Err(ExprError::Parse("Unexpected end of expression".into()));
        };

        Ok(result)
    }

    // New method to handle atomic expressions (numbers and parenthesized expressions)
    fn compute_atom(&mut self) -> Result<i32> {
        match self.iter.peek() {
            Some(Token::Number(num)) => {
                let val = *num;
                self.iter.next();
                Ok(val)
            }
            Some(Token::LeftParen) => {
                self.iter.next(); // consume '('
                let result = self.compute_expr(1)?;
                match self.iter.next() {
                    Some(Token::RightParen) => Ok(result),
                    _ => Err(ExprError::Parse("Expected closing parenthesis".into())),
                }
            }
            _ => Err(ExprError::Parse("Expected number or parenthesis".into())),
        }
    }

    pub fn compute_expr(&mut self, min_prec: i32) -> Result<i32> {
        let mut lhs = self.compute_atom()?;

        while let Some(&token) = self.iter.peek() {
            if !token.is_operator() || Token::precedence(&token) < min_prec {
                break;
            }

            let op = token;
            self.iter.next();

            let next_min_prec = if op.assoc() == ASSOC_LEFT {
                Token::precedence(&op) + 1
            } else {
                Token::precedence(&op)
            };

            let rhs = self.compute_expr(next_min_prec)?;
            lhs = op.compute(lhs, rhs).ok_or(ExprError::InvalidNumber)?;
        }

        Ok(lhs)
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
            "Parse error: Expected number or parenthesis"
        );

        let mut expr = Expr::new("1 + 2 / 0");
        assert_eq!(expr.eval().unwrap_err().to_string(), "Invalid number format");

        let mut expr = Expr::new("1 + 2 * 3 -");
        assert_eq!(
            expr.eval().unwrap_err().to_string(),
            "Parse error: Expected number or parenthesis"
        );

        let mut expr = Expr::new("1 + 2 * 3 - 4 / 0");
        assert_eq!(expr.eval().unwrap_err().to_string(), "Invalid number format");
    }

    #[test]
    fn test_parentheses() {
        let mut expr = Expr::new("(2 + 3) * 4");
        assert_eq!(expr.eval().unwrap(), 20);
    }

    #[test]
    fn test_power() {
        let mut expr = Expr::new("2 ^ 3");
        assert_eq!(expr.eval().unwrap(), 8);
    }

    #[test]
    fn test_complex_expressions() {
        assert_eq!(Expr::new("2 + 3 * 4").eval().unwrap(), 14);
        assert_eq!(Expr::new("(2 + 3) * 4").eval().unwrap(), 20);
        assert_eq!(Expr::new("2 ^ 3 ^ 2").eval().unwrap(), 512);
        assert_eq!(Expr::new("2 * (3 + 4) ^ 2").eval().unwrap(), 98);
        assert_eq!(Expr::new("2 ^ (1 ^ 4)").eval().unwrap(), 2);
        assert_eq!(Expr::new("(2 ^ 1) ^ 4").eval().unwrap(), 16);
    }
}
