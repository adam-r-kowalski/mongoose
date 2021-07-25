use crate::tokenizer::{self, Tokens};

struct Entity {
    index: usize,
}

struct Token {
    index: usize,
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    Symbol,
    Int,
    Function,
}

pub struct Ast {
    pub kinds: Vec<Kind>,
    pub indices: Vec<usize>,
    pub symbols: Vec<String>,
    pub ints: Vec<String>,
}

fn inc_token(Token { index }: Token) -> Token {
    Token { index: index + 1 }
}

fn parse_symbol(mut ast: Ast, tokens: &Tokens, token: Token) -> (Ast, Token, Entity) {
    let entity = Entity {
        index: ast.kinds.len(),
    };
    ast.kinds.push(Kind::Symbol);
    ast.indices.push(tokens.indices[token.index]);
    (ast, inc_token(token), entity)
}

fn parse_int(mut ast: Ast, tokens: &Tokens, token: Token) -> (Ast, Token, Entity) {
    let entity = Entity {
        index: ast.kinds.len(),
    };
    ast.kinds.push(Kind::Int);
    ast.indices.push(tokens.indices[token.index]);
    (ast, inc_token(token), entity)
}

type PrefixParser = impl Fn(Ast, &Tokens, Token) -> (Ast, Token, Entity);

fn prefix_parser(kind: tokenizer::Kind) -> PrefixParser {
    match kind {
        tokenizer::Kind::Symbol => parse_symbol,
        tokenizer::Kind::Int => parse_int,
        token => panic!("no prefix parser for {:?}", token),
    }
}

fn consume(tokens: &Tokens, token: Token, kind: tokenizer::Kind) -> Token {
    assert_eq!(tokens.kinds[token.index], kind);
    inc_token(token)
}

fn parse_function(ast: Ast, tokens: &Tokens, token: Token, name: Entity) -> (Ast, Token, Entity) {
    assert_eq!(ast.kinds[name.index], Kind::Symbol);
    let token = consume(tokens, token, tokenizer::Kind::RightParen);
    let token = consume(tokens, token, tokenizer::Kind::Arrow);
    let (ast, token, return_type) = parse_expression(ast, tokens, token);
    assert_eq!(ast.kinds[return_type.index], Kind::Symbol);
    let token = consume(tokens, token, tokenizer::Kind::Equal);
    let (ast, token, body) = parse_expression(ast, tokens, token);
    panic!("got here");
}

type InfixParser = impl Fn(Ast, &Tokens, Token, Entity) -> (Ast, Token, Entity);

fn infix_parser(kind: tokenizer::Kind) -> Option<InfixParser> {
    match kind {
        tokenizer::Kind::LeftParen => Some(parse_function),
        _ => None,
    }
}

fn parse_expression(ast: Ast, tokens: &Tokens, token: Token) -> (Ast, Token, Entity) {
    let parse_prefix = prefix_parser(tokens.kinds[token.index]);
    let (ast, token, left) = parse_prefix(ast, tokens, token);
    let parse_infix = tokens
        .kinds
        .get(token.index)
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
        symbols: vec![],
        ints: vec![],
    };
    let (mut ast, _, _) = parse_expression(ast, &tokens, Token { index: 0 });
    ast.symbols = tokens.symbols;
    ast.ints = tokens.ints;
    ast
}
