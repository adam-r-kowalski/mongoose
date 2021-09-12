use pretty_assertions::assert_eq;

use compiler::tokenizer::tokenize;

#[test]
fn test_tokenize_int() {
    let tokens = tokenize("fn start() -> i64: 0");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(0),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_add() {
    let tokens = tokenize("fn start() -> i64: 5 + 10");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(5),
        Cross,
        Int(10),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_subtract() {
    let tokens = tokenize("fn start() -> i64: 5 - 10");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(5),
        Dash,
        Int(10),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_multiply() {
    let tokens = tokenize("fn start() -> i64: 5 * 10");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(5),
        Asterisk,
        Int(10),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_divide() {
    let tokens = tokenize("fn start() -> i64: 10 / 5");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(10),
        Slash,
        Int(5),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_modulo() {
    let tokens = tokenize("fn start() -> i64: 10 % 5");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(10),
        Percent,
        Int(5),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_bitwise_and() {
    let tokens = tokenize("fn start() -> i64: 2 & 1");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(2),
        Ampersand,
        Int(1),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_bitwise_or() {
    let tokens = tokenize("fn start() -> i64: 2 | 1");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(2),
        VerticalBar,
        Int(1),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_bitwise_xor() {
    let tokens = tokenize("fn start() -> i64: 2 ^ 1");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(2),
        Caret,
        Int(1),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_shift_left() {
    let tokens = tokenize("fn start() -> i64: 2 << 1");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(2),
        LessThanLessThan,
        Int(1),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_shift_right_signed() {
    let tokens = tokenize("fn start() -> i64: 8 >> 1");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(8),
        GreaterThanGreaterThan,
        Int(1),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_equal() {
    let tokens = tokenize("fn start() -> i64: 10 == 5");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(10),
        EqualEqual,
        Int(5),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_not_equal() {
    let tokens = tokenize("fn start() -> i64: 10 != 5");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(10),
        ExclamationEqual,
        Int(5),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_less_than_signed() {
    let tokens = tokenize("fn start() -> i64: 10 < 5");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(10),
        LessThan,
        Int(5),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_greater_than_signed() {
    let tokens = tokenize("fn start() -> i64: 10 > 5");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(10),
        GreaterThan,
        Int(5),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_less_than_or_equal_signed() {
    let tokens = tokenize("fn start() -> i64: 10 <= 5");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(10),
        LessThanEqual,
        Int(5),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_greater_than_or_equal_signed() {
    let tokens = tokenize("fn start() -> i64: 10 >= 5");
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(10),
        GreaterThanEqual,
        Int(5),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_local_variables() {
    let source = r#"
fn start() -> i64:
    x = 5
    y = 20
    x + y"#;
    let tokens = tokenize(source);
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Indent(4),
        Symbol(x),
        Equal,
        Int(5),
        Indent(4),
        Symbol(y),
        Equal,
        Int(20),
        Indent(4),
        Symbol(x),
        Cross,
        Symbol(y),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_multiple_functions() {
    let source = r#"
fn square(x: i64) -> i64: x * x

fn sum_of_squares(x: i64, y: i64) -> i64:
    x2 = square(x)
    y2 = square(y)
    x2 + y2

fn start() -> i64: sum_of_squares(5, 3)"#;
    let tokens = tokenize(source);
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(square),
        LeftParen,
        Symbol(x),
        Colon,
        Symbol(i64),
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Symbol(x),
        Asterisk,
        Symbol(x),
    ]),
    TopLevel([
        Fn,
        Symbol(sum_of_squares),
        LeftParen,
        Symbol(x),
        Colon,
        Symbol(i64),
        Comma,
        Symbol(y),
        Colon,
        Symbol(i64),
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Indent(4),
        Symbol(x2),
        Equal,
        Symbol(square),
        LeftParen,
        Symbol(x),
        RightParen,
        Indent(4),
        Symbol(y2),
        Equal,
        Symbol(square),
        LeftParen,
        Symbol(y),
        RightParen,
        Indent(4),
        Symbol(x2),
        Cross,
        Symbol(y2),
    ]),
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Symbol(sum_of_squares),
        LeftParen,
        Int(5),
        Comma,
        Int(3),
        RightParen,
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_if() {
    let source = r#"
fn min(x: i64, y: i64) -> i64:
  if x < y:
    x
  else:
    y"#;
    let tokens = tokenize(source);
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(min),
        LeftParen,
        Symbol(x),
        Colon,
        Symbol(i64),
        Comma,
        Symbol(y),
        Colon,
        Symbol(i64),
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Indent(2),
        If,
        Symbol(x),
        LessThan,
        Symbol(y),
        Colon,
        Indent(4),
        Symbol(x),
        Indent(2),
        Else,
        Colon,
        Indent(4),
        Symbol(y),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_while() {
    let source = r#"
fn start():
    i = 0
    while i < 10:
        i = i + 1
    i"#;
    let tokens = tokenize(source);
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        Colon,
        Indent(4),
        Symbol(i),
        Equal,
        Int(0),
        Indent(4),
        While,
        Symbol(i),
        LessThan,
        Int(10),
        Colon,
        Indent(8),
        Symbol(i),
        Equal,
        Symbol(i),
        Cross,
        Int(1),
        Indent(4),
        Symbol(i),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_comment() {
    let source = r#"
# comments can appear above top level expressions
fn start() -> i64:
    x = 10 # comments can appear to the right of expressions
    # comments can appear inbetween expressions
    x
# comments can appear below top level expressions
"#;
    let tokens = tokenize(source);
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Indent(4),
        Symbol(x),
        Equal,
        Int(10),
        Indent(4),
        Symbol(x),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_pipeline() {
    let source = r#"
fn square(x: i64) -> i64: x * x

fn start() -> i64: 5 |> square() |> square()
"#;
    let tokens = tokenize(source);
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Fn,
        Symbol(square),
        LeftParen,
        Symbol(x),
        Colon,
        Symbol(i64),
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Symbol(x),
        Asterisk,
        Symbol(x),
    ]),
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Int(5),
        VerticalBarGreaterThan,
        Symbol(square),
        LeftParen,
        RightParen,
        VerticalBarGreaterThan,
        Symbol(square),
        LeftParen,
        RightParen,
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_import() {
    let source = r#"
import builtin: i64_sub

fn start() -> i64:
    x = builtin.i64_add(7, 5)
    i64_sub(x, 3)
"#;
    let tokens = tokenize(source);
    assert_eq!(
        format!("{:?}", tokens),
        r#"
Tokens([
    TopLevel([
        Import,
        Symbol(builtin),
        Colon,
        Symbol(i64_sub),
    ]),
    TopLevel([
        Fn,
        Symbol(start),
        LeftParen,
        RightParen,
        DashGreaterThan,
        Symbol(i64),
        Colon,
        Indent(4),
        Symbol(x),
        Equal,
        Symbol(builtin),
        Dot,
        Symbol(i64_add),
        LeftParen,
        Int(7),
        Comma,
        Int(5),
        RightParen,
        Indent(4),
        Symbol(i64_sub),
        LeftParen,
        Symbol(x),
        Comma,
        Int(3),
        RightParen,
    ]),
])
"#
    );
}
