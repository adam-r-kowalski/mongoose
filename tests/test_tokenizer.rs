use pretty_assertions::assert_eq;

use ra::tokenizer::{tokenize, Kind, Tokens};
use test_utilities::strings;

#[test]
fn test_tokenize_literal() {
    let tokens = tokenize("start() -> i64: 0");
    assert_eq!(
        tokens,
        Tokens {
            indices: vec![0, 0, 0, 0, 1, 0, 0],
            kinds: vec![
                Kind::Symbol,
                Kind::LeftParen,
                Kind::RightParen,
                Kind::Arrow,
                Kind::Symbol,
                Kind::Colon,
                Kind::Int
            ],
            symbols: strings(["start", "i64"]),
            ints: strings(["0"]),
        }
    )
}

#[test]
fn test_tokenize_operator() {
    let tokens = tokenize("start() -> i64: 5 + 10");
    assert_eq!(
        tokens,
        Tokens {
            indices: vec![0, 0, 0, 0, 1, 0, 0, 0, 1],
            kinds: vec![
                Kind::Symbol,
                Kind::LeftParen,
                Kind::RightParen,
                Kind::Arrow,
                Kind::Symbol,
                Kind::Colon,
                Kind::Int,
                Kind::Plus,
                Kind::Int
            ],
            symbols: strings(["start", "i64"]),
            ints: strings(["5", "10"]),
        }
    )
}
