use crate::parser::Ast;

#[derive(Debug, PartialEq)]
pub struct Wasm {}

pub fn codegen(_ast: Ast) -> Wasm {
    Wasm {}
}
