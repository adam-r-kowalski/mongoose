use ra::tokenizer::{tokenize, Kind, Tokens};

#[test]
fn test_start() {
    let tokens = tokenize("start() -> i64 = 0");
    assert_eq!(
        tokens,
        Tokens {
            indices: vec![0, 0, 0, 0, 1, 0, 2],
            kinds: vec![
                Kind::Symbol,
                Kind::LeftParen,
                Kind::RightParen,
                Kind::Arrow,
                Kind::Symbol,
                Kind::Equal,
                Kind::Int
            ],
            strings: vec!["start", "i64", "0"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    )
}
