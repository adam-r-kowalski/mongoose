use pretty_assertions::assert_eq;

use ra::tokenizer::{tokenize, Kind, Tokens, TopLevel};
use test_utilities::strings;

#[test]
fn test_tokenize_int() {
    let tokens = tokenize("def start(): 0");
    assert_eq!(
        tokens,
        Tokens {
            top_level: vec![TopLevel {
                indices: vec![0, 0, 0, 0, 0, 0],
                kinds: vec![
                    Kind::Def,
                    Kind::Symbol,
                    Kind::LeftParen,
                    Kind::RightParen,
                    Kind::Colon,
                    Kind::Int
                ],
                symbols: strings(["start"]),
                ints: strings(["0"]),
                indents: vec![],
            }]
        }
    )
}

#[test]
fn test_tokenize_add() {
    let tokens = tokenize("def start(): 5 + 10");
    assert_eq!(
        tokens,
        Tokens {
            top_level: vec![TopLevel {
                indices: vec![0, 0, 0, 0, 0, 0, 0, 1],
                kinds: vec![
                    Kind::Def,
                    Kind::Symbol,
                    Kind::LeftParen,
                    Kind::RightParen,
                    Kind::Colon,
                    Kind::Int,
                    Kind::Plus,
                    Kind::Int
                ],
                symbols: strings(["start"]),
                ints: strings(["5", "10"]),
                indents: vec![],
            }]
        }
    )
}

#[test]
fn test_tokenize_subtract() {
    let tokens = tokenize("def start(): 5 - 10");
    assert_eq!(
        tokens,
        Tokens {
            top_level: vec![TopLevel {
                indices: vec![0, 0, 0, 0, 0, 0, 0, 1],
                kinds: vec![
                    Kind::Def,
                    Kind::Symbol,
                    Kind::LeftParen,
                    Kind::RightParen,
                    Kind::Colon,
                    Kind::Int,
                    Kind::Minus,
                    Kind::Int
                ],
                symbols: strings(["start"]),
                ints: strings(["5", "10"]),
                indents: vec![],
            }]
        }
    )
}

#[test]
fn test_tokenize_multiply() {
    let tokens = tokenize("def start(): 5 * 10");
    assert_eq!(
        tokens,
        Tokens {
            top_level: vec![TopLevel {
                indices: vec![0, 0, 0, 0, 0, 0, 0, 1],
                kinds: vec![
                    Kind::Def,
                    Kind::Symbol,
                    Kind::LeftParen,
                    Kind::RightParen,
                    Kind::Colon,
                    Kind::Int,
                    Kind::Times,
                    Kind::Int
                ],
                symbols: strings(["start"]),
                ints: strings(["5", "10"]),
                indents: vec![],
            }]
        }
    )
}

#[test]
fn test_tokenize_divide() {
    let tokens = tokenize("def start(): 10 / 5");
    assert_eq!(
        tokens,
        Tokens {
            top_level: vec![TopLevel {
                indices: vec![0, 0, 0, 0, 0, 0, 0, 1],
                kinds: vec![
                    Kind::Def,
                    Kind::Symbol,
                    Kind::LeftParen,
                    Kind::RightParen,
                    Kind::Colon,
                    Kind::Int,
                    Kind::Slash,
                    Kind::Int
                ],
                symbols: strings(["start"]),
                ints: strings(["10", "5"]),
                indents: vec![],
            }]
        }
    )
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
        tokens,
        Tokens {
            top_level: vec![TopLevel {
                indices: vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 2, 0, 1, 2, 3, 0, 4],
                kinds: vec![
                    Kind::Def,
                    Kind::Symbol,
                    Kind::LeftParen,
                    Kind::RightParen,
                    Kind::Colon,
                    Kind::Indent,
                    Kind::Symbol,
                    Kind::Equal,
                    Kind::Int,
                    Kind::Indent,
                    Kind::Symbol,
                    Kind::Equal,
                    Kind::Int,
                    Kind::Indent,
                    Kind::Symbol,
                    Kind::Plus,
                    Kind::Symbol,
                ],
                symbols: strings(["start", "x", "y", "x", "y"]),
                ints: strings(["5", "20"]),
                indents: vec![4, 4, 4],
            }]
        }
    )
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
