use std::collections::HashMap;
use std::iter::FromIterator;

use pretty_assertions::assert_eq;

use ra::{
    parser::{parse, Ast, BinaryOp, BinaryOps, Definitions, Function, FunctionCalls, Kind},
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
                arguments: vec![],
                kinds: vec![Kind::Int],
                indices: vec![0],
                binary_ops: BinaryOps {
                    ops: vec![],
                    lefts: vec![],
                    rights: vec![],
                },
                definitions: Definitions {
                    names: vec![],
                    values: vec![],
                },
                function_calls: FunctionCalls {
                    names: vec![],
                    parameters: vec![],
                },
                expressions: vec![0],
                symbols: strings(["start"]),
                ints: strings(["0"]),
            }],
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
                arguments: vec![],
                kinds: vec![Kind::Int, Kind::Int, Kind::BinaryOp],
                indices: vec![0, 1, 0],
                binary_ops: BinaryOps {
                    ops: vec![BinaryOp::Add],
                    lefts: vec![0],
                    rights: vec![1],
                },
                definitions: Definitions {
                    names: vec![],
                    values: vec![],
                },
                function_calls: FunctionCalls {
                    names: vec![],
                    parameters: vec![],
                },
                expressions: vec![2],
                symbols: strings(["start"]),
                ints: strings(["5", "10"]),
            }],
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
                arguments: vec![],
                kinds: vec![Kind::Int, Kind::Int, Kind::BinaryOp],
                indices: vec![0, 1, 0],
                binary_ops: BinaryOps {
                    ops: vec![BinaryOp::Subtract],
                    lefts: vec![0],
                    rights: vec![1],
                },
                definitions: Definitions {
                    names: vec![],
                    values: vec![],
                },
                function_calls: FunctionCalls {
                    names: vec![],
                    parameters: vec![],
                },
                expressions: vec![2],
                symbols: strings(["start"]),
                ints: strings(["5", "10"]),
            }],
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
                arguments: vec![],
                kinds: vec![Kind::Int, Kind::Int, Kind::BinaryOp],
                indices: vec![0, 1, 0],
                binary_ops: BinaryOps {
                    ops: vec![BinaryOp::Multiply],
                    lefts: vec![0],
                    rights: vec![1],
                },
                definitions: Definitions {
                    names: vec![],
                    values: vec![],
                },
                function_calls: FunctionCalls {
                    names: vec![],
                    parameters: vec![],
                },
                expressions: vec![2],
                symbols: strings(["start"]),
                ints: strings(["5", "10"]),
            }],
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
                arguments: vec![],
                kinds: vec![Kind::Int, Kind::Int, Kind::BinaryOp],
                indices: vec![0, 1, 0],
                binary_ops: BinaryOps {
                    ops: vec![BinaryOp::Divide],
                    lefts: vec![0],
                    rights: vec![1],
                },
                definitions: Definitions {
                    names: vec![],
                    values: vec![],
                },
                function_calls: FunctionCalls {
                    names: vec![],
                    parameters: vec![],
                },
                expressions: vec![2],
                symbols: strings(["start"]),
                ints: strings(["10", "5"]),
            }],
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
                arguments: vec![],
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
                definitions: Definitions {
                    names: vec![],
                    values: vec![],
                },
                function_calls: FunctionCalls {
                    names: vec![],
                    parameters: vec![],
                },
                expressions: vec![4],
                symbols: strings(["start"]),
                ints: strings(["3", "5", "10"]),
            }],
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
                arguments: vec![],
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
                definitions: Definitions {
                    names: vec![],
                    values: vec![],
                },
                function_calls: FunctionCalls {
                    names: vec![],
                    parameters: vec![],
                },
                expressions: vec![4],
                symbols: strings(["start"]),
                ints: strings(["3", "5", "10"]),
            }],
            top_level: HashMap::from_iter([(String::from("start"), 0)])
        }
    )
}

#[test]
fn test_parse_local_variables() {
    let source = r#"
 def start():
     x = 5
     y = 20
     x + y"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            functions: vec![Function {
                name: 0,
                arguments: vec![],
                kinds: vec![
                    Kind::Symbol,
                    Kind::Int,
                    Kind::Definition,
                    Kind::Symbol,
                    Kind::Int,
                    Kind::Definition,
                    Kind::Symbol,
                    Kind::Symbol,
                    Kind::BinaryOp,
                ],
                indices: vec![1, 0, 0, 2, 1, 1, 3, 4, 0],
                binary_ops: BinaryOps {
                    ops: vec![BinaryOp::Add],
                    lefts: vec![6],
                    rights: vec![7],
                },
                definitions: Definitions {
                    names: vec![0, 3],
                    values: vec![1, 4],
                },
                function_calls: FunctionCalls {
                    names: vec![],
                    parameters: vec![],
                },
                expressions: vec![2, 5, 8],
                symbols: strings(["start", "x", "y", "x", "y"]),
                ints: strings(["5", "20"]),
            }],
            top_level: HashMap::from_iter([(String::from("start"), 0)])
        }
    )
}

#[test]
fn test_parse_multiple_functions() {
    let source = r#"
def square(x): x * x

def sum_of_squares(x, y):
    x2 = square(x)
    y2 = square(y)
    x2 + y2

def start(): sum_of_squares(5, 3)"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            functions: vec![
                Function {
                    name: 0,
                    arguments: vec![1],
                    kinds: vec![Kind::Symbol, Kind::Symbol, Kind::BinaryOp,],
                    indices: vec![2, 3, 0],
                    binary_ops: BinaryOps {
                        ops: vec![BinaryOp::Multiply],
                        lefts: vec![0],
                        rights: vec![1],
                    },
                    definitions: Definitions {
                        names: vec![],
                        values: vec![],
                    },
                    function_calls: FunctionCalls {
                        names: vec![],
                        parameters: vec![],
                    },
                    expressions: vec![2],
                    symbols: strings(["square", "x", "x", "x"]),
                    ints: vec![],
                },
                Function {
                    name: 0,
                    arguments: vec![1, 2],
                    kinds: vec![
                        Kind::Symbol,
                        Kind::Symbol,
                        Kind::Symbol,
                        Kind::FunctionCall,
                        Kind::Definition,
                        Kind::Symbol,
                        Kind::Symbol,
                        Kind::Symbol,
                        Kind::FunctionCall,
                        Kind::Definition,
                        Kind::Symbol,
                        Kind::Symbol,
                        Kind::BinaryOp,
                    ],
                    indices: vec![3, 4, 5, 0, 0, 6, 7, 8, 1, 1, 9, 10, 0],
                    binary_ops: BinaryOps {
                        ops: vec![BinaryOp::Add],
                        lefts: vec![10],
                        rights: vec![11],
                    },
                    definitions: Definitions {
                        names: vec![0, 5],
                        values: vec![3, 8],
                    },
                    function_calls: FunctionCalls {
                        names: vec![1, 6],
                        parameters: vec![vec![2], vec![7]],
                    },
                    expressions: vec![4, 9, 12],
                    symbols: strings([
                        "sum_of_squares",
                        "x",
                        "y",
                        "x2",
                        "square",
                        "x",
                        "y2",
                        "square",
                        "y",
                        "x2",
                        "y2"
                    ]),
                    ints: vec![],
                },
                Function {
                    name: 0,
                    arguments: vec![],
                    kinds: vec![Kind::Symbol, Kind::Int, Kind::Int, Kind::FunctionCall],
                    indices: vec![1, 0, 1, 0],
                    binary_ops: BinaryOps {
                        ops: vec![],
                        lefts: vec![],
                        rights: vec![],
                    },
                    definitions: Definitions {
                        names: vec![],
                        values: vec![],
                    },
                    function_calls: FunctionCalls {
                        names: vec![0],
                        parameters: vec![vec![1, 2]],
                    },
                    expressions: vec![3],
                    symbols: strings(["start", "sum_of_squares"]),
                    ints: strings(["5", "3"]),
                }
            ],
            top_level: HashMap::from_iter([
                (String::from("square"), 0),
                (String::from("sum_of_squares"), 1),
                (String::from("start"), 2),
            ])
        }
    )
}
