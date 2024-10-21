#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Number(i32),
    Plus,
    Minus,
}

#[derive(Debug)]
struct Accumulator {
    result: i32,
    operand: Operand,
}

#[derive(Debug)]
enum Operand {
    Add,
    Sub,
}

fn tokenize(src: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = src.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut num = 0;

                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) {
                        num = num * 10 + c.to_digit(10).unwrap() as i32;
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Number(num));
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            ' ' => {
                chars.next();
            }
            _ => return Err(format!("Invalid character: {}", c)),
        }
    }
    Ok(tokens)
}

fn eval_expr(src: &str) -> Result<i32, String> {
    let tokens = tokenize(src)?;

    let initial_acc = Accumulator {
        result: 0,
        operand: Operand::Add,
    };

    // Currently, we only have two operands: Add and Sub, so we don't need to worry about precedence, just evaluate from left to right.
    let sum = tokens
        .iter()
        .fold(initial_acc, |mut acc, token| {
            match token {
                Token::Number(num) => match acc.operand {
                    Operand::Add => acc.result += num,
                    Operand::Sub => acc.result -= num,
                },
                Token::Plus => acc.operand = Operand::Add,
                Token::Minus => acc.operand = Operand::Sub,
            }
            acc
        })
        .result;

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("1 + 2 - 3").unwrap(),
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
}
