# eval_expr

This is a simple expression evaluator written in Rust that can evaluate basic arithmetic expressions.

- **v1**: Introduced simple functionality.
- **v2**: Added a `Token` enum to define units, a `tokenize` function to convert input strings into tokens, and an `eval_expr` function to parse tokens and return the evaluation result.
- **v3**: Modified the implementation to accumulate the result from the tokens and return the final result as a number.
- **v4**: Introduced new operators, `*` and `/`, and updated the `eval_expr` function to handle these operators, ensuring that they are processed before `+` and `-` due to their higher precedence.
- **v5**: Refactored the `tokenize` function, optimized the `Tokenize` struct, and implemented the `Iterator` trait for it.
- **v6**: Refactored the `eval_expr` function to use the `Expr` struct for storing tokens and the result.
- **v7**: Refactored the `Token` enum and added functionality to parse a string into a token.
- **v8**: Implemented error handling for parsing and applying operators.
- **v9**: Added three new operators: `^`, `(`, and `)`, and implemented the top-down operator precedence parsing algorithm to handle operator precedence.
- **v10**: Added support for associativity of operators.
