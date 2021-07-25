use pretty_assertions::assert_eq;

use ra::tokenizer::{tokenize, Kind, Tokens};
use test_utilities::strings;

#[test]
fn test_start() {
    let tokens = tokenize("start() -> i64 = 0");
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
                Kind::Equal,
                Kind::Int
            ],
            symbols: strings(["start", "i64"]),
            ints: strings(["0"]),
        }
    )
}
