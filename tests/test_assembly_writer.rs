use pretty_assertions::assert_eq;
use std::str;

use ra::{assembly_writer::write_assembly, codegen::codegen, lower::lower, parser::parse};

#[test]
fn write_assembly_literal() {
    let source = "(let start (Fn [] I32) (fn [] 0))";
    let ast = parse(source);
    let ir = lower(&ast);
    let x86 = codegen(&ir);
    let buffer = Vec::<u8>::new();
    let buffer = write_assembly(buffer, &x86).unwrap();
    let actual = str::from_utf8(&buffer).unwrap();
    let expected = "
    default rel
    global _main

    section .text

_main:
    push rbp
    mov rbp, rsp
    mov edi, 0
    mov rax, 201
    syscall";
    assert_eq!(actual, expected);
}
