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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct AstEntity(pub usize);

#[derive(Debug, PartialEq)]
pub struct Ast<'a> {
    pub indices: Vec<usize>,
    pub kinds: Vec<AstKind>,
    pub strings: Vec<&'a str>,
    pub children: Vec<Vec<AstEntity>>,
    pub top_level: Vec<AstEntity>,
}

pub enum ExpressionKind {
    Call,
    Return,
}

pub struct IrEntity(pub usize);

pub struct Entities<'a> {
    pub literals: HashMap<IrEntity, &'a str>,
}

pub struct Calls {
    pub func: Vec<IrEntity>,
    pub args: Vec<IrEntity>,
}

pub struct BasicBlock {
    pub kinds: Vec<ExpressionKind>,
    pub indices: Vec<usize>,
    pub calls: Calls,
    pub returns: Vec<IrEntity>,
}

pub struct Environment<'a> {
    pub basic_blocks: Vec<BasicBlock>,
    pub entities: Entities<'a>,
    pub current_basic_block: usize,
}

pub struct TopLevel<'a> {
    pub name: &'a str,
    pub env: Environment<'a>,
}

pub struct Ir<'a> {
    pub top_level: Vec<TopLevel<'a>>,
}
