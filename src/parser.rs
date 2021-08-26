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
    Assign,
    FunctionCall,
    If,
    While,
    Grouping,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, PartialEq)]
pub struct BinaryOps {
    pub ops: Vec<BinaryOp>,
    pub lefts: Vec<usize>,
    pub rights: Vec<usize>,
}

#[derive(Debug, PartialEq)]
pub struct Assignments {
    pub names: Vec<usize>,
    pub values: Vec<usize>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCalls {
    pub names: Vec<usize>,
    pub parameters: Vec<Vec<usize>>,
}

#[derive(Debug, PartialEq)]
pub struct Ifs {
    pub conditionals: Vec<usize>,
    pub then_branches: Vec<Vec<usize>>,
    pub else_branches: Vec<Vec<usize>>,
}

#[derive(Debug, PartialEq)]
pub struct Whiles {
    pub conditionals: Vec<usize>,
    pub bodies: Vec<Vec<usize>>,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: usize,
    pub arguments: Vec<usize>,
    pub kinds: Vec<Kind>,
    pub indices: Vec<usize>,
    pub binary_ops: BinaryOps,
    pub assignments: Assignments,
    pub function_calls: FunctionCalls,
    pub expressions: Vec<usize>,
    pub symbols: Vec<String>,
    pub ints: Vec<String>,
    pub ifs: Ifs,
    pub whiles: Whiles,
    pub groupings: Vec<usize>,
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
    FunctionCall,
    Pipeline,
    NewLinePipeline,
}

struct ParseResult(Function, Token, usize);

const LOWEST: Precedence = 0;
const IS_EQUAL: Precedence = LOWEST + 10;
const NOT_EQUAL: Precedence = IS_EQUAL;
const LESS_THAN: Precedence = IS_EQUAL;
const LESS_THAN_EQUAL: Precedence = IS_EQUAL;
const GREATER_THAN: Precedence = IS_EQUAL;
const GREATER_THAN_EQUAL: Precedence = IS_EQUAL;
const BITWISE_OR: Precedence = LESS_THAN + 10;
const BITWISE_XOR: Precedence = BITWISE_OR + 10;
const BITWISE_AND: Precedence = BITWISE_XOR + 10;
const SHIFT_LEFT: Precedence = BITWISE_AND + 10;
const SHIFT_RIGHT: Precedence = SHIFT_LEFT;
const ADD: Precedence = SHIFT_LEFT + 10;
const SUBTRACT: Precedence = ADD;
const MULTIPLY: Precedence = ADD + 10;
const DIVIDE: Precedence = MULTIPLY;
const MODULO: Precedence = MULTIPLY;
const HIGHEST: Precedence = DIVIDE + 10;

fn precedence_of(parser: &InfixParser) -> Precedence {
    match parser {
        InfixParser::BinaryOp(precedence, _) => *precedence,
        InfixParser::Definition => LOWEST,
        InfixParser::FunctionCall => HIGHEST,
        InfixParser::Pipeline => HIGHEST,
        InfixParser::NewLinePipeline => HIGHEST,
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

fn parse_then_branch(
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    mut expressions: Vec<usize>,
) -> (Function, Token, Vec<usize>) {
    match top_level.kinds[token.0] {
        tokenizer::Kind::Indent => {
            parse_then_branch(func, top_level, inc_token(token), expressions)
        }
        tokenizer::Kind::Else => (func, token, expressions),
        _ => {
            let ParseResult(func, token, expression) =
                parse_expression(func, top_level, token, LOWEST);
            expressions.push(expression);
            parse_then_branch(func, top_level, token, expressions)
        }
    }
}

fn parse_else_branch(
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    mut expressions: Vec<usize>,
    indent: usize,
) -> (Function, Token, Vec<usize>) {
    match top_level.kinds.get(token.0) {
        Some(tokenizer::Kind::Indent) => {
            let next_indent = top_level.indents[top_level.indices[token.0]];
            if indent != 0 && indent != next_indent {
                (func, token, expressions)
            } else {
                parse_else_branch(func, top_level, inc_token(token), expressions, next_indent)
            }
        }
        Some(_) => {
            let ParseResult(func, token, expression) =
                parse_expression(func, top_level, token, LOWEST);
            expressions.push(expression);
            if indent > 0 {
                parse_else_branch(func, top_level, token, expressions, indent)
            } else {
                (func, token, expressions)
            }
        }
        None => (func, token, expressions),
    }
}

fn parse_if(func: Function, top_level: &tokenizer::TopLevel, token: Token) -> ParseResult {
    let token = consume(top_level, token, tokenizer::Kind::If);
    let ParseResult(func, token, conditional) = parse_expression(func, top_level, token, LOWEST);
    let token = consume(top_level, token, tokenizer::Kind::Colon);
    let (func, token, then_branch) = parse_then_branch(func, top_level, token, vec![]);
    let token = consume(top_level, token, tokenizer::Kind::Else);
    let token = consume(top_level, token, tokenizer::Kind::Colon);
    let (mut func, token, else_branch) = parse_else_branch(func, top_level, token, vec![], 0);
    let entity = fresh_entity(&func);
    func.kinds.push(Kind::If);
    func.indices.push(func.ifs.conditionals.len());
    func.ifs.conditionals.push(conditional);
    func.ifs.then_branches.push(then_branch);
    func.ifs.else_branches.push(else_branch);
    ParseResult(func, token, entity)
}

fn parse_while_body(
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    mut expressions: Vec<usize>,
    indent: usize,
) -> (Function, Token, Vec<usize>) {
    match top_level.kinds.get(token.0) {
        Some(tokenizer::Kind::Indent) => {
            let next_indent = top_level.indents[top_level.indices[token.0]];
            if indent != 0 && indent != next_indent {
                (func, token, expressions)
            } else {
                parse_while_body(func, top_level, inc_token(token), expressions, next_indent)
            }
        }
        Some(_) => {
            let ParseResult(func, token, expression) =
                parse_expression(func, top_level, token, LOWEST);
            expressions.push(expression);
            if indent > 0 {
                parse_while_body(func, top_level, token, expressions, indent)
            } else {
                (func, token, expressions)
            }
        }
        None => (func, token, expressions),
    }
}

fn parse_while(func: Function, top_level: &tokenizer::TopLevel, token: Token) -> ParseResult {
    let token = consume(top_level, token, tokenizer::Kind::While);
    let ParseResult(func, token, conditional) = parse_expression(func, top_level, token, LOWEST);
    let token = consume(top_level, token, tokenizer::Kind::Colon);
    let (mut func, token, body) = parse_while_body(func, top_level, token, vec![], 0);
    let entity = fresh_entity(&func);
    func.kinds.push(Kind::While);
    func.indices.push(func.whiles.conditionals.len());
    func.whiles.conditionals.push(conditional);
    func.whiles.bodies.push(body);
    ParseResult(func, token, entity)
}

fn parse_grouping(func: Function, top_level: &tokenizer::TopLevel, token: Token) -> ParseResult {
    let token = consume(top_level, token, tokenizer::Kind::LeftParen);
    let ParseResult(mut func, token, expression) = parse_expression(func, top_level, token, LOWEST);
    let token = consume(top_level, token, tokenizer::Kind::RightParen);
    let entity = fresh_entity(&func);
    func.kinds.push(Kind::Grouping);
    func.indices.push(func.groupings.len());
    func.groupings.push(expression);
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
        tokenizer::Kind::If => parse_if(func, top_level, token),
        tokenizer::Kind::While => parse_while(func, top_level, token),
        tokenizer::Kind::LeftParen => parse_grouping(func, top_level, token),
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

fn parse_assignment(
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    name: usize,
) -> ParseResult {
    let ParseResult(mut func, token, value) = parse_expression(func, top_level, token, 0);
    let entity = fresh_entity(&func);
    func.kinds.push(Kind::Assign);
    func.indices.push(func.assignments.names.len());
    func.assignments.names.push(name);
    func.assignments.values.push(value);
    ParseResult(func, token, entity)
}

fn parse_function_parameters(
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    mut parameters: Vec<usize>,
) -> (Function, Token, Vec<usize>) {
    let ParseResult(func, token, parameter) = parse_expression(func, top_level, token, 0);
    parameters.push(parameter);
    match top_level.kinds[token.0] {
        tokenizer::Kind::Comma => {
            parse_function_parameters(func, top_level, inc_token(token), parameters)
        }
        tokenizer::Kind::RightParen => (func, token, parameters),
        kind => panic!(
            "Parsing function parameters, expected comma or right paren, found {:?}",
            kind
        ),
    }
}

fn parse_function_call(
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    name: usize,
) -> ParseResult {
    assert_eq!(func.kinds[name], Kind::Symbol);
    let (mut func, token, parameters) = if top_level.kinds[token.0] != tokenizer::Kind::RightParen {
        parse_function_parameters(func, top_level, token, vec![])
    } else {
        (func, token, vec![])
    };
    let token = consume(top_level, token, tokenizer::Kind::RightParen);
    let entity = fresh_entity(&func);
    func.kinds.push(Kind::FunctionCall);
    func.indices.push(func.function_calls.names.len());
    func.function_calls.names.push(name);
    func.function_calls.parameters.push(parameters);
    ParseResult(func, token, entity)
}

fn parse_pipeline_parameters(
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    mut parameters: Vec<usize>,
    seen_underscore: bool,
) -> (Function, Token, Vec<usize>) {
    let ParseResult(func, token, parameter) = parse_expression(func, top_level, token, 0);
    let is_symbol = func.kinds[parameter] == Kind::Symbol;
    let is_underscore = top_level.symbols[func.indices[parameter]] == "_";
    let (parameters, seen_underscore) = if is_symbol && is_underscore {
        assert!(!seen_underscore);
        let parameter = parameters.remove(0);
        parameters.push(parameter);
        (parameters, true)
    } else {
        parameters.push(parameter);
        (parameters, seen_underscore)
    };
    match top_level.kinds[token.0] {
        tokenizer::Kind::Comma => parse_pipeline_parameters(
            func,
            top_level,
            inc_token(token),
            parameters,
            seen_underscore,
        ),
        tokenizer::Kind::RightParen => (func, token, parameters),
        kind => panic!(
            "Parsing function parameters, expected comma or right paren, found {:?}",
            kind
        ),
    }
}

fn parse_pipeline(
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    first_parameter: usize,
) -> ParseResult {
    let ParseResult(func, token, name) = parse_primitive(func, top_level, token, Kind::Symbol);
    let token = inc_token(token);
    let parameters = vec![first_parameter];
    let (mut func, token, parameters) = match top_level.kinds.get(token.0) {
        Some(tokenizer::Kind::LeftParen) => {
            let token = inc_token(token);
            let (func, token, parameters) =
                if top_level.kinds[token.0] != tokenizer::Kind::RightParen {
                    parse_pipeline_parameters(func, top_level, token, parameters, false)
                } else {
                    (func, token, parameters)
                };
            let token = consume(top_level, token, tokenizer::Kind::RightParen);
            (func, token, parameters)
        }
        _ => (func, token, parameters),
    };
    let entity = fresh_entity(&func);
    func.kinds.push(Kind::FunctionCall);
    func.indices.push(func.function_calls.names.len());
    func.function_calls.names.push(name);
    func.function_calls.parameters.push(parameters);
    ParseResult(func, token, entity)
}

fn parse_new_line_pipeline(
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    first_parameter: usize,
) -> ParseResult {
    let token = consume(top_level, token, tokenizer::Kind::VerticalBarGreaterThan);
    parse_pipeline(func, top_level, token, first_parameter)
}

fn infix_parser(top_level: &tokenizer::TopLevel, token: Token) -> Option<InfixParser> {
    top_level
        .kinds
        .get(token.0)
        .map(|kind| match kind {
            tokenizer::Kind::Plus => Some(InfixParser::BinaryOp(ADD, BinaryOp::Add)),
            tokenizer::Kind::Minus => Some(InfixParser::BinaryOp(SUBTRACT, BinaryOp::Subtract)),
            tokenizer::Kind::Asterisk => Some(InfixParser::BinaryOp(MULTIPLY, BinaryOp::Multiply)),
            tokenizer::Kind::Slash => Some(InfixParser::BinaryOp(DIVIDE, BinaryOp::Divide)),
            tokenizer::Kind::Percent => Some(InfixParser::BinaryOp(MODULO, BinaryOp::Modulo)),
            tokenizer::Kind::LessThan => Some(InfixParser::BinaryOp(LESS_THAN, BinaryOp::LessThan)),
            tokenizer::Kind::LessThanEqual => Some(InfixParser::BinaryOp(
                LESS_THAN_EQUAL,
                BinaryOp::LessThanEqual,
            )),
            tokenizer::Kind::LessThanLessThan => {
                Some(InfixParser::BinaryOp(SHIFT_LEFT, BinaryOp::ShiftLeft))
            }
            tokenizer::Kind::GreaterThan => {
                Some(InfixParser::BinaryOp(GREATER_THAN, BinaryOp::GreaterThan))
            }
            tokenizer::Kind::GreaterThanEqual => Some(InfixParser::BinaryOp(
                GREATER_THAN_EQUAL,
                BinaryOp::GreaterThanEqual,
            )),
            tokenizer::Kind::GreaterThanGreaterThan => {
                Some(InfixParser::BinaryOp(SHIFT_RIGHT, BinaryOp::ShiftRight))
            }
            tokenizer::Kind::ExclamationEqual => {
                Some(InfixParser::BinaryOp(NOT_EQUAL, BinaryOp::NotEqual))
            }
            tokenizer::Kind::EqualEqual => Some(InfixParser::BinaryOp(IS_EQUAL, BinaryOp::Equal)),
            tokenizer::Kind::Equal => Some(InfixParser::Definition),
            tokenizer::Kind::Ampersand => {
                Some(InfixParser::BinaryOp(BITWISE_AND, BinaryOp::BitwiseAnd))
            }
            tokenizer::Kind::VerticalBar => {
                Some(InfixParser::BinaryOp(BITWISE_OR, BinaryOp::BitwiseOr))
            }
            tokenizer::Kind::Caret => {
                Some(InfixParser::BinaryOp(BITWISE_XOR, BinaryOp::BitwiseXor))
            }
            tokenizer::Kind::LeftParen => Some(InfixParser::FunctionCall),
            tokenizer::Kind::VerticalBarGreaterThan => Some(InfixParser::Pipeline),
            tokenizer::Kind::Indent => top_level
                .kinds
                .get(token.0 + 1)
                .map(|kind| match kind {
                    tokenizer::Kind::VerticalBarGreaterThan => Some(InfixParser::NewLinePipeline),
                    _ => None,
                })
                .flatten(),
            _ => None,
        })
        .flatten()
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
        InfixParser::Definition => parse_assignment(func, top_level, token, left),
        InfixParser::FunctionCall => parse_function_call(func, top_level, token, left),
        InfixParser::Pipeline => parse_pipeline(func, top_level, token, left),
        InfixParser::NewLinePipeline => parse_new_line_pipeline(func, top_level, token, left),
    }
}

fn parse_right(
    func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
    left: usize,
    precedence: Precedence,
) -> ParseResult {
    match infix_parser(top_level, token) {
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

fn parse_function_arguments(
    mut func: Function,
    top_level: &tokenizer::TopLevel,
    token: Token,
) -> (Function, Token) {
    assert_eq!(top_level.kinds[token.0], tokenizer::Kind::Symbol);
    func.arguments.push(top_level.indices[token.0]);
    let token = inc_token(token);
    match top_level.kinds[token.0] {
        tokenizer::Kind::Comma => parse_function_arguments(func, top_level, inc_token(token)),
        tokenizer::Kind::RightParen => (func, token),
        kind => panic!(
            "Parsing function arguments, expected comma or right paren, found {:?}",
            kind
        ),
    }
}

fn parse_function(top_level: &tokenizer::TopLevel, token: Token) -> Function {
    let token = consume(top_level, token, tokenizer::Kind::Def);
    assert_eq!(top_level.kinds[token.0], tokenizer::Kind::Symbol);
    let func = Function {
        name: top_level.indices[token.0],
        arguments: vec![],
        kinds: vec![],
        indices: vec![],
        binary_ops: BinaryOps {
            ops: vec![],
            lefts: vec![],
            rights: vec![],
        },
        assignments: Assignments {
            names: vec![],
            values: vec![],
        },
        function_calls: FunctionCalls {
            names: vec![],
            parameters: vec![],
        },
        expressions: vec![],
        symbols: vec![],
        ints: vec![],
        ifs: Ifs {
            conditionals: vec![],
            then_branches: vec![],
            else_branches: vec![],
        },
        whiles: Whiles {
            conditionals: vec![],
            bodies: vec![],
        },
        groupings: vec![],
    };
    let token = consume(top_level, inc_token(token), tokenizer::Kind::LeftParen);
    let (func, token) = if top_level.kinds[token.0] != tokenizer::Kind::RightParen {
        parse_function_arguments(func, top_level, token)
    } else {
        (func, token)
    };
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
