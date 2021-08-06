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
    pub name: usize,
    pub instructions: Vec<Instruction>,
    pub operand_kinds: Vec<Vec<OperandKind>>,
    pub operands: Vec<Vec<usize>>,
    pub locals: Vec<String>,
    pub name_to_local: HashMap<String, usize>,
    pub symbols: Vec<String>,
    pub ints: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Wasm {
    pub functions: Vec<Function>,
}

fn codegen_int(mut wasm_func: Function, ast_func: &parser::Function, entity: usize) -> Function {
    wasm_func.instructions.push(Instruction::I64Const);
    wasm_func.operand_kinds.push(vec![OperandKind::IntLiteral]);
    wasm_func.operands.push(vec![ast_func.indices[entity]]);
    wasm_func
}

fn codegen_binary_op(wasm_func: Function, ast_func: &parser::Function, entity: usize) -> Function {
    let index = ast_func.indices[entity];
    let wasm_func = codegen_expression(wasm_func, ast_func, ast_func.binary_ops.lefts[index]);
    let mut wasm_func = codegen_expression(wasm_func, ast_func, ast_func.binary_ops.rights[index]);
    let instruction = match ast_func.binary_ops.ops[index] {
        parser::BinaryOp::Add => Instruction::I64Add,
        parser::BinaryOp::Subtract => Instruction::I64Sub,
        parser::BinaryOp::Multiply => Instruction::I64Mul,
        parser::BinaryOp::Divide => Instruction::I64DivS,
    };
    wasm_func.instructions.push(instruction);
    wasm_func.operand_kinds.push(vec![]);
    wasm_func.operands.push(vec![]);
    wasm_func
}

fn codegen_definition(wasm_func: Function, ast_func: &parser::Function, entity: usize) -> Function {
    let index = ast_func.indices[entity];
    let name_index = ast_func.definitions.names[index];
    assert_eq!(ast_func.kinds[name_index], parser::Kind::Symbol);
    let mut wasm_func = codegen_expression(wasm_func, ast_func, ast_func.definitions.values[index]);
    let name = ast_func.symbols[ast_func.indices[name_index]].clone();
    let local = wasm_func.locals.len();
    wasm_func.locals.push(format!("${}", name));
    wasm_func.name_to_local.try_insert(name, local).unwrap();
    wasm_func.instructions.push(Instruction::SetLocal);
    wasm_func.operand_kinds.push(vec![OperandKind::Local]);
    wasm_func.operands.push(vec![local]);
    wasm_func
}

fn codegen_symbol(mut wasm_func: Function, ast_func: &parser::Function, entity: usize) -> Function {
    let index = ast_func.indices[entity];
    let local = wasm_func
        .name_to_local
        .get(&ast_func.symbols[index])
        .unwrap();
    wasm_func.instructions.push(Instruction::GetLocal);
    wasm_func.operand_kinds.push(vec![OperandKind::Local]);
    wasm_func.operands.push(vec![*local]);
    wasm_func
}

fn codegen_expression(wasm_func: Function, ast_func: &parser::Function, entity: usize) -> Function {
    match ast_func.kinds[entity] {
        parser::Kind::Int => codegen_int(wasm_func, ast_func, entity),
        parser::Kind::BinaryOp => codegen_binary_op(wasm_func, ast_func, entity),
        parser::Kind::Definition => codegen_definition(wasm_func, ast_func, entity),
        parser::Kind::Symbol => codegen_symbol(wasm_func, ast_func, entity),
        parser::Kind::FunctionCall => panic!("codegen ast_function call"),
    }
}

fn codegen_function(ast_func: &parser::Function) -> Function {
    let wasm_func = Function {
        name: ast_func.name,
        instructions: vec![],
        operand_kinds: vec![],
        operands: vec![],
        locals: vec![],
        name_to_local: HashMap::new(),
        symbols: vec![],
        ints: vec![],
    };
    let mut wasm_func = ast_func
        .expressions
        .iter()
        .fold(wasm_func, |wasm_func, &expression| {
            codegen_expression(wasm_func, ast_func, expression)
        });
    wasm_func.symbols = ast_func.symbols.clone();
    wasm_func.ints = ast_func.ints.clone();
    wasm_func
}

pub fn codegen(ast: Ast) -> Wasm {
    let start = *ast.top_level.get("start").unwrap();
    let wasm_func = codegen_function(&ast.functions[start]);
    Wasm {
        functions: vec![wasm_func],
    }
}
