use pretty_assertions::assert_eq;

use ra::{
    parser::parse,
    tokenizer::tokenize,
    codegen::{codegen, Wasm},
};


#[test]
fn test_parser() {
    let tokens = tokenize("start() -> i64 = 0");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    assert_eq!(wasm, Wasm{});
}
