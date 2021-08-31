use pretty_assertions::assert_eq;

use mongoose::tokenizer::{tokenize, Kind, Tokens, TopLevel};

fn token_string_literal(
    top_level: &TopLevel,
    token: usize,
    mut output: String,
    text: &str,
) -> String {
    output.push_str("        ");
    output.push_str(text);
    output.push_str(",\n");
    token_string_impl(top_level, token + 1, output)
}

fn token_string_symbol(top_level: &TopLevel, token: usize, mut output: String) -> String {
    let text = &top_level.symbols[top_level.indices[token]];
    output.push_str("        ");
    output.push_str("Symbol(");
    output.push_str(text);
    output.push_str("),\n");
    token_string_impl(top_level, token + 1, output)
}

fn token_string_int(top_level: &TopLevel, token: usize, mut output: String) -> String {
    let text = &top_level.ints[top_level.indices[token]];
    output.push_str("        ");
    output.push_str("Int(");
    output.push_str(text);
    output.push_str("),\n");
    token_string_impl(top_level, token + 1, output)
}

fn token_string_indent(top_level: &TopLevel, token: usize, mut output: String) -> String {
    let indent = &top_level.indents[top_level.indices[token]];
    output.push_str("        ");
    output.push_str("Indent(");
    output.push_str(&indent.to_string());
    output.push_str("),\n");
    token_string_impl(top_level, token + 1, output)
}

fn token_string_impl(top_level: &TopLevel, token: usize, output: String) -> String {
    match top_level.kinds.get(token) {
        Some(Kind::Def) => token_string_literal(top_level, token, output, "Def"),
        Some(Kind::LeftParen) => token_string_literal(top_level, token, output, "LeftParen"),
        Some(Kind::RightParen) => token_string_literal(top_level, token, output, "RightParen"),
        Some(Kind::Plus) => token_string_literal(top_level, token, output, "Plus"),
        Some(Kind::Minus) => token_string_literal(top_level, token, output, "Minus"),
        Some(Kind::Asterisk) => token_string_literal(top_level, token, output, "Asterisk"),
        Some(Kind::Slash) => token_string_literal(top_level, token, output, "Slash"),
        Some(Kind::Percent) => token_string_literal(top_level, token, output, "Percent"),
        Some(Kind::Colon) => token_string_literal(top_level, token, output, "Colon"),
        Some(Kind::Equal) => token_string_literal(top_level, token, output, "Equal"),
        Some(Kind::EqualEqual) => token_string_literal(top_level, token, output, "EqualEqual"),
        Some(Kind::ExclamationEqual) => {
            token_string_literal(top_level, token, output, "ExclamationEqual")
        }
        Some(Kind::Ampersand) => token_string_literal(top_level, token, output, "Ampersand"),
        Some(Kind::VerticalBar) => token_string_literal(top_level, token, output, "VerticalBar"),
        Some(Kind::VerticalBarGreaterThan) => {
            token_string_literal(top_level, token, output, "VerticalBarGreaterThan")
        }
        Some(Kind::Caret) => token_string_literal(top_level, token, output, "Caret"),
        Some(Kind::Dot) => token_string_literal(top_level, token, output, "Dot"),
        Some(Kind::LessThan) => token_string_literal(top_level, token, output, "LessThan"),
        Some(Kind::LessThanEqual) => {
            token_string_literal(top_level, token, output, "LessThanEqual")
        }
        Some(Kind::LessThanLessThan) => {
            token_string_literal(top_level, token, output, "LessThanLessThan")
        }
        Some(Kind::GreaterThan) => token_string_literal(top_level, token, output, "GreaterThan"),
        Some(Kind::GreaterThanEqual) => {
            token_string_literal(top_level, token, output, "GreaterThanEqual")
        }
        Some(Kind::GreaterThanGreaterThan) => {
            token_string_literal(top_level, token, output, "GreaterThanGreaterThan")
        }
        Some(Kind::Comma) => token_string_literal(top_level, token, output, "Comma"),
        Some(Kind::If) => token_string_literal(top_level, token, output, "If"),
        Some(Kind::Else) => token_string_literal(top_level, token, output, "Else"),
        Some(Kind::While) => token_string_literal(top_level, token, output, "While"),
        Some(Kind::Import) => token_string_literal(top_level, token, output, "Import"),
        Some(Kind::Symbol) => token_string_symbol(top_level, token, output),
        Some(Kind::Int) => token_string_int(top_level, token, output),
        Some(Kind::Indent) => token_string_indent(top_level, token, output),
        None => output,
    }
}

fn token_string(tokens: &Tokens) -> String {
    let mut output =
        tokens
            .top_level
            .iter()
            .fold(String::from("\nTokens([\n"), |mut output, top_level| {
                output.push_str("    TopLevel([\n");
                let mut output = token_string_impl(top_level, 0, output);
                output.push_str("    ]),\n");
                output
            });
    output.push_str("])\n");
    output
}

#[test]
fn test_tokenize_int() {
    let tokens = tokenize("def start(): 0");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
        Colon,
        Int(0),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_add() {
    let tokens = tokenize("def start(): 5 + 10");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
        Colon,
        Int(5),
        Plus,
        Int(10),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_subtract() {
    let tokens = tokenize("def start(): 5 - 10");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
        Colon,
        Int(5),
        Minus,
        Int(10),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_multiply() {
    let tokens = tokenize("def start(): 5 * 10");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
    let tokens = tokenize("def start(): 10 / 5");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
    let tokens = tokenize("def start(): 10 % 5");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
    let tokens = tokenize("def start(): 2 & 1");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
    let tokens = tokenize("def start(): 2 | 1");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
    let tokens = tokenize("def start(): 2 ^ 1");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
    let tokens = tokenize("def start(): 2 << 1");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
    let tokens = tokenize("def start(): 8 >> 1");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
    let tokens = tokenize("def start(): 10 == 5");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
    let tokens = tokenize("def start(): 10 != 5");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
    let tokens = tokenize("def start(): 10 < 5");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
    let tokens = tokenize("def start(): 10 > 5");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
    let tokens = tokenize("def start(): 10 <= 5");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
    let tokens = tokenize("def start(): 10 >= 5");
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
def start():
    x = 5
    y = 20
    x + y"#;
    let tokens = tokenize(source);
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
        Plus,
        Symbol(y),
    ]),
])
"#
    );
}

#[test]
fn test_tokenize_multiple_functions() {
    let source = r#"
def square(x): x * x

def sum_of_squares(x, y):
    x2 = square(x)
    y2 = square(y)
    x2 + y2

def start(): sum_of_squares(5, 3)"#;
    let tokens = tokenize(source);
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(square),
        LeftParen,
        Symbol(x),
        RightParen,
        Colon,
        Symbol(x),
        Asterisk,
        Symbol(x),
    ]),
    TopLevel([
        Def,
        Symbol(sum_of_squares),
        LeftParen,
        Symbol(x),
        Comma,
        Symbol(y),
        RightParen,
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
        Plus,
        Symbol(y2),
    ]),
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
def min(x, y):
  if x < y:
    x
  else:
    y"#;
    let tokens = tokenize(source);
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(min),
        LeftParen,
        Symbol(x),
        Comma,
        Symbol(y),
        RightParen,
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
def start():
    i = 0
    while i < 10:
        i = i + 1
    i"#;
    let tokens = tokenize(source);
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
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
        Plus,
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
def start():
    x = 10 # comments can appear to the right of expressions
    # comments can appear inbetween expressions
    x
# comments can appear below top level expressions
"#;
    let tokens = tokenize(source);
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
def square(x): x * x

def start(): 5 |> square() |> square()
"#;
    let tokens = tokenize(source);
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Def,
        Symbol(square),
        LeftParen,
        Symbol(x),
        RightParen,
        Colon,
        Symbol(x),
        Asterisk,
        Symbol(x),
    ]),
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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

def start():
    x = builtin.i64_add(7, 5)
    i64_sub(x, 3)
"#;
    let tokens = tokenize(source);
    assert_eq!(
        token_string(&tokens),
        r#"
Tokens([
    TopLevel([
        Import,
        Symbol(builtin),
        Colon,
        Symbol(i64_sub),
    ]),
    TopLevel([
        Def,
        Symbol(start),
        LeftParen,
        RightParen,
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
