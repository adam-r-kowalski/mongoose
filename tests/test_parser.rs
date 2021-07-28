use pretty_assertions::assert_eq;
use std::collections::HashMap;

use ra::{
    parser::{parse, Ast, BinaryOperator, BinaryOperators, Entity, Functions, Kind},
    tokenizer::tokenize,
};

use test_utilities::strings;

#[test]
fn test_parse_literal() {
    let tokens = tokenize("start() -> i64: 0");
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
            binary_operators: BinaryOperators {
                operators: vec![],
                lefts: vec![],
                rights: vec![],
            },
            symbols: strings(["start", "i64"]),
            ints: strings(["0"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(3))])
        }
    )
}

#[test]
fn test_parse_operator() {
    let tokens = tokenize("start() -> i64: 5 + 10");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            kinds: vec![
                Kind::Symbol,
                Kind::Symbol,
                Kind::Int,
                Kind::Int,
                Kind::BinaryOperator,
                Kind::Function
            ],
            indices: vec![0, 1, 0, 1, 0, 0],
            functions: Functions {
                names: vec![Entity(0)],
                return_types: vec![Entity(1)],
                bodies: vec![Entity(4)],
            },
            binary_operators: BinaryOperators {
                operators: vec![BinaryOperator::Add],
                lefts: vec![Entity(2)],
                rights: vec![Entity(3)],
            },
            symbols: strings(["start", "i64"]),
            ints: strings(["5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(5))])
        }
    )
}
