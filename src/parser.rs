#[derive(Debug, PartialEq)]
pub enum Kind {
    Int,
    Symbol,
    Keyword,
    Brackets,
    Parens,
}

#[derive(Debug, PartialEq)]
pub struct Ast<'a> {
    pub index: Vec<usize>,
    pub kind: Vec<Kind>,
    pub strings: Vec<&'a str>,
    pub children: Vec<Vec<usize>>,
    pub top_level: Vec<usize>,
}

fn trim_whitespace(source: &str) -> &str {
    let length = source.chars().take_while(|c| c.is_whitespace()).count();
    &source[length..]
}

enum Sign {
    Negative,
    Positive,
}

fn number<'a>(sign: Sign, source: &'a str, mut ast: Ast<'a>) -> (&'a str, Ast<'a>, usize) {
    let skip = match sign {
        Sign::Negative => 1,
        Sign::Positive => 0,
    };
    let length = source
        .chars()
        .skip(skip)
        .take_while(|&c| c.is_numeric())
        .count()
        + skip;
    let index = ast.index.len();
    ast.kind.push(Kind::Int);
    ast.index.push(ast.strings.len());
    ast.strings.push(&source[..length]);
    (&source[length..], ast, index)
}

fn list<'a>(
    kind: Kind,
    delimiter: char,
    source: &'a str,
    mut ast: Ast<'a>,
    mut children: Vec<usize>,
) -> (&'a str, Ast<'a>, usize) {
    let source = trim_whitespace(source);
    match source.chars().next() {
        Some(c) if c == delimiter => {
            let index = ast.index.len();
            ast.index.push(ast.children.len());
            ast.kind.push(kind);
            ast.children.push(children);
            (&source[1..], ast, index)
        }
        _ => {
            let (source, ast, index) = expression(source, ast);
            children.push(index);
            list(kind, delimiter, source, ast, children)
        }
    }
}

fn is_reserved(c: char) -> bool {
    match c {
        '[' | ']' => true,
        '(' | ')' => true,
        _ if c.is_whitespace() => true,
        _ => false,
    }
}

fn identifier<'a>(kind: Kind, source: &'a str, mut ast: Ast<'a>) -> (&'a str, Ast<'a>, usize) {
    let length = source.chars().take_while(|&c| !is_reserved(c)).count();
    let index = ast.index.len();
    ast.kind.push(kind);
    ast.index.push(ast.strings.len());
    ast.strings.push(&source[..length]);
    (&source[length..], ast, index)
}

fn expression<'a>(source: &'a str, ast: Ast<'a>) -> (&'a str, Ast<'a>, usize) {
    match source.chars().next() {
        Some(c) if c.is_numeric() => number(Sign::Positive, source, ast),
        Some('-') => number(Sign::Negative, source, ast),
        Some('[') => list(Kind::Brackets, ']', &source[1..], ast, vec![]),
        Some('(') => list(Kind::Parens, ')', &source[1..], ast, vec![]),
        Some(':') => identifier(Kind::Keyword, source, ast),
        _ => identifier(Kind::Symbol, source, ast),
    }
}

fn parse_impl<'a>(source: &'a str, ast: Ast<'a>) -> Ast<'a> {
    let source = trim_whitespace(source);
    match source.is_empty() {
        true => ast,
        false => {
            let (source, mut ast, id) = expression(source, ast);
            ast.top_level.push(id);
            parse_impl(source, ast)
        }
    }
}

pub fn parse(source: &str) -> Ast {
    parse_impl(
        source,
        Ast {
            index: vec![],
            kind: vec![],
            strings: vec![],
            children: vec![],
            top_level: vec![],
        },
    )
}
