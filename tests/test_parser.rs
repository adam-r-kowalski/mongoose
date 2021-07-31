use std::collections::HashMap;
use std::iter::FromIterator;

use pretty_assertions::assert_eq;

use ra::{
    parser::{parse, Ast, BinaryOp, BinaryOps, Entity, Functions, Kind},
    tokenizer::tokenize,
};

use test_utilities::strings;

#[test]
fn test_parse_int() {
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
            binary_ops: BinaryOps {
                ops: vec![],
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
fn test_parse_add() {
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
                Kind::BinaryOp,
                Kind::Function
            ],
            indices: vec![0, 1, 0, 1, 0, 0],
            functions: Functions {
                names: vec![Entity(0)],
                return_types: vec![Entity(1)],
                bodies: vec![Entity(4)],
            },
            binary_ops: BinaryOps {
                ops: vec![BinaryOp::Add],
                lefts: vec![Entity(2)],
                rights: vec![Entity(3)],
            },
            symbols: strings(["start", "i64"]),
            ints: strings(["5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(5))])
        }
    )
}

#[test]
fn test_parse_subtract() {
    let tokens = tokenize("start() -> i64: 5 - 10");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            kinds: vec![
                Kind::Symbol,
                Kind::Symbol,
                Kind::Int,
                Kind::Int,
                Kind::BinaryOp,
                Kind::Function
            ],
            indices: vec![0, 1, 0, 1, 0, 0],
            functions: Functions {
                names: vec![Entity(0)],
                return_types: vec![Entity(1)],
                bodies: vec![Entity(4)],
            },
            binary_ops: BinaryOps {
                ops: vec![BinaryOp::Subtract],
                lefts: vec![Entity(2)],
                rights: vec![Entity(3)],
            },
            symbols: strings(["start", "i64"]),
            ints: strings(["5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(5))])
        }
    )
}

#[test]
fn test_parse_multiply() {
    let tokens = tokenize("start() -> i64: 5 * 10");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            kinds: vec![
                Kind::Symbol,
                Kind::Symbol,
                Kind::Int,
                Kind::Int,
                Kind::BinaryOp,
                Kind::Function
            ],
            indices: vec![0, 1, 0, 1, 0, 0],
            functions: Functions {
                names: vec![Entity(0)],
                return_types: vec![Entity(1)],
                bodies: vec![Entity(4)],
            },
            binary_ops: BinaryOps {
                ops: vec![BinaryOp::Multiply],
                lefts: vec![Entity(2)],
                rights: vec![Entity(3)],
            },
            symbols: strings(["start", "i64"]),
            ints: strings(["5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(5))])
        }
    )
}

#[test]
fn test_parse_divide() {
    let tokens = tokenize("start() -> i64: 10 / 5");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            kinds: vec![
                Kind::Symbol,
                Kind::Symbol,
                Kind::Int,
                Kind::Int,
                Kind::BinaryOp,
                Kind::Function
            ],
            indices: vec![0, 1, 0, 1, 0, 0],
            functions: Functions {
                names: vec![Entity(0)],
                return_types: vec![Entity(1)],
                bodies: vec![Entity(4)],
            },
            binary_ops: BinaryOps {
                ops: vec![BinaryOp::Divide],
                lefts: vec![Entity(2)],
                rights: vec![Entity(3)],
            },
            symbols: strings(["start", "i64"]),
            ints: strings(["10", "5"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(5))])
        }
    )
}

#[test]
fn test_parse_add_then_multiply() {
    let tokens = tokenize("start() -> i64: 3 + 5 * 10");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            kinds: vec![
                Kind::Symbol,
                Kind::Symbol,
                Kind::Int,
                Kind::Int,
                Kind::Int,
                Kind::BinaryOp,
                Kind::BinaryOp,
                Kind::Function
            ],
            indices: vec![0, 1, 0, 1, 2, 0, 1, 0],
            functions: Functions {
                names: vec![Entity(0)],
                return_types: vec![Entity(1)],
                bodies: vec![Entity(6)],
            },
            binary_ops: BinaryOps {
                ops: vec![BinaryOp::Multiply, BinaryOp::Add],
                lefts: vec![Entity(3), Entity(2)],
                rights: vec![Entity(4), Entity(5)],
            },
            symbols: strings(["start", "i64"]),
            ints: strings(["3", "5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(7))])
        }
    )
}

#[test]
fn test_parse_multiply_then_add() {
    let tokens = tokenize("start() -> i64: 3 * 5 + 10");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            kinds: vec![
                Kind::Symbol,
                Kind::Symbol,
                Kind::Int,
                Kind::Int,
                Kind::BinaryOp,
                Kind::Int,
                Kind::BinaryOp,
                Kind::Function
            ],
            indices: vec![0, 1, 0, 1, 0, 2, 1, 0],
            functions: Functions {
                names: vec![Entity(0)],
                return_types: vec![Entity(1)],
                bodies: vec![Entity(6)],
            },
            binary_ops: BinaryOps {
                ops: vec![BinaryOp::Multiply, BinaryOp::Add],
                lefts: vec![Entity(2), Entity(4)],
                rights: vec![Entity(3), Entity(5)],
            },
            symbols: strings(["start", "i64"]),
            ints: strings(["3", "5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(7))])
        }
    )
}
