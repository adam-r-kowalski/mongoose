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

#[derive(Debug, PartialEq)]
pub enum ExpressionKind {
    Call,
    Return,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct IrEntity(pub usize);

#[derive(Debug, PartialEq)]
pub struct Entities<'a> {
    pub name_to_entity: HashMap<&'a str, IrEntity>,
    pub literals: HashMap<IrEntity, &'a str>,
    pub next_entity: IrEntity,
}

#[derive(Debug, PartialEq)]
pub struct Calls {
    pub functions: Vec<IrEntity>,
    pub arguments: Vec<Vec<IrEntity>>,
    pub returns: Vec<IrEntity>,
}

#[derive(Debug, PartialEq)]
pub struct IrBlock {
    pub kinds: Vec<ExpressionKind>,
    pub indices: Vec<usize>,
    pub calls: Calls,
    pub returns: Vec<IrEntity>,
}

#[derive(Debug, PartialEq)]
pub struct Environment<'a> {
    pub basic_blocks: Vec<IrBlock>,
    pub entities: Entities<'a>,
    pub current_basic_block: usize,
}

#[derive(Debug, PartialEq)]
pub struct TopLevel<'a> {
    pub name: &'a str,
    pub environment: Environment<'a>,
    pub type_entity: IrEntity,
    pub type_basic_block: usize,
    pub value_entity: IrEntity,
    pub value_basic_block: usize,
}

#[derive(Debug, PartialEq)]
pub struct Ir<'a> {
    pub top_level: Vec<TopLevel<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Push,
    Mov,
    Syscall,
}

#[derive(Debug, PartialEq)]
pub enum OperandKind {
    Register,
    Literal,
    Int,
}

#[derive(Debug, PartialEq)]
pub enum Register {
    Rbp,
    Rsp,
    Edi,
    Rax,
}

#[derive(Debug, PartialEq)]
pub struct X86Block<'a> {
    pub instructions: Vec<Instruction>,
    pub operand_kinds: Vec<Vec<OperandKind>>,
    pub operands: Vec<Vec<usize>>,
    pub literals: Vec<&'a str>,
}

#[derive(Debug, PartialEq)]
pub struct X86<'a> {
    pub blocks: Vec<X86Block<'a>>,
}
