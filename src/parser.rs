use rayon::prelude::*;
use std::collections::HashMap;

use crate::tokenizer::{self, Tokens};

#[derive(Copy, Clone)]
struct Token(usize);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Kind {
    Symbol,
    Int,
    BinaryOp,
    Definition,
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
pub struct Definitions {
    pub names: Vec<usize>,
    pub values: Vec<usize>,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: usize,
    pub kinds: Vec<Kind>,
    pub indices: Vec<usize>,
    pub binary_ops: BinaryOps,
    pub definitions: Definitions,
    pub expressions: Vec<usize>,
    pub symbols: Vec<String>,
    pub ints: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub functions: Vec<Function>,
    pub top_level: HashMap<String, usize>,
}

type Precedence = u8;

#[derive(Debug)]
enum InfixParser {
    BinaryOp(Precedence, BinaryOp),
    Definition,
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
        InfixParser::Definition => LOWEST,
    }
}

fn inc_token(Token(index): Token) -> Token {
    Token(index + 1)
}

fn fresh_entity(func: &Function) -> usize {
    func.kinds.len()
}

fn parse_primitive(
    mut func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    kind: Kind,
) -> ParseResult {
    let entity = fresh_entity(&func);
    func.kinds.push(kind);
    func.indices.push(top_level.indices[token.0]);
    ParseResult(func, token, entity)
}

fn prefix_parser(
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    kind: tokenizer::Kind,
) -> ParseResult {
    match kind {
        tokenizer::Kind::Symbol => parse_primitive(func, top_level, token, Kind::Symbol),
        tokenizer::Kind::Int => parse_primitive(func, top_level, token, Kind::Int),
        token => panic!("no prefix parser for {:?}", token),
    }
}

fn consume(top_level: &tokenizer::TopLevel, token: Token, kind: tokenizer::Kind) -> Token {
    assert_eq!(top_level.kinds[token.0], kind);
    inc_token(token)
}

fn parse_binary_op(
    precedence: Precedence,
    binary_op: BinaryOp,
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    left: usize,
) -> ParseResult {
    let ParseResult(mut func, token, right) = parse_expression(func, top_level, token, precedence);
    let entity = fresh_entity(&func);
    func.kinds.push(Kind::BinaryOp);
    func.indices.push(func.binary_ops.lefts.len());
    func.binary_ops.ops.push(binary_op);
    func.binary_ops.lefts.push(left);
    func.binary_ops.rights.push(right);
    ParseResult(func, token, entity)
}

fn parse_definition(
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    name: usize,
) -> ParseResult {
    let ParseResult(mut func, token, value) = parse_expression(func, top_level, token, 0);
    let entity = fresh_entity(&func);
    func.kinds.push(Kind::Definition);
    func.indices.push(func.definitions.names.len());
    func.definitions.names.push(name);
    func.definitions.values.push(value);
    ParseResult(func, token, entity)
}

fn infix_parser(kind: tokenizer::Kind) -> Option<InfixParser> {
    match kind {
        tokenizer::Kind::Plus => Some(InfixParser::BinaryOp(ADD, BinaryOp::Add)),
        tokenizer::Kind::Minus => Some(InfixParser::BinaryOp(SUBTRACT, BinaryOp::Subtract)),
        tokenizer::Kind::Times => Some(InfixParser::BinaryOp(MULTIPLY, BinaryOp::Multiply)),
        tokenizer::Kind::Slash => Some(InfixParser::BinaryOp(DIVIDE, BinaryOp::Divide)),
        tokenizer::Kind::Equal => Some(InfixParser::Definition),
        _ => None,
    }
}

fn run_infix_parser(
    parser: InfixParser,
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    left: usize,
) -> ParseResult {
    match parser {
        InfixParser::BinaryOp(precedence, binary_op) => {
            parse_binary_op(precedence, binary_op, func, top_level, token, left)
        }
        InfixParser::Definition => parse_definition(func, top_level, token, left),
    }
}

fn parse_right(
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    left: usize,
    precedence: Precedence,
) -> ParseResult {
    let parser = top_level
        .kinds
        .get(token.0)
        .map(|&kind| infix_parser(kind))
        .flatten();
    match parser {
        Some(parser) if precedence <= precedence_of(&parser) => {
            let ParseResult(func, token, left) =
                run_infix_parser(parser, func, top_level, inc_token(token), left);
            parse_right(func, top_level, token, left, precedence)
        }
        _ => ParseResult(func, token, left),
    }
}

fn parse_expression(
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    precedence: Precedence,
) -> ParseResult {
    let kind = top_level.kinds[token.0];
    let ParseResult(func, token, left) = prefix_parser(func, top_level, token, kind);
    parse_right(func, top_level, inc_token(token), left, precedence)
}

fn parse_function_body(
    func: Function,
    top_level: &tokenizer::TopLevel,
    mut token: Token,
) -> Function {
    if token.0 >= top_level.kinds.len() {
        func
    } else {
        if top_level.kinds[token.0] == tokenizer::Kind::Indent {
            token = inc_token(token);
        }
        let ParseResult(mut func, token, body) = parse_expression(func, top_level, token, LOWEST);
        func.expressions.push(body);
        parse_function_body(func, top_level, token)
    }
}

fn parse_function(top_level: &tokenizer::TopLevel, token: Token) -> Function {
    let token = consume(top_level, token, tokenizer::Kind::Def);
    assert_eq!(top_level.kinds[token.0], tokenizer::Kind::Symbol);
    let func = Function {
        name: top_level.indices[token.0],
        kinds: vec![],
        indices: vec![],
        binary_ops: BinaryOps {
            ops: vec![],
            lefts: vec![],
            rights: vec![],
        },
        definitions: Definitions {
            names: vec![],
            values: vec![],
        },
        expressions: vec![],
        symbols: vec![],
        ints: vec![],
    };
    let token = consume(top_level, inc_token(token), tokenizer::Kind::LeftParen);
    let token = consume(top_level, token, tokenizer::Kind::RightParen);
    let token = consume(top_level, token, tokenizer::Kind::Colon);
    parse_function_body(func, top_level, token)
}

pub fn parse(tokens: Tokens) -> Ast {
    let functions: Vec<Function> = tokens
        .top_level
        .into_par_iter()
        .map(|top_level| {
            let mut func = parse_function(&top_level, Token(0));
            func.symbols = top_level.symbols;
            func.ints = top_level.ints;
            func
        })
        .collect();
    let top_level =
        functions
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut top_level, (i, func)| {
                top_level
                    .try_insert(func.symbols[func.name].clone(), i)
                    .unwrap();
                top_level
            });
    Ast {
        functions,
        top_level,
    }
}
