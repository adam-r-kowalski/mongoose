#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Kind {
    Def,
    Symbol,
    LeftParen,
    RightParen,
    Colon,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    ColonEqual,
    Comma,
    LessThan,
    Indent,
    Int,
    If,
    Else,
}

#[derive(Debug, PartialEq)]
pub struct TopLevel {
    pub indices: Vec<usize>,
    pub kinds: Vec<Kind>,
    pub symbols: Vec<String>,
    pub ints: Vec<String>,
    pub indents: Vec<usize>,
}

#[derive(Debug, PartialEq)]
pub struct Tokens {
    pub top_level: Vec<TopLevel>,
}

fn insert_keyword(mut top_level: TopLevel, kind: Kind) -> TopLevel {
    top_level.kinds.push(kind);
    top_level.indices.push(0);
    top_level
}

fn insert_symbol(mut top_level: TopLevel, symbol: String) -> TopLevel {
    top_level.kinds.push(Kind::Symbol);
    top_level.indices.push(top_level.symbols.len());
    top_level.symbols.push(symbol);
    top_level
}

fn tokenize_symbol(top_level: TopLevel, source: &str) -> (TopLevel, &str) {
    let length = 1 + source[1..]
        .chars()
        .take_while(|&c| c.is_alphanumeric() || c == '_')
        .count();
    let top_level = match &source[..length] {
        "def" => insert_keyword(top_level, Kind::Def),
        "if" => insert_keyword(top_level, Kind::If),
        "else" => insert_keyword(top_level, Kind::Else),
        _ => insert_symbol(top_level, source[..length].to_string()),
    };
    tokenize_top_level(top_level, &source[length..])
}

fn tokenize_one(mut top_level: TopLevel, source: &str, kind: Kind) -> (TopLevel, &str) {
    top_level.kinds.push(kind);
    top_level.indices.push(0);
    tokenize_top_level(top_level, &source[1..])
}

fn tokenize_colon(mut top_level: TopLevel, source: &str) -> (TopLevel, &str) {
    let (length, kind) = match source.chars().skip(1).next() {
        Some('=') => (2, Kind::ColonEqual),
        _ => (1, Kind::Colon),
    };
    top_level.kinds.push(kind);
    top_level.indices.push(0);
    tokenize_top_level(top_level, &source[length..])
}

fn tokenize_equal(mut top_level: TopLevel, source: &str) -> (TopLevel, &str) {
    let (length, kind) = match source.chars().skip(1).next() {
        Some('=') => (2, Kind::EqualEqual),
        _ => (1, Kind::Equal),
    };
    top_level.kinds.push(kind);
    top_level.indices.push(0);
    tokenize_top_level(top_level, &source[length..])
}

fn tokenize_number(mut top_level: TopLevel, source: &str) -> (TopLevel, &str) {
    let length = 1 + source[1..].chars().take_while(|c| c.is_numeric()).count();
    top_level.kinds.push(Kind::Int);
    top_level.indices.push(top_level.ints.len());
    top_level.ints.push(source[..length].to_string());
    tokenize_top_level(top_level, &source[length..])
}

fn tokenize_indent(mut top_level: TopLevel, source: &str) -> (TopLevel, &str) {
    let length = source[1..]
        .chars()
        .take_while(|c| ['\t', '\x0C', ' '].contains(c))
        .count();
    if length > 0 {
        top_level.kinds.push(Kind::Indent);
        top_level.indices.push(top_level.indents.len());
        top_level.indents.push(length);
        tokenize_top_level(top_level, &source[length + 1..])
    } else {
        (top_level, source)
    }
}

fn trim(source: &str, predicate: fn(&char) -> bool) -> &str {
    let length = source.chars().take_while(|c| predicate(c)).count();
    &source[length..]
}

fn tokenize_top_level(top_level: TopLevel, source: &str) -> (TopLevel, &str) {
    let source = trim(source, |c| ['\t', '\x0C', '\r', ' '].contains(c));
    match source.chars().next() {
        Some(c) if c.is_alphabetic() || c == '_' => tokenize_symbol(top_level, source),
        Some('(') => tokenize_one(top_level, source, Kind::LeftParen),
        Some(')') => tokenize_one(top_level, source, Kind::RightParen),
        Some('+') => tokenize_one(top_level, source, Kind::Plus),
        Some('-') => tokenize_one(top_level, source, Kind::Minus),
        Some('*') => tokenize_one(top_level, source, Kind::Asterisk),
        Some('/') => tokenize_one(top_level, source, Kind::Slash),
        Some('%') => tokenize_one(top_level, source, Kind::Percent),
        Some(',') => tokenize_one(top_level, source, Kind::Comma),
        Some('<') => tokenize_one(top_level, source, Kind::LessThan),
        Some(':') => tokenize_colon(top_level, source),
        Some('=') => tokenize_equal(top_level, source),
        Some('0'..='9') => tokenize_number(top_level, source),
        Some('\n') => tokenize_indent(top_level, source),
        Some(c) => panic!("not implemented for char \"{}\"", c),
        None => (top_level, source),
    }
}

fn tokenize_impl(mut tokens: Tokens, source: &str) -> Tokens {
    let source = trim(source, |c| c.is_whitespace());
    if source.len() == 0 {
        tokens
    } else {
        let top_level = TopLevel {
            indices: vec![],
            kinds: vec![],
            symbols: vec![],
            ints: vec![],
            indents: vec![],
        };
        let (top_level, source) = tokenize_top_level(top_level, source);
        tokens.top_level.push(top_level);
        tokenize_impl(tokens, source)
    }
}

pub fn tokenize(source: &str) -> Tokens {
    let tokens = Tokens { top_level: vec![] };
    tokenize_impl(tokens, source)
}
