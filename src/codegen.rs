use std::collections::HashMap;

use crate::parser::{self, Ast};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    I64Const,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    SetLocal,
    GetLocal,
}

#[derive(Debug, PartialEq)]
pub enum OperandKind {
    IntLiteral,
    Local,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub instructions: Vec<Instruction>,
    pub operand_kinds: Vec<Vec<OperandKind>>,
    pub operands: Vec<Vec<usize>>,
    pub locals: Vec<String>,
    pub name_to_local: HashMap<String, usize>,
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

fn codegen_binary_op(wasm: Wasm, ast: &Ast, func: &parser::Function, entity: usize) -> Wasm {
    let index = func.indices[entity];
    let wasm = codegen_expression(wasm, ast, func, func.binary_ops.lefts[index]);
    let mut wasm = codegen_expression(wasm, ast, func, func.binary_ops.rights[index]);
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

fn codegen_definition(wasm: Wasm, ast: &Ast, func: &parser::Function, entity: usize) -> Wasm {
    let index = func.indices[entity];
    let name_index = func.definitions.names[index];
    assert_eq!(func.kinds[name_index], parser::Kind::Symbol);
    let mut wasm = codegen_expression(wasm, ast, func, func.definitions.values[index]);
    let name = ast.symbols[func.indices[name_index]].clone();
    let local = wasm.function.locals.len();
    wasm.function.locals.push(format!("${}", name));
    wasm.function.name_to_local.try_insert(name, local).unwrap();
    wasm.function.instructions.push(Instruction::SetLocal);
    wasm.function.operand_kinds.push(vec![OperandKind::Local]);
    wasm.function.operands.push(vec![local]);
    wasm
}

fn codegen_symbol(mut wasm: Wasm, ast: &Ast, func: &parser::Function, entity: usize) -> Wasm {
    let index = func.indices[entity];
    let local = wasm
        .function
        .name_to_local
        .get(&ast.symbols[index])
        .unwrap();
    wasm.function.instructions.push(Instruction::GetLocal);
    wasm.function.operand_kinds.push(vec![OperandKind::Local]);
    wasm.function.operands.push(vec![*local]);
    wasm
}

fn codegen_expression(wasm: Wasm, ast: &Ast, func: &parser::Function, entity: usize) -> Wasm {
    match func.kinds[entity] {
        parser::Kind::Int => codegen_int(wasm, func, entity),
        parser::Kind::BinaryOp => codegen_binary_op(wasm, ast, func, entity),
        parser::Kind::Definition => codegen_definition(wasm, ast, func, entity),
        parser::Kind::Symbol => codegen_symbol(wasm, ast, func, entity),
    }
}

pub fn codegen(ast: Ast) -> Wasm {
    let wasm = Wasm {
        function: Function {
            instructions: vec![],
            operand_kinds: vec![],
            operands: vec![],
            locals: vec![],
            name_to_local: HashMap::new(),
        },
        symbols: vec![],
        ints: vec![],
    };
    let start = *ast.top_level.get("start").unwrap();
    let func = &ast.functions[start];
    let mut wasm = func.expressions.iter().fold(wasm, |wasm, &expression| {
        codegen_expression(wasm, &ast, func, expression)
    });
    wasm.symbols = ast.symbols;
    wasm.ints = ast.ints;
    wasm
}
