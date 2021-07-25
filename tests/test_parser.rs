use ra::{parser::parse, tokenizer::tokenize};

#[test]
fn test_parser() {
    let tokens = tokenize("start() -> i64 = 0");
    let _ast = parse(tokens);
}
