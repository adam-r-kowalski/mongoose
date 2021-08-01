use crate::parser::{self};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    I64Const,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
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

fn codegen_int(mut wasm: Wasm, func: &parser::Function, entity: usize) -> Wasm {
    wasm.function.instructions.push(Instruction::I64Const);
    wasm.function
        .operand_kinds
        .push(vec![OperandKind::IntLiteral]);
    wasm.function.operands.push(vec![func.indices[entity]]);
    wasm
}

fn codegen_binary_op(wasm: Wasm, func: &parser::Function, entity: usize) -> Wasm {
    let index = func.indices[entity];
    let wasm = codegen_expression(wasm, func, func.binary_ops.lefts[index]);
    let mut wasm = codegen_expression(wasm, func, func.binary_ops.rights[index]);
    let instruction = match func.binary_ops.ops[index] {
        parser::BinaryOp::Add => Instruction::I64Add,
        parser::BinaryOp::Subtract => Instruction::I64Sub,
        parser::BinaryOp::Multiply => Instruction::I64Mul,
        parser::BinaryOp::Divide => Instruction::I64DivS,
    };
    wasm.function.instructions.push(instruction);
    wasm.function.operand_kinds.push(vec![]);
    wasm.function.operands.push(vec![]);
    wasm
}

fn codegen_expression(wasm: Wasm, func: &parser::Function, entity: usize) -> Wasm {
    match func.kinds[entity] {
        parser::Kind::Int => codegen_int(wasm, func, entity),
        parser::Kind::BinaryOp => codegen_binary_op(wasm, func, entity),
        kind => panic!("codegen expression for kind {:?} not implemented", kind),
    }
}

pub fn codegen(ast: parser::Ast) -> Wasm {
    let wasm = Wasm {
        function: Function {
            instructions: vec![],
            operand_kinds: vec![],
            operands: vec![],
        },
        symbols: vec![],
        ints: vec![],
    };
    let start = *ast.top_level.get("start").unwrap();
    let func = &ast.functions[start];
    let mut wasm = codegen_expression(wasm, func, func.expressions[0]);
    wasm.symbols = ast.symbols;
    wasm.ints = ast.ints;
    wasm
}
