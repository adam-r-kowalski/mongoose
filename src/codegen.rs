use crate::types::{
    ir::{self, Ir},
    x86::{self, Instruction, Kind, Register, TopLevel, X86},
};

impl x86::Block {
    fn new() -> x86::Block {
        x86::Block {
            instructions: vec![],
            operand_kinds: vec![],
            operands: vec![],
        }
    }
}

pub fn codegen<'a>(ir: &'a Ir) -> X86<'a> {
    let &ir::Entity(start) = ir.entities.name_to_entity.get("start").unwrap();
    let ir_top_level = &ir.top_level[start];
    let ir_block = &ir_top_level.environment.blocks[ir_top_level.value_block];
    let x86_block = x86::Block::new();
    let literals = Vec::<&'a str>::new();
    let (x86_block, literals) = ir_block.kinds.iter().enumerate().fold(
        (x86_block, literals),
        |(mut x86_block, mut literals), (index, kind)| match kind {
            ir::Kind::Return => {
                let return_entity = &ir_block.returns[ir_block.indices[index]];
                x86_block.instructions.push(Instruction::Mov);
                x86_block
                    .operand_kinds
                    .push(vec![Kind::Register, Kind::Literal]);
                x86_block
                    .operands
                    .push(vec![Register::Edi as usize, literals.len()]);
                literals.push(
                    ir_top_level
                        .environment
                        .entities
                        .literals
                        .get(return_entity)
                        .unwrap(),
                );
                (x86_block, literals)
            }
            kind => panic!("kind {:?} is not yet supported in codegen", kind),
        },
    );
    X86 {
        top_level: vec![TopLevel {
            blocks: vec![x86_block],
            literals,
        }],
    }
}
