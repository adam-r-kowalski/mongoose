use std::collections::HashMap;

use crate::tokenizer::{self, Tokens};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Entity(pub usize);

#[derive(Copy, Clone)]
struct Token(usize);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Kind {
    Symbol,
    Int,
    Function,
    BinaryOp,
}

#[derive(Debug, PartialEq)]
pub struct Functions {
    pub names: Vec<Entity>,
    pub return_types: Vec<Entity>,
    pub bodies: Vec<Entity>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BinaryOp {
    Add,
    Multiply,
}

#[derive(Debug, PartialEq)]
pub struct BinaryOps {
    pub ops: Vec<BinaryOp>,
    pub lefts: Vec<Entity>,
    pub rights: Vec<Entity>,
}

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub kinds: Vec<Kind>,
    pub indices: Vec<usize>,
    pub functions: Functions,
    pub binary_ops: BinaryOps,
    pub symbols: Vec<String>,
    pub ints: Vec<String>,
    pub top_level: HashMap<String, Entity>,
}

struct ParseResult(Ast, Token, Entity);

fn inc_token(Token(index): Token) -> Token {
    Token(index + 1)
}

fn fresh_entity(ast: &Ast) -> Entity {
    Entity(ast.kinds.len())
}

fn parse_primitive(mut ast: Ast, tokens: &Tokens, token: Token, kind: Kind) -> ParseResult {
    let entity = fresh_entity(&ast);
    ast.kinds.push(kind);
    ast.indices.push(tokens.indices[token.0]);
    ParseResult(ast, inc_token(token), entity)
}

fn prefix_parser(ast: Ast, tokens: &Tokens, token: Token, kind: tokenizer::Kind) -> ParseResult {
    match kind {
        tokenizer::Kind::Symbol => parse_primitive(ast, tokens, token, Kind::Symbol),
        tokenizer::Kind::Int => parse_primitive(ast, tokens, token, Kind::Int),
        token => panic!("no prefix parser for {:?}", token),
    }
}

fn consume(tokens: &Tokens, token: Token, kind: tokenizer::Kind) -> Token {
    assert_eq!(tokens.kinds[token.0], kind);
    inc_token(token)
}

fn parse_function(ast: Ast, tokens: &Tokens, token: Token, name: Entity) -> ParseResult {
    let token = inc_token(token);
    assert_eq!(ast.kinds[name.0], Kind::Symbol);
    let token = consume(tokens, token, tokenizer::Kind::RightParen);
    let token = consume(tokens, token, tokenizer::Kind::Arrow);
    let ParseResult(ast, token, return_type) = parse_expression(ast, tokens, token);
    assert_eq!(ast.kinds[return_type.0], Kind::Symbol);
    let token = consume(tokens, token, tokenizer::Kind::Colon);
    let ParseResult(mut ast, token, body) = parse_expression(ast, tokens, token);
    let entity = fresh_entity(&ast);
    ast.kinds.push(Kind::Function);
    ast.indices.push(ast.functions.names.len());
    ast.functions.names.push(name);
    ast.functions.return_types.push(return_type);
    ast.functions.bodies.push(body);
    ParseResult(ast, inc_token(token), entity)
}

fn parse_binary_op(
    binary_op: BinaryOp,
    ast: Ast,
    tokens: &Tokens,
    token: Token,
    left: Entity,
) -> ParseResult {
    let token = inc_token(token);
    // assert_eq!(ast.kinds[left.0], Kind::Int);
    let ParseResult(mut ast, token, right) = parse_expression(ast, tokens, token);
    // assert_eq!(ast.kinds[right.0], Kind::Int);
    let entity = fresh_entity(&ast);
    ast.kinds.push(Kind::BinaryOp);
    ast.indices.push(ast.binary_ops.lefts.len());
    ast.binary_ops.ops.push(binary_op);
    ast.binary_ops.lefts.push(left);
    ast.binary_ops.rights.push(right);
    ParseResult(ast, inc_token(token), entity)
}

fn infix_parser(
    ast: Ast,
    tokens: &Tokens,
    token: Token,
    kind: tokenizer::Kind,
    left: Entity,
) -> ParseResult {
    match kind {
        tokenizer::Kind::LeftParen => parse_function(ast, tokens, token, left),
        tokenizer::Kind::Plus => parse_binary_op(BinaryOp::Add, ast, tokens, token, left),
        tokenizer::Kind::Times => parse_binary_op(BinaryOp::Multiply, ast, tokens, token, left),
        _ => ParseResult(ast, token, left),
    }
}

fn parse_right(ast: Ast, tokens: &Tokens, token: Token, left: Entity) -> ParseResult {
    match tokens.kinds.get(token.0) {
        Some(&kind) => infix_parser(ast, tokens, token, kind, left),
        None => ParseResult(ast, token, left),
    }
}

fn parse_expression(ast: Ast, tokens: &Tokens, token: Token) -> ParseResult {
    let kind = tokens.kinds[token.0];
    let ParseResult(ast, token, left) = prefix_parser(ast, tokens, token, kind);
    parse_right(ast, tokens, token, left)
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
        binary_ops: BinaryOps {
            ops: vec![],
            lefts: vec![],
            rights: vec![],
        },
        symbols: vec![],
        ints: vec![],
        top_level: HashMap::new(),
    };
    let ParseResult(mut ast, _, function) = parse_expression(ast, &tokens, Token(0));
    assert_eq!(ast.kinds[function.0], Kind::Function);
    let name = &ast.functions.names[ast.indices[function.0]];
    assert_eq!(ast.kinds[name.0], Kind::Symbol);
    ast.top_level
        .try_insert(tokens.symbols[ast.indices[name.0]].clone(), function)
        .unwrap();
    ast.symbols = tokens.symbols;
    ast.ints = tokens.ints;
    ast
}
