#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Number(i32),
    Plus,
    Minus,
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

fn eval_expr(src: &str) -> Result<Vec<Token>, String> {
    let tokens = tokenize(src)?;
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            eval_expr("1 + 2 - 3").unwrap(),
            vec![
                Token::Number(1),
                Token::Plus,
                Token::Number(2),
                Token::Minus,
                Token::Number(3)
            ]
        );
    }
}
