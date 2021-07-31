use pretty_assertions::assert_eq;

use ra::tokenizer::{tokenize, Kind, Tokens};
use test_utilities::strings;

#[test]
fn test_tokenize_int() {
    let tokens = tokenize("start(): 0");
    assert_eq!(
        tokens,
        Tokens {
            indices: vec![0, 0, 0, 0, 0],
            kinds: vec![
                Kind::Symbol,
                Kind::LeftParen,
                Kind::RightParen,
                Kind::Colon,
                Kind::Int
            ],
            symbols: strings(["start"]),
            ints: strings(["0"]),
        }
    )
}

#[test]
fn test_tokenize_add() {
    let tokens = tokenize("start(): 5 + 10");
    assert_eq!(
        tokens,
        Tokens {
            indices: vec![0, 0, 0, 0, 0, 0, 1],
            kinds: vec![
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
        }
    )
}

#[test]
fn test_tokenize_subtract() {
    let tokens = tokenize("start(): 5 - 10");
    assert_eq!(
        tokens,
        Tokens {
            indices: vec![0, 0, 0, 0, 0, 0, 1],
            kinds: vec![
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
        }
    )
}

#[test]
fn test_tokenize_multiply() {
    let tokens = tokenize("start(): 5 * 10");
    assert_eq!(
        tokens,
        Tokens {
            indices: vec![0, 0, 0, 0, 0, 0, 1],
            kinds: vec![
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
        }
    )
}

#[test]
fn test_tokenize_divide() {
    let tokens = tokenize("start(): 10 / 5");
    assert_eq!(
        tokens,
        Tokens {
            indices: vec![0, 0, 0, 0, 0, 0, 1],
            kinds: vec![
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
        }
    )
}
