use std::fmt::{Error, Write};

use rayon::prelude::*;

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

pub fn write_block(mut code: String, func: &Function, i: usize) -> Result<String, Error> {
    assert_eq!(func.operand_kinds[i], vec![OperandKind::Label]);
    let operands = &func.operands[i];
    assert_eq!(operands.len(), 1);
    write!(code, "\n    block $.label.{}", operands[0])?;
    Ok(code)
}

pub fn write_loop(mut code: String, func: &Function, i: usize) -> Result<String, Error> {
    assert_eq!(func.operand_kinds[i], vec![OperandKind::Label]);
    let operands = &func.operands[i];
    assert_eq!(operands.len(), 1);
    write!(code, "\n    loop $.label.{}", operands[0])?;
    Ok(code)
}

pub fn write_end(mut code: String, func: &Function, i: usize) -> Result<String, Error> {
    code.push_str("\n    end");
    let operand_kinds = &func.operand_kinds[i];
    if operand_kinds.len() > 0 {
        assert_eq!(operand_kinds, &[OperandKind::Label]);
        let operands = &func.operands[i];
        assert_eq!(operands.len(), 1);
        write!(code, " $.label.{}", operands[0])?;
    }
    Ok(code)
}

pub fn write_br_if(mut code: String, func: &Function, i: usize) -> Result<String, Error> {
    assert_eq!(func.operand_kinds[i], vec![OperandKind::Label]);
    let operands = &func.operands[i];
    assert_eq!(operands.len(), 1);
    write!(code, "\n    br_if $.label.{}", operands[0])?;
    Ok(code)
}

pub fn write_br(mut code: String, func: &Function, i: usize) -> Result<String, Error> {
    assert_eq!(func.operand_kinds[i], vec![OperandKind::Label]);
    let operands = &func.operands[i];
    assert_eq!(operands.len(), 1);
    write!(code, "\n    br $.label.{}", operands[0])?;
    Ok(code)
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
                Instruction::I64And => write_str(code, "i64.and"),
                Instruction::I64Xor => write_str(code, "i64.xor"),
                Instruction::I64Or => write_str(code, "i64.or"),
                Instruction::I64Shl => write_str(code, "i64.shl"),
                Instruction::I64ShrS => write_str(code, "i64.shr_s"),
                Instruction::I64Eq => write_str(code, "i64.eq"),
                Instruction::I64Neq => write_str(code, "i64.ne"),
                Instruction::I64LtS => write_str(code, "i64.lt_s"),
                Instruction::I64GtS => write_str(code, "i64.gt_s"),
                Instruction::I64LeS => write_str(code, "i64.le_s"),
                Instruction::I64GeS => write_str(code, "i64.ge_s"),
                Instruction::I32Eqz => write_str(code, "i32.eqz"),
                Instruction::SetLocal => write_set_local(code, &func, i),
                Instruction::GetLocal => write_get_local(code, &func, i),
                Instruction::Call => write_call(code, &func, i),
                Instruction::If => write_str(code, "if (result i64)"),
                Instruction::Block => write_block(code, &func, i),
                Instruction::Loop => write_loop(code, &func, i),
                Instruction::Else => write_str(code, "else"),
                Instruction::End => write_end(code, &func, i),
                Instruction::BrIf => write_br_if(code, &func, i),
                Instruction::Br => write_br(code, &func, i),
            })?;
    code.push(')');
    Ok(code)
}

pub fn write(wasm: &Wasm) -> String {
    let mut code = String::new();
    code.push_str("\n(module");
    let mut code = wasm
        .functions
        .par_iter()
        .map(|function| write_function(String::new(), function).unwrap())
        .collect::<Vec<String>>()
        .iter()
        .fold(code, |mut code, fragment| {
            code.push_str(&fragment);
            code
        });
    code.push_str("\n\n  (export \"_start\" (func $start)))\n");
    code
}
