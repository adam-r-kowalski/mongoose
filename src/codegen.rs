use crate::types::{
    ir::{self, Ir},
    x86::X86,
};

pub fn codegen<'a>(ir: &'a Ir) -> X86<'a> {
    let &ir::Entity(start) = ir.entities.name_to_entity.get("start").unwrap();
    let top_level = &ir.top_level[start];
    let block = &top_level.environment.blocks[top_level.value_block];
    let indices = block
        .kinds
        .iter()
        .enumerate()
        .map(|(index, kind)| match kind {
            ir::Kind::Return => index,
            kind => panic!("kind {} is not yet supported in codegen", kind),
        })
        .collect::<Vec<usize>>();
    X86 { top_level: vec![] }
}
