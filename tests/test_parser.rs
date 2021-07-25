use std::collections::HashMap;
use pretty_assertions::assert_eq;

use ra::{
    parser::{parse, Ast, Functions, Kind, Entity},
    tokenizer::tokenize,
};

use test_utilities::strings;

#[test]
fn test_parser() {
    let tokens = tokenize("start() -> i64 = 0");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            kinds: vec![Kind::Symbol, Kind::Symbol, Kind::Int, Kind::Function],
            indices: vec![0, 1, 0, 0],
            functions: Functions {
                names: vec![Entity(0)],
                return_types: vec![Entity(1)],
                bodies: vec![Entity(2)],
            },
            symbols: strings(["start", "i64"]),
            ints: strings(["0"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(3))])
        }
    )
}
