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
    println!("name to local {:?}!!!", wasm_func.name_to_local);
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
    let name = ast_func.function_calls.names[ast_func.indices[entity]];
    assert_eq!(ast_func.kinds[name], parser::Kind::Symbol);
    println!("spawning {}!!!", ast_func.symbols[ast_func.indices[name]]);
    println!("TODO[ADAM]: Add function arguments to name to locals");
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
        .map(|&argument| {
            assert_eq!(ast_func.kinds[argument], parser::Kind::Symbol);
            ast_func.symbols[ast_func.indices[argument]].clone()
        })
        .collect::<Vec<String>>();
    let name_to_local =
        locals
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut name_to_local, (local, name)| {
                name_to_local.try_insert(name.clone(), local).unwrap();
                name_to_local
            });
    let wasm_func = Function {
        name: ast_func.name,
        instructions: vec![],
        operand_kinds: vec![],
        operands: vec![],
        locals,
        name_to_local,
        symbols: vec![],
        ints: vec![],
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
    let mut wasm = Wasm { functions: vec![] };
    let (tx, rx) = mpsc::channel();
    tx.send(Message::Spawn(String::from("start"))).unwrap();
    loop {
        match rx.recv().unwrap() {
            Message::Spawn(name) => {
                in_flight += 1;
                let index = *ast.top_level.get(&name).unwrap();
                let wasm_func = codegen_function(tx.clone(), &ast.functions[index]);
                wasm.functions.push(wasm_func);
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
