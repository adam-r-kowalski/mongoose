use crate::parser::{self, Ast};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    I32Const,
}

#[derive(Debug, PartialEq)]
pub enum OperandKind {
    IntLiteral,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub instructions: Vec<Instruction>,
    pub operand_kinds: Vec<Vec<OperandKind>>,
    pub operands: Vec<Vec<usize>>,
}

#[derive(Debug, PartialEq)]
pub struct Wasm {
    pub function: Function,
    pub symbols: Vec<String>,
    pub ints: Vec<String>,
}

pub fn codegen(ast: Ast) -> Wasm {
    let start = ast.top_level.get("start").unwrap();
    let start_index = ast.indices[start.0];
    let return_type = ast.functions.return_types[start_index];
    assert_eq!(ast.kinds[return_type.0], parser::Kind::Symbol);
    assert_eq!(ast.symbols[ast.indices[return_type.0]], "i64");
    let body = ast.functions.bodies[start_index];
    assert_eq!(ast.kinds[body.0], parser::Kind::Int);
    Wasm {
        function: Function {
            instructions: vec![Instruction::I32Const],
            operand_kinds: vec![vec![OperandKind::IntLiteral]],
            operands: vec![vec![0]],
        },
        symbols: ast.symbols,
        ints: ast.ints,
    }
}
