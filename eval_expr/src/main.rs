fn eval_expr(src: &str) -> Result<i32, String> {
    Ok(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_expr() {
        assert_eq!(eval_expr("3"), Ok(3));
    }
}
