#[derive(Debug, PartialEq)]
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
pub enum Register {
    Rbp,
    Rsp,
    Edi,
    Rax,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub instructions: Vec<Instruction>,
    pub operand_kinds: Vec<Vec<Kind>>,
    pub operands: Vec<Vec<usize>>,
}

#[derive(Debug, PartialEq)]
pub struct TopLevel<'a> {
    pub blocks: Vec<Block>,
    pub literals: Vec<&'a str>,
}

#[derive(Debug, PartialEq)]
pub struct X86<'a> {
    pub top_level: Vec<TopLevel<'a>>,
}
