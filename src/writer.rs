use std::fmt::{Error, Write};

use crate::codegen::{Function, Instruction, OperandKind, Wasm};

pub fn write_i64_const(mut code: String, func: &Function, i: usize) -> Result<String, Error> {
    assert_eq!(func.operand_kinds[i], vec![OperandKind::IntLiteral]);
    let operands = &func.operands[i];
    assert_eq!(operands.len(), 1);
    let literal = &func.ints[operands[0]];
    write!(code, "\n    (i64.const {})", literal)?;
    Ok(code)
}

pub fn write_set_local(mut code: String, func: &Function, i: usize) -> Result<String, Error> {
    assert_eq!(func.operand_kinds[i], vec![OperandKind::Local]);
    let operands = &func.operands[i];
    assert_eq!(operands.len(), 1);
    let local = &func.locals[operands[0]];
    write!(code, "\n    (set_local {})", local)?;
    Ok(code)
}

pub fn write_get_local(mut code: String, func: &Function, i: usize) -> Result<String, Error> {
    assert_eq!(func.operand_kinds[i], vec![OperandKind::Local]);
    let operands = &func.operands[i];
    assert_eq!(operands.len(), 1);
    let local = &func.locals[operands[0]];
    write!(code, "\n    (get_local {})", local)?;
    Ok(code)
}

pub fn write_call(mut code: String, func: &Function, i: usize) -> Result<String, Error> {
    assert_eq!(func.operand_kinds[i], vec![OperandKind::Symbol]);
    let operands = &func.operands[i];
    assert_eq!(operands.len(), 1);
    let symbol = &func.symbols[operands[0]];
    write!(code, "\n    (call ${})", symbol)?;
    Ok(code)
}

pub fn write_str(mut code: String, text: &str) -> Result<String, Error> {
    write!(code, "\n    {}", text)?;
    Ok(code)
}

fn write_arguments(code: String, func: &Function) -> Result<String, Error> {
    func.locals[..func.arguments]
        .iter()
        .try_fold(code, |mut code, local| {
            write!(code, " (param {} i64)", local)?;
            Ok(code)
        })
}

fn write_locals(code: String, func: &Function) -> Result<String, Error> {
    func.locals[func.arguments..]
        .iter()
        .try_fold(code, |mut code, local| {
            write!(code, "\n    (local {} i64)", local)?;
            Ok(code)
        })
}

fn write_function(mut code: String, func: &Function) -> Result<String, Error> {
    write!(code, "\n\n  (func ${}", func.symbols[func.name])?;
    let mut code = write_arguments(code, &func)?;
    code.push_str(" (result i64)");
    let code = write_locals(code, &func)?;
    let mut code =
        func.instructions
            .iter()
            .enumerate()
            .try_fold(code, |code, (i, instruction)| match instruction {
                Instruction::I64Const => write_i64_const(code, &func, i),
                Instruction::I64Add => write_str(code, "i64.add"),
                Instruction::I64Sub => write_str(code, "i64.sub"),
                Instruction::I64Mul => write_str(code, "i64.mul"),
                Instruction::I64DivS => write_str(code, "i64.div_s"),
                Instruction::I64RemS => write_str(code, "i64.rem_s"),
                Instruction::I64Eq => write_str(code, "i64.eq"),
                Instruction::I64Shl => write_str(code, "i64.shl"),
                Instruction::I64LtS => write_str(code, "i64.lt_s"),
                Instruction::SetLocal => write_set_local(code, &func, i),
                Instruction::GetLocal => write_get_local(code, &func, i),
                Instruction::Call => write_call(code, &func, i),
                Instruction::If => write_str(code, "if (result i64)"),
                Instruction::Else => write_str(code, "else"),
                Instruction::End => write_str(code, "end"),
            })?;
    code.push(')');
    Ok(code)
}

pub fn write(wasm: Wasm) -> String {
    let mut code = String::new();
    code.push_str("(module");
    let mut code = wasm
        .functions
        .iter()
        .try_fold(code, |code, function| write_function(code, function))
        .unwrap();
    code.push_str("\n\n  (export \"_start\" (func $start)))");
    code
}
