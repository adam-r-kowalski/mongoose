use std::{collections::HashMap, iter::FromIterator};

use crate::tokenizer::{self, Tokens};

#[derive(Copy, Clone)]
struct Token(usize);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Kind {
    Symbol,
    Int,
    BinaryOp,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: usize,
    pub kinds: Vec<Kind>,
    pub indices: Vec<usize>,
    pub binary_ops: BinaryOps,
    pub expressions: Vec<usize>,
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
    pub lefts: Vec<usize>,
    pub rights: Vec<usize>,
}

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub functions: Vec<Function>,
    pub symbols: Vec<String>,
    pub ints: Vec<String>,
    pub top_level: HashMap<String, usize>,
}

type Precedence = u8;

#[derive(Debug)]
enum InfixParser {
    BinaryOp(Precedence, BinaryOp),
}

struct ParseResult(Function, Token, usize);

const LOWEST: Precedence = 0;
const ADD: Precedence = 10;
const SUBTRACT: Precedence = ADD;
const MULTIPLY: Precedence = 20;
const DIVIDE: Precedence = MULTIPLY;

fn precedence_of(parser: &InfixParser) -> Precedence {
    match parser {
        InfixParser::BinaryOp(precedence, _) => *precedence,
    }
}

fn inc_token(Token(index): Token) -> Token {
    Token(index + 1)
}

fn fresh_entity(func: &Function) -> usize {
    func.kinds.len()
}

fn parse_primitive(mut func: Function, tokens: &Tokens, token: Token, kind: Kind) -> ParseResult {
    let entity = fresh_entity(&func);
    func.kinds.push(kind);
    func.indices.push(tokens.indices[token.0]);
    ParseResult(func, token, entity)
}

fn prefix_parser(
    func: Function,
    tokens: &Tokens,
    token: Token,
    kind: tokenizer::Kind,
) -> ParseResult {
    match kind {
        tokenizer::Kind::Symbol => parse_primitive(func, tokens, token, Kind::Symbol),
        tokenizer::Kind::Int => parse_primitive(func, tokens, token, Kind::Int),
        token => panic!("no prefix parser for {:?}", token),
    }
}

fn consume(tokens: &Tokens, token: Token, kind: tokenizer::Kind) -> Token {
    assert_eq!(tokens.kinds[token.0], kind);
    inc_token(token)
}

fn parse_binary_op(
    precedence: Precedence,
    binary_op: BinaryOp,
    func: Function,
    tokens: &Tokens,
    token: Token,
    left: usize,
) -> ParseResult {
    let ParseResult(mut func, token, right) = parse_expression(func, tokens, token, precedence);
    let entity = fresh_entity(&func);
    func.kinds.push(Kind::BinaryOp);
    func.indices.push(func.binary_ops.lefts.len());
    func.binary_ops.ops.push(binary_op);
    func.binary_ops.lefts.push(left);
    func.binary_ops.rights.push(right);
    ParseResult(func, token, entity)
}

fn infix_parser(kind: tokenizer::Kind) -> Option<InfixParser> {
    match kind {
        tokenizer::Kind::Plus => Some(InfixParser::BinaryOp(ADD, BinaryOp::Add)),
        tokenizer::Kind::Minus => Some(InfixParser::BinaryOp(SUBTRACT, BinaryOp::Subtract)),
        tokenizer::Kind::Times => Some(InfixParser::BinaryOp(MULTIPLY, BinaryOp::Multiply)),
        tokenizer::Kind::Slash => Some(InfixParser::BinaryOp(DIVIDE, BinaryOp::Divide)),
        _ => None,
    }
}

fn run_infix_parser(
    parser: InfixParser,
    func: Function,
    tokens: &Tokens,
    token: Token,
    left: usize,
) -> ParseResult {
    match parser {
        InfixParser::BinaryOp(precedence, binary_op) => {
            parse_binary_op(precedence, binary_op, func, tokens, token, left)
        }
    }
}

fn parse_right(
    func: Function,
    tokens: &Tokens,
    token: Token,
    left: usize,
    precedence: Precedence,
) -> ParseResult {
    let parser = tokens
        .kinds
        .get(token.0)
        .map(|&kind| infix_parser(kind))
        .flatten();
    match parser {
        Some(parser) if precedence < precedence_of(&parser) => {
            let ParseResult(func, token, left) =
                run_infix_parser(parser, func, tokens, inc_token(token), left);
            parse_right(func, tokens, token, left, precedence)
        }
        _ => ParseResult(func, token, left),
    }
}

fn parse_expression(
    func: Function,
    tokens: &Tokens,
    token: Token,
    precedence: Precedence,
) -> ParseResult {
    let kind = tokens.kinds[token.0];
    let ParseResult(func, token, left) = prefix_parser(func, tokens, token, kind);
    parse_right(func, tokens, inc_token(token), left, precedence)
}

fn parse_function(tokens: &Tokens, token: Token) -> Function {
    let token = consume(tokens, token, tokenizer::Kind::Def);
    assert_eq!(tokens.kinds[token.0], tokenizer::Kind::Symbol);
    let func = Function {
        name: tokens.indices[token.0],
        kinds: vec![],
        indices: vec![],
        binary_ops: BinaryOps {
            ops: vec![],
            lefts: vec![],
            rights: vec![],
        },
        expressions: vec![],
    };
    let token = consume(tokens, inc_token(token), tokenizer::Kind::LeftParen);
    let token = consume(tokens, token, tokenizer::Kind::RightParen);
    let token = consume(tokens, token, tokenizer::Kind::Colon);
    let ParseResult(mut func, _, body) = parse_expression(func, tokens, token, LOWEST);
    func.expressions.push(body);
    func
}

pub fn parse(tokens: Tokens) -> Ast {
    let func = parse_function(&tokens, Token(0));
    let name = tokens.symbols[func.name].clone();
    Ast {
        functions: vec![func],
        symbols: tokens.symbols,
        ints: tokens.ints,
        top_level: HashMap::from_iter([(name, 0)]),
    }
}
