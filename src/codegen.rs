use crate::parser::{self, Ast};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    I32Const,
    I32Add,
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

fn codegen_int(mut wasm: Wasm, ast: &Ast, entity: parser::Entity) -> Wasm {
    wasm.function.instructions.push(Instruction::I32Const);
    wasm.function
        .operand_kinds
        .push(vec![OperandKind::IntLiteral]);
    wasm.function.operands.push(vec![ast.indices[entity.0]]);
    wasm
}

fn codegen_binary_operator(wasm: Wasm, ast: &Ast, entity: parser::Entity) -> Wasm {
    let index = ast.indices[entity.0];
    let wasm = codegen_expression(wasm, ast, ast.binary_operators.lefts[index]);
    let mut wasm = codegen_expression(wasm, ast, ast.binary_operators.rights[index]);
    wasm.function.instructions.push(Instruction::I32Add);
    wasm.function.operand_kinds.push(vec![]);
    wasm.function.operands.push(vec![]);
    wasm
}

fn codegen_expression(wasm: Wasm, ast: &Ast, entity: parser::Entity) -> Wasm {
    match ast.kinds[entity.0] {
        parser::Kind::Int => codegen_int(wasm, ast, entity),
        parser::Kind::BinaryOperator => codegen_binary_operator(wasm, ast, entity),
        kind => panic!("codegen expression for kind {:?} not implemented", kind),
    }
}

pub fn codegen(ast: Ast) -> Wasm {
    let start = ast.top_level.get("start").unwrap();
    let start_index = ast.indices[start.0];
    let return_type = ast.functions.return_types[start_index];
    assert_eq!(ast.kinds[return_type.0], parser::Kind::Symbol);
    assert_eq!(ast.symbols[ast.indices[return_type.0]], "i64");
    let wasm = Wasm {
        function: Function {
            instructions: vec![],
            operand_kinds: vec![],
            operands: vec![],
        },
        symbols: vec![],
        ints: vec![],
    };
    let body = ast.functions.bodies[start_index];
    let mut wasm = codegen_expression(wasm, &ast, body);
    wasm.symbols = ast.symbols;
    wasm.ints = ast.ints;
    wasm
}
