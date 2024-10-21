# eval_expr

This is a simple expression evaluator written in rust. It can evaluate simple arithmetic expressions.

- v1: Simple functionality.
- v2: This version includes a Token enum for defining units, a tokenize function to convert input strings into tokens, an `eval_expr` function to parse tokens and return tokens.
- v3: This version we accumulate the result from the tokens and return the result as a number.
- v4: This version includes new operators: `*` and `/` and also modified the `eval_expr` function to handle these operators. since `*` and `/` have higher precedence than `+` and `-`, we need to handle them first.
- v5: Refactor tokenize function, optimized the `Tokenize` struct and implemented the `Iterator` trait for it.
