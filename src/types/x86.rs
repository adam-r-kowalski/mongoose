use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    Push,
    Mov,
    Syscall,
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    Register,
    Literal,
    Int,
}

#[derive(Debug, PartialEq)]
#[repr(usize)]
pub enum Register {
    Rbp,
    Rsp,
    Edi,
    Rax,
}

#[derive(Debug, PartialEq)]
pub struct Block<'a> {
    pub instructions: Vec<Instruction>,
    pub operand_kinds: Vec<Vec<Kind>>,
    pub operands: Vec<Vec<usize>>,
    pub literals: Vec<&'a str>,
}

#[derive(Debug, PartialEq)]
pub struct TopLevel<'a> {
    pub blocks: Vec<Block<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct X86<'a> {
    pub top_level: Vec<TopLevel<'a>>,
    pub name_to_top_level: HashMap<&'a str, usize>,
}
