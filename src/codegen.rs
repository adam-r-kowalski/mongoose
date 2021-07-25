use crate::parser::{Ast, self};

#[derive(Debug, PartialEq)]
pub struct Wasm {}

pub fn codegen(ast: Ast) -> Wasm {
    let start = ast.top_level.get("start").unwrap();
    let start_index = ast.indices[start.0];
    let return_type = ast.functions.return_types[start_index];
    assert_eq!(ast.kinds[return_type.0], parser::Kind::Symbol);
    assert_eq!(ast.symbols[ast.indices[return_type.0]], "i64");
    let body = ast.functions.bodies[start_index];
    assert_eq!(ast.kinds[body.0], parser::Kind::Int);
    Wasm {}
}
