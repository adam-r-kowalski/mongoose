use std::convert::identity;

use crate::types::{Ast, AstEntity, AstKind};

fn trim_whitespace(source: &str) -> &str {
    let length = source.chars().take_while(|c| c.is_whitespace()).count();
    &source[length..]
}

enum Sign {
    Negative,
    Positive,
}

struct Decimals(u64);

fn number<'a>(
    sign: Sign,
    Decimals(decimals): Decimals,
    source: &'a str,
    mut ast: Ast<'a>,
) -> (&'a str, Ast<'a>, AstEntity) {
    let skip = match sign {
        Sign::Negative => 1,
        Sign::Positive if decimals == 1 => 1,
        Sign::Positive => 0,
    };
    let (length, decimals) = source
        .chars()
        .skip(skip)
        .try_fold((skip, decimals), |(length, decimals), val| match val {
            c if c.is_numeric() => Ok((length + 1, decimals)),
            '.' => Ok((length + 1, decimals + 1)),
            _ => Err((length, decimals)),
        })
        .map_or_else(identity, identity);
    let entity = AstEntity(ast.indices.len());
    let kind = match length {
        1 if skip == 1 => AstKind::Symbol,
        _ if decimals > 0 => AstKind::Float,
        _ => AstKind::Int,
    };
    ast.kinds.push(kind);
    ast.indices.push(ast.strings.len());
    ast.strings.push(&source[..length]);
    (&source[length..], ast, entity)
}

fn list<'a>(
    kind: AstKind,
    delimiter: char,
    source: &'a str,
    mut ast: Ast<'a>,
    mut children: Vec<AstEntity>,
) -> (&'a str, Ast<'a>, AstEntity) {
    let source = trim_whitespace(source);
    match source.chars().next() {
        Some(c) if c == delimiter => {
            let entity = AstEntity(ast.indices.len());
            ast.indices.push(ast.children.len());
            ast.kinds.push(kind);
            ast.children.push(children);
            (&source[1..], ast, entity)
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
        '[' | ']' | '(' | ')' => true,
        _ if c.is_whitespace() => true,
        _ => false,
    }
}

fn identifier<'a>(
    kind: AstKind,
    source: &'a str,
    mut ast: Ast<'a>,
) -> (&'a str, Ast<'a>, AstEntity) {
    let length = source.chars().take_while(|&c| !is_reserved(c)).count();
    let entity = AstEntity(ast.indices.len());
    ast.kinds.push(kind);
    ast.indices.push(ast.strings.len());
    ast.strings.push(&source[..length]);
    (&source[length..], ast, entity)
}

fn expression<'a>(source: &'a str, ast: Ast<'a>) -> (&'a str, Ast<'a>, AstEntity) {
    match source.chars().next() {
        Some(c) if c.is_numeric() => number(Sign::Positive, Decimals(0), source, ast),
        Some('-') => number(Sign::Negative, Decimals(0), source, ast),
        Some('.') => number(Sign::Positive, Decimals(1), source, ast),
        Some('[') => list(AstKind::Brackets, ']', &source[1..], ast, vec![]),
        Some('(') => list(AstKind::Parens, ')', &source[1..], ast, vec![]),
        Some(':') => identifier(AstKind::Keyword, source, ast),
        _ => identifier(AstKind::Symbol, source, ast),
    }
}

fn parse_impl<'a>(source: &'a str, ast: Ast<'a>) -> Ast<'a> {
    let source = trim_whitespace(source);
    match source.is_empty() {
        true => ast,
        false => {
            let (source, mut ast, entity) = expression(source, ast);
            ast.top_level.push(entity);
            parse_impl(source, ast)
        }
    }
}

pub fn parse(source: &str) -> Ast {
    parse_impl(
        source,
        Ast {
            indices: vec![],
            kinds: vec![],
            strings: vec![],
            children: vec![],
            top_level: vec![],
        },
    )
}
