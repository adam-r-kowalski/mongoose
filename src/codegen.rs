use crate::types::{ir::Ir, x86::X86};

pub fn codegen<'a>(ir: &'a Ir) -> X86<'a> {
    X86 { blocks: vec![] }
}
