use crate::types::{Ir, X86};

pub fn codegen<'a>(ir: &'a Ir) -> X86<'a> {
    X86 { blocks: vec![] }
}
