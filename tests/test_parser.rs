use pretty_assertions::assert_eq;

use mongoose::{
    parser::{parse, Ast, BinaryOp, Function, Kind},
    tokenizer::tokenize,
};

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

fn ast_string_symbol(mut output: String, func: &Function, expression: usize) -> String {
    output.push_str("Symbol(");
    output.push_str(&func.symbols[func.indices[expression]]);
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
        BinaryOp::Add => output.push_str("Add"),
        BinaryOp::Subtract => output.push_str("Subtract"),
        BinaryOp::Multiply => output.push_str("Multiply"),
        BinaryOp::Divide => output.push_str("Divide"),
        BinaryOp::Modulo => output.push_str("Modulo"),
        BinaryOp::Equal => output.push_str("Equal"),
        BinaryOp::LessThan => output.push_str("LessThan"),
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

fn ast_string_definition(
    mut output: String,
    func: &Function,
    expression: usize,
    indent: usize,
) -> String {
    output.push_str("Definition(\n");
    let mut output = write_indent(output, indent);
    output.push_str("name=");
    let index = func.indices[expression];
    output.push_str(&func.symbols[func.indices[func.definitions.names[index]]]);
    output.push_str(",\n");
    let mut output = write_indent(output, indent);
    output.push_str("value=");
    let output = ast_string_expression(output, func, func.definitions.values[index], indent);
    let mut output = write_indent(output, indent - INDENT);
    output.push_str("),\n");
    output
}

fn ast_string_function_call(
    mut output: String,
    func: &Function,
    expression: usize,
    indent: usize,
) -> String {
    output.push_str("FunctionCall(\n");
    let mut output = write_indent(output, indent);
    output.push_str("name=");
    let index = func.indices[expression];
    output.push_str(&func.symbols[func.indices[func.function_calls.names[index]]]);
    output.push_str(",\n");
    let mut output = write_indent(output, indent);
    output.push_str("parameters=[\n");
    let indent = indent + INDENT;
    let output = func.function_calls.parameters[index]
        .iter()
        .fold(output, |output, &parameter| {
            let output = write_indent(output, indent);
            ast_string_expression(output, func, parameter, indent)
        });
    let mut output = write_indent(output, indent - INDENT);
    output.push_str("]\n");
    let mut output = write_indent(output, indent - 2 * INDENT);
    output.push_str("),\n");
    output
}

fn ast_string_if(mut output: String, func: &Function, expression: usize, indent: usize) -> String {
    output.push_str("If(\n");
    let mut output = write_indent(output, indent);
    output.push_str("condition=");
    let index = func.indices[expression];
    let output = ast_string_expression(output, func, func.ifs.conditionals[index], indent);
    let mut output = write_indent(output, indent);
    output.push_str("then=[\n");
    let next_indent = indent + INDENT;
    let output = func.ifs.then_branches[index]
        .iter()
        .fold(output, |output, &parameter| {
            let output = write_indent(output, next_indent);
            ast_string_expression(output, func, parameter, next_indent)
        });
    let mut output = write_indent(output, indent);
    output.push_str("],\n");
    let mut output = write_indent(output, indent);
    output.push_str("else=[\n");
    let output = func.ifs.else_branches[index]
        .iter()
        .fold(output, |output, &parameter| {
            let output = write_indent(output, next_indent);
            ast_string_expression(output, func, parameter, next_indent)
        });
    let mut output = write_indent(output, indent);
    output.push_str("]\n");
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
        Kind::Symbol => ast_string_symbol(output, func, expression),
        Kind::BinaryOp => ast_string_binary_op(output, func, expression, indent + INDENT),
        Kind::Definition => ast_string_definition(output, func, expression, indent + INDENT),
        Kind::FunctionCall => ast_string_function_call(output, func, expression, indent + INDENT),
        Kind::If => ast_string_if(output, func, expression, indent + INDENT),
    }
}

fn ast_string_function(mut output: String, func: &Function) -> String {
    output.push_str("    Function(\n");
    output.push_str("        name=");
    output.push_str(&func.symbols[func.name]);
    output.push_str(",\n        arguments=[\n");
    let mut output = func.arguments.iter().fold(output, |mut output, &argument| {
        output.push_str("            ");
        output.push_str(&func.symbols[argument]);
        output.push_str(",\n");
        output
    });
    output.push_str("        ],\n        body=[\n");
    let mut output = func.expressions.iter().fold(output, |output, &expression| {
        let indent = 12;
        let output = write_indent(output, indent);
        ast_string_expression(output, func, expression, indent)
    });
    output.push_str("        ]\n    ),\n");
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
    ),
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
                op=Add,
                left=Int(5),
                right=Int(10),
            ),
        ]
    ),
])
"#
    );
}

#[test]
fn test_parse_subtract() {
    let tokens = tokenize("def start(): 5 - 10");
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
                op=Subtract,
                left=Int(5),
                right=Int(10),
            ),
        ]
    ),
])
"#
    );
}

#[test]
fn test_parse_multiply() {
    let tokens = tokenize("def start(): 5 * 10");
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
                op=Multiply,
                left=Int(5),
                right=Int(10),
            ),
        ]
    ),
])
"#
    );
}

#[test]
fn test_parse_divide() {
    let tokens = tokenize("def start(): 10 / 5");
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
                op=Divide,
                left=Int(10),
                right=Int(5),
            ),
        ]
    ),
])
"#
    );
}

#[test]
fn test_parse_modulo() {
    let tokens = tokenize("def start(): 10 % 5");
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
                op=Modulo,
                left=Int(10),
                right=Int(5),
            ),
        ]
    ),
])
"#
    );
}

#[test]
fn test_parse_compare() {
    let tokens = tokenize("def start(): 10 == 5");
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
                op=Equal,
                left=Int(10),
                right=Int(5),
            ),
        ]
    ),
])
"#
    );
}

#[test]
fn test_parse_less_than() {
    let tokens = tokenize("def start(): 10 < 5");
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
                op=LessThan,
                left=Int(10),
                right=Int(5),
            ),
        ]
    ),
])
"#
    );
}

#[test]
fn test_parse_add_then_multiply() {
    let tokens = tokenize("def start(): 3 + 5 * 10");
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
                op=Add,
                left=Int(3),
                right=BinaryOp(
                    op=Multiply,
                    left=Int(5),
                    right=Int(10),
                ),
            ),
        ]
    ),
])
"#
    );
}

#[test]
fn test_parse_multiply_then_add() {
    let tokens = tokenize("def start(): 3 * 5 + 10");
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
                op=Add,
                left=BinaryOp(
                    op=Multiply,
                    left=Int(3),
                    right=Int(5),
                ),
                right=Int(10),
            ),
        ]
    ),
])
"#
    );
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
        ast_string(&ast),
        r#"
Ast([
    Function(
        name=start,
        arguments=[
        ],
        body=[
            Definition(
                name=x,
                value=Int(5),
            ),
            Definition(
                name=y,
                value=Int(20),
            ),
            BinaryOp(
                op=Add,
                left=Symbol(x),
                right=Symbol(y),
            ),
        ]
    ),
])
"#
    );
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
        ast_string(&ast),
        r#"
Ast([
    Function(
        name=square,
        arguments=[
            x,
        ],
        body=[
            BinaryOp(
                op=Multiply,
                left=Symbol(x),
                right=Symbol(x),
            ),
        ]
    ),
    Function(
        name=sum_of_squares,
        arguments=[
            x,
            y,
        ],
        body=[
            Definition(
                name=x2,
                value=FunctionCall(
                    name=square,
                    parameters=[
                        Symbol(x),
                    ]
                ),
            ),
            Definition(
                name=y2,
                value=FunctionCall(
                    name=square,
                    parameters=[
                        Symbol(y),
                    ]
                ),
            ),
            BinaryOp(
                op=Add,
                left=Symbol(x2),
                right=Symbol(y2),
            ),
        ]
    ),
    Function(
        name=start,
        arguments=[
        ],
        body=[
            FunctionCall(
                name=sum_of_squares,
                parameters=[
                    Int(5),
                    Int(3),
                ]
            ),
        ]
    ),
])
"#
    );
}

#[test]
fn test_parse_single_line_if() {
    let source = r#"
def min(x, y):
  if x < y: x else: y"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        ast_string(&ast),
        r#"
Ast([
    Function(
        name=min,
        arguments=[
            x,
            y,
        ],
        body=[
            If(
                condition=BinaryOp(
                    op=LessThan,
                    left=Symbol(x),
                    right=Symbol(y),
                ),
                then=[
                    Symbol(x),
                ],
                else=[
                    Symbol(y),
                ]
            ),
        ]
    ),
])
"#
    );
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
        ast_string(&ast),
        r#"
Ast([
    Function(
        name=main,
        arguments=[
        ],
        body=[
            Definition(
                name=a,
                value=Int(5),
            ),
            Definition(
                name=b,
                value=Int(10),
            ),
            If(
                condition=BinaryOp(
                    op=LessThan,
                    left=Symbol(a),
                    right=Symbol(b),
                ),
                then=[
                    Definition(
                        name=c,
                        value=Int(7),
                    ),
                    BinaryOp(
                        op=Add,
                        left=Symbol(a),
                        right=BinaryOp(
                            op=Add,
                            left=Symbol(b),
                            right=Symbol(c),
                        ),
                    ),
                ],
                else=[
                    Definition(
                        name=d,
                        value=Int(8),
                    ),
                    BinaryOp(
                        op=Multiply,
                        left=Symbol(a),
                        right=BinaryOp(
                            op=Multiply,
                            left=Symbol(b),
                            right=Symbol(c),
                        ),
                    ),
                ]
            ),
        ]
    ),
])
"#
    );
}

#[test]
fn test_parse_multiple_single_line_ifs() {
    let source = r#"
def main():
  a = 5
  b = 10
  c = if a < b: 15 else: 20
  d = if b < a: 5 else: 10
  c + d"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        ast_string(&ast),
        r#"
Ast([
    Function(
        name=main,
        arguments=[
        ],
        body=[
            Definition(
                name=a,
                value=Int(5),
            ),
            Definition(
                name=b,
                value=Int(10),
            ),
            Definition(
                name=c,
                value=If(
                    condition=BinaryOp(
                        op=LessThan,
                        left=Symbol(a),
                        right=Symbol(b),
                    ),
                    then=[
                        Int(15),
                    ],
                    else=[
                        Int(20),
                    ]
                ),
            ),
            Definition(
                name=d,
                value=If(
                    condition=BinaryOp(
                        op=LessThan,
                        left=Symbol(b),
                        right=Symbol(a),
                    ),
                    then=[
                        Int(5),
                    ],
                    else=[
                        Int(10),
                    ]
                ),
            ),
            BinaryOp(
                op=Add,
                left=Symbol(c),
                right=Symbol(d),
            ),
        ]
    ),
])
"#
    );
}

#[test]
fn test_parse_multi_line_if_returns_value() {
    let source = r#"
def main():
  a = 5
  b = 10
  c = if a < b:
    15
  else:
    20
  d = if b < a: 5 else: 10
  c + d"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        ast_string(&ast),
        r#"
Ast([
    Function(
        name=main,
        arguments=[
        ],
        body=[
            Definition(
                name=a,
                value=Int(5),
            ),
            Definition(
                name=b,
                value=Int(10),
            ),
            Definition(
                name=c,
                value=If(
                    condition=BinaryOp(
                        op=LessThan,
                        left=Symbol(a),
                        right=Symbol(b),
                    ),
                    then=[
                        Int(15),
                    ],
                    else=[
                        Int(20),
                    ]
                ),
            ),
            Definition(
                name=d,
                value=If(
                    condition=BinaryOp(
                        op=LessThan,
                        left=Symbol(b),
                        right=Symbol(a),
                    ),
                    then=[
                        Int(5),
                    ],
                    else=[
                        Int(10),
                    ]
                ),
            ),
            BinaryOp(
                op=Add,
                left=Symbol(c),
                right=Symbol(d),
            ),
        ]
    ),
])
"#
    );
}
