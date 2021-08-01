#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Kind {
    Def,
    Symbol,
    LeftParen,
    RightParen,
    Colon,
    Plus,
    Minus,
    Times,
    Slash,
    Equal,
    Indent,
    Int,
}

#[derive(Debug, PartialEq)]
pub struct Tokens {
    pub indices: Vec<usize>,
    pub kinds: Vec<Kind>,
    pub symbols: Vec<String>,
    pub ints: Vec<String>,
    pub indents: Vec<usize>,
}

fn tokenize_symbol(mut tokens: Tokens, source: &str) -> Tokens {
    let length = 1 + source[1..]
        .chars()
        .take_while(|c| c.is_alphanumeric())
        .count();
    if &source[..length] == "def" {
        tokens.kinds.push(Kind::Def);
        tokens.indices.push(0);
    } else {
        tokens.kinds.push(Kind::Symbol);
        tokens.indices.push(tokens.symbols.len());
        tokens.symbols.push(source[..length].to_string());
    }
    tokenize_impl(tokens, &source[length..])
}

fn tokenize_one(mut tokens: Tokens, source: &str, kind: Kind) -> Tokens {
    tokens.kinds.push(kind);
    tokens.indices.push(0);
    tokenize_impl(tokens, &source[1..])
}

fn tokenize_number(mut tokens: Tokens, source: &str) -> Tokens {
    let length = 1 + source[1..].chars().take_while(|c| c.is_numeric()).count();
    tokens.kinds.push(Kind::Int);
    tokens.indices.push(tokens.ints.len());
    tokens.ints.push(source[..length].to_string());
    tokenize_impl(tokens, &source[length..])
}

fn tokenize_indent(mut tokens: Tokens, source: &str) -> Tokens {
    let length = source[1..]
        .chars()
        .take_while(|&c| is_whitespace(c))
        .count();
    if length > 1 {
        tokens.kinds.push(Kind::Indent);
        tokens.indices.push(tokens.indents.len());
        tokens.indents.push(length);
    }
    tokenize_impl(tokens, &source[length + 1..])
}

fn is_whitespace(c: char) -> bool {
    match c {
        '\t' | '\x0C' | '\r' | ' ' => true,
        _ => false,
    }
}

fn trim_whitespace(source: &str) -> &str {
    let length = source.chars().take_while(|&c| is_whitespace(c)).count();
    &source[length..]
}

fn tokenize_impl(tokens: Tokens, source: &str) -> Tokens {
    let source = trim_whitespace(source);
    match source.chars().next() {
        Some(c) if c.is_alphabetic() => tokenize_symbol(tokens, source),
        Some('(') => tokenize_one(tokens, source, Kind::LeftParen),
        Some(')') => tokenize_one(tokens, source, Kind::RightParen),
        Some(':') => tokenize_one(tokens, source, Kind::Colon),
        Some('+') => tokenize_one(tokens, source, Kind::Plus),
        Some('-') => tokenize_one(tokens, source, Kind::Minus),
        Some('*') => tokenize_one(tokens, source, Kind::Times),
        Some('/') => tokenize_one(tokens, source, Kind::Slash),
        Some('=') => tokenize_one(tokens, source, Kind::Equal),
        Some('0'..='9') => tokenize_number(tokens, source),
        Some('\n') => tokenize_indent(tokens, source),
        Some(c) => panic!("not implemented for char {}", c),
        None => tokens,
    }
}

pub fn tokenize(source: &str) -> Tokens {
    let tokens = Tokens {
        indices: vec![],
        kinds: vec![],
        symbols: vec![],
        ints: vec![],
        indents: vec![],
    };
    tokenize_impl(tokens, source)
}
