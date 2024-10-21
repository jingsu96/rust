use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Divide,
    Multiply,
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

fn precedence(op: &Token) -> i32 {
    match op {
        Token::Multiply | Token::Divide => 2,
        Token::Plus | Token::Minus => 1,
        _ => 0,
    }
}

fn apply_op(numbers: &mut Vec<i32>, op: Token) -> Result<(), String> {
    if numbers.len() < 2 {
        return Err("Not enough numbers to apply operator".to_string());
    }

    let b = numbers.pop().unwrap();
    let a = numbers.pop().unwrap();

    let result = match op {
        Token::Plus => a + b,
        Token::Minus => a - b,
        Token::Multiply => a * b,
        Token::Divide => {
            if b == 0 {
                return Err("Division by zero".to_string());
            }
            a / b
        }
        _ => return Err("Invalid operator".to_string()),
    };
    numbers.push(result);
    Ok(())
}

fn eval_expr(src: &str) -> Result<i32, String> {
    let tokens = Tokenizer::new(src);
    let mut numbers = Vec::new();
    let mut ops = Vec::new();

    for token in tokens {
        match token {
            Token::Number(num) => numbers.push(num),
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                while let Some(&op) = ops.last() {
                    if precedence(&op) >= precedence(&token) {
                        apply_op(&mut numbers, ops.pop().unwrap())?;
                    } else {
                        break;
                    }
                }
                ops.push(token);
            }
        }
    }

    while let Some(op) = ops.pop() {
        apply_op(&mut numbers, op)?;
    }

    numbers
        .pop()
        .ok_or_else(|| "Invalid expression".to_string())
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
        assert_eq!(eval_expr("1 + 2 - 3").unwrap(), 0);
        assert_eq!(eval_expr("1 + 2 - 3 + 4").unwrap(), 4);
        assert_eq!(eval_expr("1 + 2 - 3 + 4 - 5").unwrap(), -1);
    }

    #[test]
    fn test_eval_expr_with_precedence() {
        assert_eq!(eval_expr("1 + 2 * 3").unwrap(), 7);
        assert_eq!(eval_expr("1 + 2 * 3 - 4").unwrap(), 3);
        assert_eq!(eval_expr("1 + 2 * 3 - 4 / 2").unwrap(), 5);
    }
}
