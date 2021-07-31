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
    pub bodies: Vec<Entity>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
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

type Precedence = u8;

#[derive(Debug)]
enum InfixParser {
    Function,
    BinaryOp(Precedence, BinaryOp),
}

struct ParseResult(Ast, Token, Entity);

const FUNCTION: Precedence = 100;
const ADD: Precedence = 10;
const SUBTRACT: Precedence = ADD;
const MULTIPLY: Precedence = 20;
const DIVIDE: Precedence = MULTIPLY;

fn precedence_of(parser: &InfixParser) -> Precedence {
    match parser {
        InfixParser::Function => FUNCTION,
        InfixParser::BinaryOp(precedence, _) => *precedence,
    }
}

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
    ParseResult(ast, token, entity)
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
    assert_eq!(ast.kinds[name.0], Kind::Symbol);
    let token = consume(tokens, token, tokenizer::Kind::RightParen);
    let token = consume(tokens, token, tokenizer::Kind::Colon);
    let ParseResult(mut ast, token, body) = parse_expression(ast, tokens, token, 0);
    let entity = fresh_entity(&ast);
    ast.kinds.push(Kind::Function);
    ast.indices.push(ast.functions.names.len());
    ast.functions.names.push(name);
    ast.functions.bodies.push(body);
    ParseResult(ast, token, entity)
}

fn parse_binary_op(
    precedence: Precedence,
    binary_op: BinaryOp,
    ast: Ast,
    tokens: &Tokens,
    token: Token,
    left: Entity,
) -> ParseResult {
    let ParseResult(mut ast, token, right) = parse_expression(ast, tokens, token, precedence);
    let entity = fresh_entity(&ast);
    ast.kinds.push(Kind::BinaryOp);
    ast.indices.push(ast.binary_ops.lefts.len());
    ast.binary_ops.ops.push(binary_op);
    ast.binary_ops.lefts.push(left);
    ast.binary_ops.rights.push(right);
    ParseResult(ast, token, entity)
}

fn infix_parser(kind: tokenizer::Kind) -> Option<InfixParser> {
    match kind {
        tokenizer::Kind::LeftParen => Some(InfixParser::Function),
        tokenizer::Kind::Plus => Some(InfixParser::BinaryOp(ADD, BinaryOp::Add)),
        tokenizer::Kind::Minus => Some(InfixParser::BinaryOp(SUBTRACT, BinaryOp::Subtract)),
        tokenizer::Kind::Times => Some(InfixParser::BinaryOp(MULTIPLY, BinaryOp::Multiply)),
        tokenizer::Kind::Slash => Some(InfixParser::BinaryOp(DIVIDE, BinaryOp::Divide)),
        _ => None,
    }
}

fn run_infix_parser(
    parser: InfixParser,
    ast: Ast,
    tokens: &Tokens,
    token: Token,
    left: Entity,
) -> ParseResult {
    match parser {
        InfixParser::Function => parse_function(ast, tokens, token, left),
        InfixParser::BinaryOp(precedence, binary_op) => {
            parse_binary_op(precedence, binary_op, ast, tokens, token, left)
        }
    }
}

fn parse_right(
    ast: Ast,
    tokens: &Tokens,
    token: Token,
    left: Entity,
    precedence: Precedence,
) -> ParseResult {
    let parser = tokens
        .kinds
        .get(token.0)
        .map(|&kind| infix_parser(kind))
        .flatten();
    match parser {
        Some(parser) if precedence < precedence_of(&parser) => {
            let ParseResult(ast, token, left) =
                run_infix_parser(parser, ast, tokens, inc_token(token), left);
            parse_right(ast, tokens, token, left, precedence)
        }
        _ => ParseResult(ast, token, left),
    }
}

fn parse_expression(
    ast: Ast,
    tokens: &Tokens,
    token: Token,
    precedence: Precedence,
) -> ParseResult {
    let kind = tokens.kinds[token.0];
    let ParseResult(ast, token, left) = prefix_parser(ast, tokens, token, kind);
    parse_right(ast, tokens, inc_token(token), left, precedence)
}

pub fn parse(tokens: Tokens) -> Ast {
    let ast = Ast {
        kinds: vec![],
        indices: vec![],
        functions: Functions {
            names: vec![],
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
    let ParseResult(mut ast, _, function) = parse_expression(ast, &tokens, Token(0), 0);
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
