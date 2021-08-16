use pretty_assertions::assert_eq;

use smith::tokenizer::{tokenize, Kind, Tokens, TopLevel};
use test_utilities::strings;

fn token_string_literal(
    top_level: &TopLevel,
    token: usize,
    mut output: String,
    text: &str,
) -> String {
    output.push_str(text);
    output.push_str("\n");
    token_string_impl(top_level, token + 1, output)
}

fn token_string_symbol(top_level: &TopLevel, token: usize, mut output: String) -> String {
    let text = &top_level.symbols[top_level.indices[token]];
    output.push_str("Symbol ");
    output.push_str(text);
    output.push_str("\n");
    token_string_impl(top_level, token + 1, output)
}

fn token_string_int(top_level: &TopLevel, token: usize, mut output: String) -> String {
    let text = &top_level.ints[top_level.indices[token]];
    output.push_str("Int ");
    output.push_str(text);
    output.push_str("\n");
    token_string_impl(top_level, token + 1, output)
}

fn token_string_indent(top_level: &TopLevel, token: usize, mut output: String) -> String {
    let indent = &top_level.indents[top_level.indices[token]];
    output.push_str("Indent ");
    output.push_str(&indent.to_string());
    output.push_str("\n");
    token_string_impl(top_level, token + 1, output)
}

fn token_string_impl(top_level: &TopLevel, token: usize, output: String) -> String {
    match top_level.kinds.get(token) {
        Some(Kind::Def) => token_string_literal(top_level, token, output, "Def"),
        Some(Kind::LeftParen) => token_string_literal(top_level, token, output, "LeftParen"),
        Some(Kind::RightParen) => token_string_literal(top_level, token, output, "RightParen"),
        Some(Kind::Colon) => token_string_literal(top_level, token, output, "Colon"),
        Some(Kind::Plus) => token_string_literal(top_level, token, output, "Plus"),
        Some(Kind::Minus) => token_string_literal(top_level, token, output, "Minus"),
        Some(Kind::Times) => token_string_literal(top_level, token, output, "Times"),
        Some(Kind::Slash) => token_string_literal(top_level, token, output, "Slash"),
        Some(Kind::Equal) => token_string_literal(top_level, token, output, "Equal"),
        Some(Kind::Symbol) => token_string_symbol(top_level, token, output),
        Some(Kind::Int) => token_string_int(top_level, token, output),
        Some(Kind::Indent) => token_string_indent(top_level, token, output),
        Some(kind) => panic!("not implemented for kind {:?}", kind),
        None => output,
    }
}

fn token_string(tokens: &Tokens) -> String {
    tokens
        .top_level
        .iter()
        .fold(String::from("\n"), |output, top_level| {
            token_string_impl(top_level, 0, output)
        })
}

#[test]
fn test_tokenize_int() {
    let tokens = tokenize("def start(): 0");
    assert_eq!(
        token_string(&tokens),
        r#"
Def
Symbol start
LeftParen
RightParen
Colon
Int 0
"#
    );
}

#[test]
fn test_tokenize_add() {
    let tokens = tokenize("def start(): 5 + 10");
    assert_eq!(
        token_string(&tokens),
        r#"
Def
Symbol start
LeftParen
RightParen
Colon
Int 5
Plus
Int 10
"#
    );
}

#[test]
fn test_tokenize_subtract() {
    let tokens = tokenize("def start(): 5 - 10");
    assert_eq!(
        token_string(&tokens),
        r#"
Def
Symbol start
LeftParen
RightParen
Colon
Int 5
Minus
Int 10
"#
    );
}

#[test]
fn test_tokenize_multiply() {
    let tokens = tokenize("def start(): 5 * 10");
    assert_eq!(
        token_string(&tokens),
        r#"
Def
Symbol start
LeftParen
RightParen
Colon
Int 5
Times
Int 10
"#
    );
}

#[test]
fn test_tokenize_divide() {
    let tokens = tokenize("def start(): 10 / 5");
    assert_eq!(
        token_string(&tokens),
        r#"
Def
Symbol start
LeftParen
RightParen
Colon
Int 10
Slash
Int 5
"#
    );
}

#[test]
fn test_tokenize_local_variables() {
    let source = r#"
def start():
    x = 5
    y = 20
    x + y"#;
    let tokens = tokenize(source);
    assert_eq!(
        token_string(&tokens),
        r#"
Def
Symbol start
LeftParen
RightParen
Colon
Indent 4
Symbol x
Equal
Int 5
Indent 4
Symbol y
Equal
Int 20
Indent 4
Symbol x
Plus
Symbol y
"#
    );
}

#[test]
fn test_tokenize_multiple_functions() {
    let source = r#"
def square(x): x * x

def sum_of_squares(x, y):
    x2 = square(x)
    y2 = square(y)
    x2 + y2

def start(): sum_of_squares(5, 3)"#;
    let tokens = tokenize(source);
    assert_eq!(
        tokens,
        Tokens {
            top_level: vec![
                TopLevel {
                    indices: vec![0, 0, 0, 1, 0, 0, 2, 0, 3],
                    kinds: vec![
                        Kind::Def,
                        Kind::Symbol,
                        Kind::LeftParen,
                        Kind::Symbol,
                        Kind::RightParen,
                        Kind::Colon,
                        Kind::Symbol,
                        Kind::Times,
                        Kind::Symbol
                    ],
                    symbols: strings(["square", "x", "x", "x"]),
                    ints: strings([]),
                    indents: vec![],
                },
                TopLevel {
                    indices: vec![
                        0, 0, 0, 1, 0, 2, 0, 0, 0, 3, 0, 4, 0, 5, 0, 1, 6, 0, 7, 0, 8, 0, 2, 9, 0,
                        10
                    ],
                    kinds: vec![
                        Kind::Def,
                        Kind::Symbol,
                        Kind::LeftParen,
                        Kind::Symbol,
                        Kind::Comma,
                        Kind::Symbol,
                        Kind::RightParen,
                        Kind::Colon,
                        Kind::Indent,
                        Kind::Symbol,
                        Kind::Equal,
                        Kind::Symbol,
                        Kind::LeftParen,
                        Kind::Symbol,
                        Kind::RightParen,
                        Kind::Indent,
                        Kind::Symbol,
                        Kind::Equal,
                        Kind::Symbol,
                        Kind::LeftParen,
                        Kind::Symbol,
                        Kind::RightParen,
                        Kind::Indent,
                        Kind::Symbol,
                        Kind::Plus,
                        Kind::Symbol,
                    ],
                    symbols: strings([
                        "sum_of_squares",
                        "x",
                        "y",
                        "x2",
                        "square",
                        "x",
                        "y2",
                        "square",
                        "y",
                        "x2",
                        "y2"
                    ]),
                    ints: strings([]),
                    indents: vec![4, 4, 4],
                },
                TopLevel {
                    indices: vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0],
                    kinds: vec![
                        Kind::Def,
                        Kind::Symbol,
                        Kind::LeftParen,
                        Kind::RightParen,
                        Kind::Colon,
                        Kind::Symbol,
                        Kind::LeftParen,
                        Kind::Int,
                        Kind::Comma,
                        Kind::Int,
                        Kind::RightParen,
                    ],
                    symbols: strings(["start", "sum_of_squares"]),
                    ints: strings(["5", "3"]),
                    indents: vec![],
                }
            ]
        }
    )
}

#[test]
fn test_tokenize_if() {
    let source = r#"
def min(x, y):
  if x < y:
    x
  else:
    y"#;
    let tokens = tokenize(source);
    assert_eq!(
        tokens,
        Tokens {
            top_level: vec![TopLevel {
                indices: vec![0, 0, 0, 1, 0, 2, 0, 0, 0, 0, 3, 0, 4, 0, 1, 5, 2, 0, 0, 3, 6],
                kinds: vec![
                    Kind::Def,
                    Kind::Symbol,
                    Kind::LeftParen,
                    Kind::Symbol,
                    Kind::Comma,
                    Kind::Symbol,
                    Kind::RightParen,
                    Kind::Colon,
                    Kind::Indent,
                    Kind::If,
                    Kind::Symbol,
                    Kind::LessThan,
                    Kind::Symbol,
                    Kind::Colon,
                    Kind::Indent,
                    Kind::Symbol,
                    Kind::Indent,
                    Kind::Else,
                    Kind::Colon,
                    Kind::Indent,
                    Kind::Symbol,
                ],
                symbols: strings(["min", "x", "y", "x", "y", "x", "y"]),
                ints: vec![],
                indents: vec![2, 4, 2, 4],
            }]
        }
    )
}
