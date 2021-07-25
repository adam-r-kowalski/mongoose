use std::collections::HashMap;

use crate::tokenizer::{self, Tokens};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Entity(pub usize);

struct Token(usize);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Kind {
    Symbol,
    Int,
    Function,
}

#[derive(Debug, PartialEq)]
pub struct Functions {
    pub names: Vec<Entity>,
    pub return_types: Vec<Entity>,
    pub bodies: Vec<Entity>,
}

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub kinds: Vec<Kind>,
    pub indices: Vec<usize>,
    pub functions: Functions,
    pub symbols: Vec<String>,
    pub ints: Vec<String>,
    pub top_level: HashMap<String, Entity>,
}

fn inc_token(Token(index): Token) -> Token {
    Token(index + 1)
}

fn fresh_entity(ast: &Ast) -> Entity {
    Entity(ast.kinds.len())
}

type PrefixParser = impl Fn(Ast, &Tokens, Token) -> (Ast, Token, Entity);

fn parse_primitive(kind: Kind) -> PrefixParser {
    return move |mut ast: Ast, tokens: &Tokens, token: Token| {
        let entity = fresh_entity(&ast);
        ast.kinds.push(kind);
        ast.indices.push(tokens.indices[token.0]);
        (ast, inc_token(token), entity)
    }
}

fn prefix_parser(kind: tokenizer::Kind) -> PrefixParser {
    match kind {
        tokenizer::Kind::Symbol => parse_primitive(Kind::Symbol),
        tokenizer::Kind::Int => parse_primitive(Kind::Int),
        token => panic!("no prefix parser for {:?}", token),
    }
}

fn consume(tokens: &Tokens, token: Token, kind: tokenizer::Kind) -> Token {
    assert_eq!(tokens.kinds[token.0], kind);
    inc_token(token)
}

fn parse_function(ast: Ast, tokens: &Tokens, token: Token, name: Entity) -> (Ast, Token, Entity) {
    assert_eq!(ast.kinds[name.0], Kind::Symbol);
    let token = consume(tokens, token, tokenizer::Kind::RightParen);
    let token = consume(tokens, token, tokenizer::Kind::Arrow);
    let (ast, token, return_type) = parse_expression(ast, tokens, token);
    assert_eq!(ast.kinds[return_type.0], Kind::Symbol);
    let token = consume(tokens, token, tokenizer::Kind::Equal);
    let (mut ast, token, body) = parse_expression(ast, tokens, token);
    let entity = fresh_entity(&ast);
    ast.kinds.push(Kind::Function);
    ast.indices.push(ast.functions.names.len());
    ast.functions.names.push(name);
    ast.functions.return_types.push(return_type);
    ast.functions.bodies.push(body);
    (ast, inc_token(token), entity)
}

type InfixParser = impl Fn(Ast, &Tokens, Token, Entity) -> (Ast, Token, Entity);

fn infix_parser(kind: tokenizer::Kind) -> Option<InfixParser> {
    match kind {
        tokenizer::Kind::LeftParen => Some(parse_function),
        _ => None,
    }
}

fn parse_expression(ast: Ast, tokens: &Tokens, token: Token) -> (Ast, Token, Entity) {
    let parse_prefix = prefix_parser(tokens.kinds[token.0]);
    let (ast, token, left) = parse_prefix(ast, tokens, token);
    let parse_infix = tokens
        .kinds
        .get(token.0)
        .map(|&kind| infix_parser(kind))
        .flatten();
    match parse_infix {
        Some(parse_infix) => parse_infix(ast, tokens, inc_token(token), left),
        _ => (ast, token, left),
    }
}

pub fn parse(tokens: Tokens) -> Ast {
    let ast = Ast {
        kinds: vec![],
        indices: vec![],
        functions: Functions {
            names: vec![],
            return_types: vec![],
            bodies: vec![],
        },
        symbols: vec![],
        ints: vec![],
        top_level: HashMap::new()
    };
    let (mut ast, _, function) = parse_expression(ast, &tokens, Token(0));
    assert_eq!(ast.kinds[function.0], Kind::Function);
    let name = &ast.functions.names[ast.indices[function.0]];
    assert_eq!(ast.kinds[name.0], Kind::Symbol);
    ast.top_level.try_insert(tokens.symbols[ast.indices[name.0]].clone(), function).unwrap();
    ast.symbols = tokens.symbols;
    ast.ints = tokens.ints;
    ast
}
