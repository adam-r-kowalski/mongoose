#[derive(Debug, PartialEq)]
pub enum Kind {
    Symbol,
    LeftParen,
    RightParen,
    Arrow,
    Equal,
    Int,
}

#[derive(Debug, PartialEq)]
pub struct Tokens {
    pub indices: Vec<usize>,
    pub kinds: Vec<Kind>,
    pub strings: Vec<String>,
}

fn tokenize_symbol(mut tokens: Tokens, source: &str) -> Tokens {
    let length = 1 + source[1..]
        .chars()
        .take_while(|c| c.is_alphanumeric())
        .count();
    tokens.kinds.push(Kind::Symbol);
    tokens.indices.push(tokens.strings.len());
    tokens.strings.push(source[..length].to_string());
    tokenize_impl(tokens, &source[length..])
}

fn tokenize_paren(mut tokens: Tokens, source: &str, kind: Kind) -> Tokens {
    tokens.kinds.push(kind);
    tokens.indices.push(0);
    tokenize_impl(tokens, &source[1..])
}

fn tokenize_arrow(mut tokens: Tokens, source: &str) -> Tokens {
    match source[1..].chars().next() {
        Some('>') => {
            tokens.kinds.push(Kind::Arrow);
            tokens.indices.push(0);
            tokenize_impl(tokens, &source[2..])
        }
        c => panic!("tokenize arrow expected '>' found {:?}", c),
    }
}

fn tokenize_equal(mut tokens: Tokens, source: &str) -> Tokens {
    tokens.kinds.push(Kind::Equal);
    tokens.indices.push(0);
    tokenize_impl(tokens, &source[1..])
}

fn tokenize_number(mut tokens: Tokens, source: &str) -> Tokens {
    let length = 1 + source[1..].chars().take_while(|c| c.is_numeric()).count();
    tokens.kinds.push(Kind::Int);
    tokens.indices.push(tokens.strings.len());
    tokens.strings.push(source[..length].to_string());
    tokenize_impl(tokens, &source[1..])
}

fn trim_whitespace(source: &str) -> &str {
    let length = source.chars().take_while(|c| c.is_whitespace()).count();
    &source[length..]
}

fn tokenize_impl(tokens: Tokens, source: &str) -> Tokens {
    let source = trim_whitespace(source);
    match source.chars().next() {
        Some(c) if c.is_alphabetic() => tokenize_symbol(tokens, source),
        Some('(') => tokenize_paren(tokens, source, Kind::LeftParen),
        Some(')') => tokenize_paren(tokens, source, Kind::RightParen),
        Some('-') => tokenize_arrow(tokens, source),
        Some('=') => tokenize_equal(tokens, source),
        Some('0'..='9') => tokenize_number(tokens, source),
        Some(c) => panic!("not implemented for char {}", c),
        None => tokens,
    }
}

pub fn tokenize(source: &str) -> Tokens {
    let tokens = Tokens {
        indices: vec![],
        kinds: vec![],
        strings: vec![],
    };
    tokenize_impl(tokens, source)
}
