use std::collections::HashMap;
use std::iter::FromIterator;

use pretty_assertions::assert_eq;

use ra::{
    parser::{parse, Ast, BinaryOp, BinaryOps, Function, Kind},
    tokenizer::tokenize,
};

use test_utilities::strings;

#[test]
fn test_parse_int() {
    let tokens = tokenize("def start(): 0");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            functions: vec![Function {
                name: 0,
                kinds: vec![Kind::Int],
                indices: vec![0],
                binary_ops: BinaryOps {
                    ops: vec![],
                    lefts: vec![],
                    rights: vec![],
                },
                expressions: vec![0]
            }],
            symbols: strings(["start"]),
            ints: strings(["0"]),
            top_level: HashMap::from_iter([(String::from("start"), 0)])
        }
    )
}

#[test]
fn test_parse_add() {
    let tokens = tokenize("def start(): 5 + 10");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            functions: vec![Function {
                name: 0,
                kinds: vec![Kind::Int, Kind::Int, Kind::BinaryOp],
                indices: vec![0, 1, 0],
                binary_ops: BinaryOps {
                    ops: vec![BinaryOp::Add],
                    lefts: vec![0],
                    rights: vec![1],
                },
                expressions: vec![2]
            }],
            symbols: strings(["start"]),
            ints: strings(["5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), 0)])
        }
    )
}

#[test]
fn test_parse_subtract() {
    let tokens = tokenize("def start(): 5 - 10");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            functions: vec![Function {
                name: 0,
                kinds: vec![Kind::Int, Kind::Int, Kind::BinaryOp],
                indices: vec![0, 1, 0],
                binary_ops: BinaryOps {
                    ops: vec![BinaryOp::Subtract],
                    lefts: vec![0],
                    rights: vec![1],
                },
                expressions: vec![2]
            }],
            symbols: strings(["start"]),
            ints: strings(["5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), 0)])
        }
    )
}

#[test]
fn test_parse_multiply() {
    let tokens = tokenize("def start(): 5 * 10");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            functions: vec![Function {
                name: 0,
                kinds: vec![Kind::Int, Kind::Int, Kind::BinaryOp],
                indices: vec![0, 1, 0],
                binary_ops: BinaryOps {
                    ops: vec![BinaryOp::Multiply],
                    lefts: vec![0],
                    rights: vec![1],
                },
                expressions: vec![2]
            }],
            symbols: strings(["start"]),
            ints: strings(["5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), 0)])
        }
    )
}

#[test]
fn test_parse_divide() {
    let tokens = tokenize("def start(): 10 / 5");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            functions: vec![Function {
                name: 0,
                kinds: vec![Kind::Int, Kind::Int, Kind::BinaryOp],
                indices: vec![0, 1, 0],
                binary_ops: BinaryOps {
                    ops: vec![BinaryOp::Divide],
                    lefts: vec![0],
                    rights: vec![1],
                },
                expressions: vec![2]
            }],
            symbols: strings(["start"]),
            ints: strings(["10", "5"]),
            top_level: HashMap::from_iter([(String::from("start"), 0)])
        }
    )
}

#[test]
fn test_parse_add_then_multiply() {
    let tokens = tokenize("def start(): 3 + 5 * 10");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            functions: vec![Function {
                name: 0,
                kinds: vec![
                    Kind::Int,
                    Kind::Int,
                    Kind::Int,
                    Kind::BinaryOp,
                    Kind::BinaryOp
                ],
                indices: vec![0, 1, 2, 0, 1],
                binary_ops: BinaryOps {
                    ops: vec![BinaryOp::Multiply, BinaryOp::Add],
                    lefts: vec![1, 0],
                    rights: vec![2, 3],
                },
                expressions: vec![4]
            }],
            symbols: strings(["start"]),
            ints: strings(["3", "5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), 0)])
        }
    )
}

#[test]
fn test_parse_multiply_then_add() {
    let tokens = tokenize("def start(): 3 * 5 + 10");
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            functions: vec![Function {
                name: 0,
                kinds: vec![
                    Kind::Int,
                    Kind::Int,
                    Kind::BinaryOp,
                    Kind::Int,
                    Kind::BinaryOp
                ],
                indices: vec![0, 1, 0, 2, 1],
                binary_ops: BinaryOps {
                    ops: vec![BinaryOp::Multiply, BinaryOp::Add],
                    lefts: vec![0, 2],
                    rights: vec![1, 3],
                },
                expressions: vec![4]
            }],
            symbols: strings(["start"]),
            ints: strings(["3", "5", "10"]),
            top_level: HashMap::from_iter([(String::from("start"), 0)])
        }
    )
}

// #[test]
// fn test_parse_local_variables() {
//     let source = r#"
// def start():
//     x = 5
//     y = 20
//     x + y"#;
//     let tokens = tokenize(source);
//     let ast = parse(tokens);
//     assert_eq!(
//         ast,
//         Ast {
//             kinds: vec![
//                 Kind::Symbol,
//                 Kind::Int,
//                 Kind::Int,
//                 Kind::BinaryOp,
//                 Kind::Int,
//                 Kind::BinaryOp,
//                 Kind::Function
//             ],
//             indices: vec![0, 0, 1, 0, 2, 1, 0],
//             functions: Functions {
//                 names: vec![Entity(0)],
//                 bodies: vec![Entity(5)],
//             },
//             binary_ops: BinaryOps {
//                 ops: vec![BinaryOp::Multiply, BinaryOp::Add],
//                 lefts: vec![Entity(1), Entity(3)],
//                 rights: vec![Entity(2), Entity(4)],
//             },
//             symbols: strings(["start"]),
//             ints: strings(["3", "5", "10"]),
//             top_level: HashMap::from_iter([(String::from("start"), Entity(6))])
//         }
//     )
// }
