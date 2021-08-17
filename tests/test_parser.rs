use std::collections::HashMap;
use std::iter::FromIterator;

use pretty_assertions::assert_eq;

use smith::{
    parser::{parse, Ast, BinaryOp, BinaryOps, Definitions, Function, FunctionCalls, Ifs, Kind},
    tokenizer::tokenize,
};

use test_utilities::strings;

fn write_indent(mut output: String, indent: usize) -> String {
    output.push_str(&String::from_utf8(vec![b' '; indent]).unwrap());
    output
}

fn ast_string_int(mut output: String, func: &Function, expression: usize) -> String {
    output.push_str("Int(");
    output.push_str(&func.ints[func.indices[expression]]);
    output.push_str("),\n");
    output
}

const INDENT: usize = 4;

fn ast_string_binary_op(
    mut output: String,
    func: &Function,
    expression: usize,
    indent: usize,
) -> String {
    output.push_str("BinaryOp(\n");
    let mut output = write_indent(output, indent);
    output.push_str("op=");
    let index = func.indices[expression];
    match func.binary_ops.ops[index] {
        BinaryOp::Add => output.push_str("Plus"),
        op => panic!("ast string binary op not implemented for {:?}", op),
    };
    output.push_str(",\n");
    let mut output = write_indent(output, indent);
    output.push_str("left=");
    let output = ast_string_expression(output, func, func.binary_ops.lefts[index], indent);
    let mut output = write_indent(output, indent);
    output.push_str("right=");
    let output = ast_string_expression(output, func, func.binary_ops.rights[index], indent);
    let mut output = write_indent(output, indent - INDENT);
    output.push_str("),\n");
    output
}

fn ast_string_expression(
    output: String,
    func: &Function,
    expression: usize,
    indent: usize,
) -> String {
    match func.kinds[expression] {
        Kind::Int => ast_string_int(output, func, expression),
        Kind::BinaryOp => ast_string_binary_op(output, func, expression, indent + INDENT),
        kind => panic!("ast string expression not implemented for {:?}!!!", kind),
    }
}

fn ast_string_function(mut output: String, func: &Function) -> String {
    output.push_str("    Function(\n");
    output.push_str("        name=");
    output.push_str(&func.symbols[func.name]);
    output.push_str(",\n        arguments=[\n");
    let mut output = func.arguments.iter().fold(output, |mut output, &argument| {
        output.push_str("            ");
        output.push_str(&func.symbols[func.indices[argument]]);
        output
    });
    output.push_str("        ],\n        body=[\n");
    let mut output = func.expressions.iter().fold(output, |output, &expression| {
        let indent = 12;
        let output = write_indent(output, indent);
        ast_string_expression(output, func, expression, indent)
    });
    output.push_str("        ]\n    )\n");
    output
}

fn ast_string(ast: &Ast) -> String {
    let mut output = ast
        .functions
        .iter()
        .fold(String::from("\nAst([\n"), ast_string_function);
    output.push_str("])\n");
    output
}

#[test]
fn test_parse_int() {
    let tokens = tokenize("def start(): 0");
    let ast = parse(tokens);
    assert_eq!(
        ast_string(&ast),
        r#"
Ast([
    Function(
        name=start,
        arguments=[
        ],
        body=[
            Int(0),
        ]
    )
])
"#
    );
}

#[test]
fn test_parse_add() {
    let tokens = tokenize("def start(): 5 + 10");
    let ast = parse(tokens);
    assert_eq!(
        ast_string(&ast),
        r#"
Ast([
    Function(
        name=start,
        arguments=[
        ],
        body=[
            BinaryOp(
                op=Plus,
                left=Int(5),
                right=Int(10),
            ),
        ]
    )
])
"#
    );
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
                ifs: Ifs {
                    conditionals: vec![],
                    then_branches: vec![],
                    else_branches: vec![],
                }
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
                ifs: Ifs {
                    conditionals: vec![],
                    then_branches: vec![],
                    else_branches: vec![],
                }
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
                ifs: Ifs {
                    conditionals: vec![],
                    then_branches: vec![],
                    else_branches: vec![],
                }
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
                ifs: Ifs {
                    conditionals: vec![],
                    then_branches: vec![],
                    else_branches: vec![],
                }
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
                ifs: Ifs {
                    conditionals: vec![],
                    then_branches: vec![],
                    else_branches: vec![],
                }
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
                ifs: Ifs {
                    conditionals: vec![],
                    then_branches: vec![],
                    else_branches: vec![],
                }
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
                ifs: Ifs {
                    conditionals: vec![],
                    then_branches: vec![],
                    else_branches: vec![],
                }
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
                    ifs: Ifs {
                        conditionals: vec![],
                        then_branches: vec![],
                        else_branches: vec![],
                    }
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
                    ifs: Ifs {
                        conditionals: vec![],
                        then_branches: vec![],
                        else_branches: vec![],
                    }
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
                    ifs: Ifs {
                        conditionals: vec![],
                        then_branches: vec![],
                        else_branches: vec![],
                    }
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

#[test]
fn test_parse_single_line_if() {
    let source = r#"
def min(x, y):
  if x < y: x else: y"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        ast,
        Ast {
            functions: vec![Function {
                name: 0,
                arguments: vec![1, 2],
                kinds: vec![
                    Kind::Symbol,
                    Kind::Symbol,
                    Kind::BinaryOp,
                    Kind::Symbol,
                    Kind::Symbol,
                    Kind::If
                ],
                indices: vec![3, 4, 0, 5, 6, 0],
                binary_ops: BinaryOps {
                    ops: vec![BinaryOp::LessThan],
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
                expressions: vec![5],
                symbols: strings(["min", "x", "y", "x", "y", "x", "y"]),
                ints: vec![],
                ifs: Ifs {
                    conditionals: vec![2],
                    then_branches: vec![vec![3]],
                    else_branches: vec![vec![4]],
                }
            }],
            top_level: HashMap::from_iter([(String::from("min"), 0)])
        }
    )
}

#[test]
fn test_parse_multi_line_if() {
    let source = r#"
def main():
  a = 5
  b = 10
  if a < b:
    c = 7
    a + b + c
  else:
    d = 8
    a * b * c"#;
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
                    Kind::Symbol,
                    Kind::Int,
                    Kind::Definition,
                    Kind::Symbol,
                    Kind::Symbol,
                    Kind::Symbol,
                    Kind::BinaryOp,
                    Kind::BinaryOp,
                    Kind::Symbol,
                    Kind::Int,
                    Kind::Definition,
                    Kind::Symbol,
                    Kind::Symbol,
                    Kind::Symbol,
                    Kind::BinaryOp,
                    Kind::BinaryOp,
                    Kind::If
                ],
                indices: vec![
                    1, 0, 0, 2, 1, 1, 3, 4, 0, 5, 2, 2, 6, 7, 8, 1, 2, 9, 3, 3, 10, 11, 12, 3, 4,
                    0,
                ],
                binary_ops: BinaryOps {
                    ops: vec![
                        BinaryOp::LessThan,
                        BinaryOp::Add,
                        BinaryOp::Add,
                        BinaryOp::Multiply,
                        BinaryOp::Multiply
                    ],
                    lefts: vec![6, 13, 12, 21, 20],
                    rights: vec![7, 14, 15, 22, 23],
                },
                definitions: Definitions {
                    names: vec![0, 3, 9, 17],
                    values: vec![1, 4, 10, 18],
                },
                function_calls: FunctionCalls {
                    names: vec![],
                    parameters: vec![],
                },
                expressions: vec![2, 5, 25],
                symbols: strings([
                    "main", "a", "b", "a", "b", "c", "a", "b", "c", "d", "a", "b", "c"
                ]),
                ints: strings(["5", "10", "7", "8"]),
                ifs: Ifs {
                    conditionals: vec![8],
                    then_branches: vec![vec![11, 16]],
                    else_branches: vec![vec![19, 24]],
                }
            }],
            top_level: HashMap::from_iter([(String::from("main"), 0)])
        }
    )
}
