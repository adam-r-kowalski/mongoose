use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum AstKind {
    Int,
    Float,
    Symbol,
    Keyword,
    Brackets,
    Parens,
}

#[derive(Debug, PartialEq)]
pub struct Ast<'a> {
    pub indices: Vec<usize>,
    pub kinds: Vec<AstKind>,
    pub strings: Vec<&'a str>,
    pub children: Vec<Vec<usize>>,
    pub top_level: Vec<usize>,
}

pub enum ExpressionKind {
    Call,
    Return,
}

pub struct Entity(usize);

pub struct Entities<'a> {
    pub literals: HashMap<Entity, &'a str>,
}

pub struct Calls {
    pub func: Vec<Entity>,
    pub args: Vec<Entity>,
}

pub struct BasicBlock {
    pub kinds: Vec<ExpressionKind>,
    pub indices: Vec<usize>,
    pub calls: Calls,
    pub returns: Vec<Entity>,
}

pub struct Environment<'a> {
    pub basic_blocks: Vec<BasicBlock>,
    pub entities: Entities<'a>,
}

pub struct TopLevel<'a> {
    pub name: &'a str,
    pub env: Environment<'a>,
}

pub struct Ir<'a> {
    pub top_level: Vec<TopLevel<'a>>,
}
