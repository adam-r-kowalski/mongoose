use pretty_assertions::assert_eq;

use compiler::{parser::parse, tokenizer::tokenize};

fn test_parse_binary_op(function_body: &str, expected_parsing: &str) {
    let remove_whitespace =
        |text: &str| -> String { text.chars().filter(|c| !c.is_whitespace()).collect() };
    let function_string = format!("fn start() -> i64: {}", function_body);
    let tokens = tokenize(&function_string);
    let ast = parse(tokens);
    let expected_function_parsing = format!(
        r#"
Ast([
    Function(
        name=start,
        arguments=[],
        argument_types=[],
        return_type=Symbol(i64),
        body=[{}]
    ),
])"#,
        expected_parsing
    );
    assert_eq!(
        remove_whitespace(&format!("{:?}", ast)),
        remove_whitespace(&expected_function_parsing)
    );
}

#[test]
fn test_parse_binary_ops() {
    test_parse_binary_op("5 + 10", "BinaryOp(op=Add,left=Int(5),right=Int(10),),");
    test_parse_binary_op(
        "5 - 10",
        "BinaryOp(op=Subtract,left=Int(5),right=Int(10),),",
    );
    test_parse_binary_op(
        "5 * 10",
        "BinaryOp(op=Multiply,left=Int(5),right=Int(10),),",
    );
    test_parse_binary_op("10 / 5", "BinaryOp(op=Divide,left=Int(10),right=Int(5),),");
    test_parse_binary_op("10 % 5", "BinaryOp(op=Modulo,left=Int(10),right=Int(5),),");
    test_parse_binary_op(
        "2 & 1",
        "BinaryOp(op=BitwiseAnd,left=Int(2),right=Int(1),),",
    );
    test_parse_binary_op("2 | 1", "BinaryOp(op=BitwiseOr,left=Int(2),right=Int(1),),");
    test_parse_binary_op(
        "2 ^ 1",
        "BinaryOp(op=BitwiseXor,left=Int(2),right=Int(1),),",
    );
    test_parse_binary_op(
        "2 << 1",
        "BinaryOp(op=ShiftLeft,left=Int(2),right=Int(1),),",
    );
    test_parse_binary_op(
        "2 >> 1",
        "BinaryOp(op=ShiftRight,left=Int(2),right=Int(1),),",
    );
    test_parse_binary_op("10 == 0", "BinaryOp(op=Equal,left=Int(10),right=Int(0),),");
    test_parse_binary_op("10 == 5", "BinaryOp(op=Equal,left=Int(10),right=Int(5),),");
    test_parse_binary_op(
        "10 != 5",
        "BinaryOp(op=NotEqual,left=Int(10),right=Int(5),),",
    );
    test_parse_binary_op(
        "10 < 5",
        "BinaryOp(op=LessThan,left=Int(10),right=Int(5),),",
    );
    test_parse_binary_op(
        "10 <= 5",
        "BinaryOp(op=LessThanEqual,left=Int(10),right=Int(5),),",
    );
    test_parse_binary_op(
        "10 > 5",
        "BinaryOp(op=GreaterThan,left=Int(10),right=Int(5),),",
    );
    test_parse_binary_op(
        "10 >= 5",
        "BinaryOp(op=GreaterThanEqual,left=Int(10),right=Int(5),),",
    );
    test_parse_binary_op("0", "Int(0),");
    test_parse_binary_op(
        "3 + 5 * 10",
        "BinaryOp(op=Add,left=Int(3),right=BinaryOp(op=Multiply,left=Int(5),right=Int(10),),),",
    );
    test_parse_binary_op(
        "3 * 5 + 10",
        "BinaryOp(op=Add,left=BinaryOp(op=Multiply,left=Int(3),right=Int(5),),right=Int(10),),",
    );
    test_parse_binary_op("3 * (5 + 10)","BinaryOp(op=Multiply,left=Int(3),right=Grouping(BinaryOp(op=Add,left=Int(5),right=Int(10),),),),",);
    test_parse_binary_op(
        r#"x = 5
           y = 20
           x + y"#,
        r#"Assign(name=x, value=Int(5),),
           Assign(name=y, value=Int(20),),
           BinaryOp(op=Add, left=Symbol(x), right=Symbol(y),),"#,
    )
}

#[test]
fn test_parse_multiple_functions() {
    let source = r#"
fn square(x: i64) -> i64: x * x

fn sum_of_squares(x: i64, y: i64) -> i64:
    x2 = square(x)
    y2 = square(y)
    x2 + y2

fn start() -> i64: sum_of_squares(5, 3)"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        format!("{:?}", ast),
        r#"
Ast([
    Function(
        name=square,
        arguments=[
            x,
        ],
        argument_types=[
            Symbol(i64),
        ],
        return_type=Symbol(i64),
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
        argument_types=[
            Symbol(i64),
            Symbol(i64),
        ],
        return_type=Symbol(i64),
        body=[
            Assign(
                name=x2,
                value=FunctionCall(
                    name=square,
                    parameters=[
                        Symbol(x),
                    ]
                ),
            ),
            Assign(
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
        argument_types=[
        ],
        return_type=Symbol(i64),
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
fn min(x: i64, y: i64) -> i64:
  if x < y: x else: y"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        format!("{:?}", ast),
        r#"
Ast([
    Function(
        name=min,
        arguments=[
            x,
            y,
        ],
        argument_types=[
            Symbol(i64),
            Symbol(i64),
        ],
        return_type=Symbol(i64),
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
fn main() -> i64:
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
        format!("{:?}", ast),
        r#"
Ast([
    Function(
        name=main,
        arguments=[
        ],
        argument_types=[
        ],
        return_type=Symbol(i64),
        body=[
            Assign(
                name=a,
                value=Int(5),
            ),
            Assign(
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
                    Assign(
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
                    Assign(
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
fn main() -> i64:
  a = 5
  b = 10
  c = if a < b: 15 else: 20
  d = if b < a: 5 else: 10
  c + d"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        format!("{:?}", ast),
        r#"
Ast([
    Function(
        name=main,
        arguments=[
        ],
        argument_types=[
        ],
        return_type=Symbol(i64),
        body=[
            Assign(
                name=a,
                value=Int(5),
            ),
            Assign(
                name=b,
                value=Int(10),
            ),
            Assign(
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
            Assign(
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
fn main() -> i64:
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
        format!("{:?}", ast),
        r#"
Ast([
    Function(
        name=main,
        arguments=[
        ],
        argument_types=[
        ],
        return_type=Symbol(i64),
        body=[
            Assign(
                name=a,
                value=Int(5),
            ),
            Assign(
                name=b,
                value=Int(10),
            ),
            Assign(
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
            Assign(
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
fn test_parse_while() {
    let source = r#"
fn start() -> i64:
    i = 0
    while i < 10:
        i = i + 1
    i"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        format!("{:?}", ast),
        r#"
Ast([
    Function(
        name=start,
        arguments=[
        ],
        argument_types=[
        ],
        return_type=Symbol(i64),
        body=[
            Assign(
                name=i,
                value=Int(0),
            ),
            While(
                condition=BinaryOp(
                    op=LessThan,
                    left=Symbol(i),
                    right=Int(10),
                ),
                body=[
                    Assign(
                        name=i,
                        value=BinaryOp(
                            op=Add,
                            left=Symbol(i),
                            right=Int(1),
                        ),
                    ),
                ]
            ),
            Symbol(i),
        ]
    ),
])
"#
    );
}

#[test]
fn test_parse_pipeline_simplest() {
    let source = r#"
fn square(x: i64) -> i64: x * x

fn start() -> i64: 5 |> square() |> square()
"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        format!("{:?}", ast),
        r#"
Ast([
    Function(
        name=square,
        arguments=[
            x,
        ],
        argument_types=[
            Symbol(i64),
        ],
        return_type=Symbol(i64),
        body=[
            BinaryOp(
                op=Multiply,
                left=Symbol(x),
                right=Symbol(x),
            ),
        ]
    ),
    Function(
        name=start,
        arguments=[
        ],
        argument_types=[
        ],
        return_type=Symbol(i64),
        body=[
            FunctionCall(
                name=square,
                parameters=[
                    FunctionCall(
                        name=square,
                        parameters=[
                            Int(5),
                        ]
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
fn test_parse_pipeline_no_paren_if_no_arguments() {
    let source = r#"
fn square(x: i64) -> i64: x * x

fn start() -> i64: 5 |> square |> square
"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        format!("{:?}", ast),
        r#"
Ast([
    Function(
        name=square,
        arguments=[
            x,
        ],
        argument_types=[
            Symbol(i64),
        ],
        return_type=Symbol(i64),
        body=[
            BinaryOp(
                op=Multiply,
                left=Symbol(x),
                right=Symbol(x),
            ),
        ]
    ),
    Function(
        name=start,
        arguments=[
        ],
        argument_types=[
        ],
        return_type=Symbol(i64),
        body=[
            FunctionCall(
                name=square,
                parameters=[
                    FunctionCall(
                        name=square,
                        parameters=[
                            Int(5),
                        ]
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
fn test_parse_pipeline_across_new_line() {
    let source = r#"
fn square(x: i64) -> i64: x * x

fn start() -> i64:
    5
    |> square
    |> square
"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        format!("{:?}", ast),
        r#"
Ast([
    Function(
        name=square,
        arguments=[
            x,
        ],
        argument_types=[
            Symbol(i64),
        ],
        return_type=Symbol(i64),
        body=[
            BinaryOp(
                op=Multiply,
                left=Symbol(x),
                right=Symbol(x),
            ),
        ]
    ),
    Function(
        name=start,
        arguments=[
        ],
        argument_types=[
        ],
        return_type=Symbol(i64),
        body=[
            FunctionCall(
                name=square,
                parameters=[
                    FunctionCall(
                        name=square,
                        parameters=[
                            Int(5),
                        ]
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
fn test_parse_pipeline_specify_location() {
    let source = r#"
fn f(x: i64, y: i64, z: i64) -> i64: x * y / z

fn start() -> i64: 10 |> f(5, _, 3)
"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        format!("{:?}", ast),
        r#"
Ast([
    Function(
        name=f,
        arguments=[
            x,
            y,
            z,
        ],
        argument_types=[
            Symbol(i64),
            Symbol(i64),
            Symbol(i64),
        ],
        return_type=Symbol(i64),
        body=[
            BinaryOp(
                op=Multiply,
                left=Symbol(x),
                right=BinaryOp(
                    op=Divide,
                    left=Symbol(y),
                    right=Symbol(z),
                ),
            ),
        ]
    ),
    Function(
        name=start,
        arguments=[
        ],
        argument_types=[
        ],
        return_type=Symbol(i64),
        body=[
            FunctionCall(
                name=f,
                parameters=[
                    Int(5),
                    Int(10),
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
fn test_parse_pipeline_with_grouped_expression() {
    let source = r#"
fn square(x: i64) -> i64: x * x

fn start() -> i64: (3 + 10) |> square
"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        format!("{:?}", ast),
        r#"
Ast([
    Function(
        name=square,
        arguments=[
            x,
        ],
        argument_types=[
            Symbol(i64),
        ],
        return_type=Symbol(i64),
        body=[
            BinaryOp(
                op=Multiply,
                left=Symbol(x),
                right=Symbol(x),
            ),
        ]
    ),
    Function(
        name=start,
        arguments=[
        ],
        argument_types=[
        ],
        return_type=Symbol(i64),
        body=[
            FunctionCall(
                name=square,
                parameters=[
                    Grouping(
                        BinaryOp(
                            op=Add,
                            left=Int(3),
                            right=Int(10),
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
fn test_parse_import() {
    let source = r#"
import foo.bar: baz

fn start() -> i64:
    baz(5, 3) + foo.bar.baz(5, 3)
"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    assert_eq!(
        format!("{:?}", ast),
        r#"
Ast([
    Import(
        path=[foo, bar, ],
        unqualified=[baz, ]
    ),
    Function(
        name=start,
        arguments=[
        ],
        argument_types=[
        ],
        return_type=Symbol(i64),
        body=[
            BinaryOp(
                op=Add,
                left=FunctionCall(
                    name=baz,
                    parameters=[
                        Int(5),
                        Int(3),
                    ]
                ),
                right=BinaryOp(
                    op=Dot,
                    left=Symbol(foo),
                    right=BinaryOp(
                        op=Dot,
                        left=Symbol(bar),
                        right=FunctionCall(
                            name=baz,
                            parameters=[
                                Int(5),
                                Int(3),
                            ]
                        ),
                    ),
                ),
            ),
        ]
    ),
])
"#
    );
}
