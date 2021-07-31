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
    let tokens = tokenize("start(): 0");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            kinds: vec![Kind::Symbol, Kind::Int, Kind::Function],
            indices: vec![0, 0, 0],
            functions: Functions {
                names: vec![Entity(0)],
                bodies: vec![Entity(1)],
            },
            binary_ops: BinaryOps {
                ops: vec![],
                lefts: vec![],
                rights: vec![],
            },
            symbols: strings(["start"]),
            ints: strings(["0"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(2))])
        }
    )
}

#[test]
fn test_parse_add() {
    let tokens = tokenize("start(): 5 + 10");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            kinds: vec![
                Kind::Symbol,
                Kind::Int,
                Kind::Int,
                Kind::BinaryOp,
                Kind::Function
            ],
            indices: vec![0, 0, 1, 0, 0],
            functions: Functions {
                names: vec![Entity(0)],
                bodies: vec![Entity(3)],
            },
            binary_ops: BinaryOps {
                ops: vec![BinaryOp::Add],
                lefts: vec![Entity(1)],
                rights: vec![Entity(2)],
            },
            symbols: strings(["start"]),
            ints: strings(["5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(4))])
        }
    )
}

#[test]
fn test_parse_subtract() {
    let tokens = tokenize("start(): 5 - 10");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            kinds: vec![
                Kind::Symbol,
                Kind::Int,
                Kind::Int,
                Kind::BinaryOp,
                Kind::Function
            ],
            indices: vec![0, 0, 1, 0, 0],
            functions: Functions {
                names: vec![Entity(0)],
                bodies: vec![Entity(3)],
            },
            binary_ops: BinaryOps {
                ops: vec![BinaryOp::Subtract],
                lefts: vec![Entity(1)],
                rights: vec![Entity(2)],
            },
            symbols: strings(["start"]),
            ints: strings(["5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(4))])
        }
    )
}

#[test]
fn test_parse_multiply() {
    let tokens = tokenize("start(): 5 * 10");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            kinds: vec![
                Kind::Symbol,
                Kind::Int,
                Kind::Int,
                Kind::BinaryOp,
                Kind::Function
            ],
            indices: vec![0, 0, 1, 0, 0],
            functions: Functions {
                names: vec![Entity(0)],
                bodies: vec![Entity(3)],
            },
            binary_ops: BinaryOps {
                ops: vec![BinaryOp::Multiply],
                lefts: vec![Entity(1)],
                rights: vec![Entity(2)],
            },
            symbols: strings(["start"]),
            ints: strings(["5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(4))])
        }
    )
}

#[test]
fn test_parse_divide() {
    let tokens = tokenize("start(): 10 / 5");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            kinds: vec![
                Kind::Symbol,
                Kind::Int,
                Kind::Int,
                Kind::BinaryOp,
                Kind::Function
            ],
            indices: vec![0, 0, 1, 0, 0],
            functions: Functions {
                names: vec![Entity(0)],
                bodies: vec![Entity(3)],
            },
            binary_ops: BinaryOps {
                ops: vec![BinaryOp::Divide],
                lefts: vec![Entity(1)],
                rights: vec![Entity(2)],
            },
            symbols: strings(["start"]),
            ints: strings(["10", "5"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(4))])
        }
    )
}

#[test]
fn test_parse_add_then_multiply() {
    let tokens = tokenize("start(): 3 + 5 * 10");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            kinds: vec![
                Kind::Symbol,
                Kind::Int,
                Kind::Int,
                Kind::Int,
                Kind::BinaryOp,
                Kind::BinaryOp,
                Kind::Function
            ],
            indices: vec![0, 0, 1, 2, 0, 1, 0],
            functions: Functions {
                names: vec![Entity(0)],
                bodies: vec![Entity(5)],
            },
            binary_ops: BinaryOps {
                ops: vec![BinaryOp::Multiply, BinaryOp::Add],
                lefts: vec![Entity(2), Entity(1)],
                rights: vec![Entity(3), Entity(4)],
            },
            symbols: strings(["start"]),
            ints: strings(["3", "5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(6))])
        }
    )
}

#[test]
fn test_parse_multiply_then_add() {
    let tokens = tokenize("start(): 3 * 5 + 10");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            kinds: vec![
                Kind::Symbol,
                Kind::Int,
                Kind::Int,
                Kind::BinaryOp,
                Kind::Int,
                Kind::BinaryOp,
                Kind::Function
            ],
            indices: vec![0, 0, 1, 0, 2, 1, 0],
            functions: Functions {
                names: vec![Entity(0)],
                bodies: vec![Entity(5)],
            },
            binary_ops: BinaryOps {
                ops: vec![BinaryOp::Multiply, BinaryOp::Add],
                lefts: vec![Entity(1), Entity(3)],
                rights: vec![Entity(2), Entity(4)],
            },
            symbols: strings(["start"]),
            ints: strings(["3", "5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), Entity(6))])
        }
    )
}
