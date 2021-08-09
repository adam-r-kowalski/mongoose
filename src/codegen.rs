use std::collections::HashMap;
use std::sync::mpsc::{self, Sender};

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
    Call,
}

#[derive(Debug, PartialEq)]
pub enum OperandKind {
    IntLiteral,
    Local,
    Symbol,
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
    pub arguments: usize,
}

#[derive(Debug, PartialEq)]
pub struct Wasm {
    pub functions: Vec<Function>,
    pub name_to_function: HashMap<String, usize>,
}

enum Message {
    Spawn(String),
    Done,
}

fn codegen_int(mut wasm_func: Function, ast_func: &parser::Function, entity: usize) -> Function {
    wasm_func.instructions.push(Instruction::I64Const);
    wasm_func.operand_kinds.push(vec![OperandKind::IntLiteral]);
    wasm_func.operands.push(vec![ast_func.indices[entity]]);
    wasm_func
}

fn codegen_binary_op(
    tx: Sender<Message>,
    wasm_func: Function,
    ast_func: &parser::Function,
    entity: usize,
) -> Function {
    let index = ast_func.indices[entity];
    let wasm_func = codegen_expression(
        tx.clone(),
        wasm_func,
        ast_func,
        ast_func.binary_ops.lefts[index],
    );
    let mut wasm_func =
        codegen_expression(tx, wasm_func, ast_func, ast_func.binary_ops.rights[index]);
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

fn codegen_definition(
    tx: Sender<Message>,
    wasm_func: Function,
    ast_func: &parser::Function,
    entity: usize,
) -> Function {
    let index = ast_func.indices[entity];
    let name_index = ast_func.definitions.names[index];
    assert_eq!(ast_func.kinds[name_index], parser::Kind::Symbol);
    let mut wasm_func =
        codegen_expression(tx, wasm_func, ast_func, ast_func.definitions.values[index]);
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
    assert_eq!(ast_func.kinds[entity], parser::Kind::Symbol);
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

fn codegen_function_call(
    tx: Sender<Message>,
    wasm_func: Function,
    ast_func: &parser::Function,
    entity: usize,
) -> Function {
    assert_eq!(ast_func.kinds[entity], parser::Kind::FunctionCall);
    let function_call = ast_func.indices[entity];
    let name = ast_func.function_calls.names[function_call];
    assert_eq!(ast_func.kinds[name], parser::Kind::Symbol);
    let mut wasm_func = ast_func.function_calls.parameters[function_call]
        .iter()
        .fold(wasm_func, |wasm_func, &parameter| {
            codegen_expression(tx.clone(), wasm_func, ast_func, parameter)
        });
    wasm_func.instructions.push(Instruction::Call);
    wasm_func.operand_kinds.push(vec![OperandKind::Symbol]);
    wasm_func.operands.push(vec![ast_func.indices[name]]);
    tx.send(Message::Spawn(
        ast_func.symbols[ast_func.indices[name]].clone(),
    ))
    .unwrap();
    wasm_func
}

fn codegen_expression(
    tx: Sender<Message>,
    wasm_func: Function,
    ast_func: &parser::Function,
    entity: usize,
) -> Function {
    match ast_func.kinds[entity] {
        parser::Kind::Int => codegen_int(wasm_func, ast_func, entity),
        parser::Kind::BinaryOp => codegen_binary_op(tx, wasm_func, ast_func, entity),
        parser::Kind::Definition => codegen_definition(tx, wasm_func, ast_func, entity),
        parser::Kind::Symbol => codegen_symbol(wasm_func, ast_func, entity),
        parser::Kind::FunctionCall => codegen_function_call(tx, wasm_func, ast_func, entity),
    }
}

fn codegen_function(tx: Sender<Message>, ast_func: &parser::Function) -> Function {
    let locals = ast_func
        .arguments
        .iter()
        .map(|&argument| format!("${}", ast_func.symbols[argument].clone()))
        .collect::<Vec<String>>();
    let name_to_local = ast_func.arguments.iter().enumerate().fold(
        HashMap::new(),
        |mut name_to_local, (i, &argument)| {
            name_to_local
                .try_insert(ast_func.symbols[argument].clone(), i)
                .unwrap();
            name_to_local
        },
    );
    let wasm_func = Function {
        name: ast_func.name,
        instructions: vec![],
        operand_kinds: vec![],
        operands: vec![],
        locals,
        name_to_local,
        symbols: vec![],
        ints: vec![],
        arguments: ast_func.arguments.len(),
    };
    let mut wasm_func = ast_func
        .expressions
        .iter()
        .fold(wasm_func, |wasm_func, &expression| {
            codegen_expression(tx.clone(), wasm_func, ast_func, expression)
        });
    wasm_func.symbols = ast_func.symbols.clone();
    wasm_func.ints = ast_func.ints.clone();
    tx.send(Message::Done).unwrap();
    wasm_func
}

pub fn codegen(ast: Ast) -> Wasm {
    let mut in_flight = 0;
    let mut wasm = Wasm {
        functions: vec![],
        name_to_function: HashMap::new(),
    };
    let (tx, rx) = mpsc::channel();
    tx.send(Message::Spawn(String::from("start"))).unwrap();
    loop {
        match rx.recv().unwrap() {
            Message::Spawn(name) => {
                if let None = wasm.name_to_function.get(&name) {
                    in_flight += 1;
                    let index = *ast.top_level.get(&name).unwrap();
                    let wasm_func = codegen_function(tx.clone(), &ast.functions[index]);
                    wasm.name_to_function
                        .try_insert(name, wasm.functions.len())
                        .unwrap();
                    wasm.functions.push(wasm_func);
                }
            }
            Message::Done => {
                in_flight -= 1;
                if in_flight == 0 {
                    break;
                }
            }
        }
    }
    wasm
}
