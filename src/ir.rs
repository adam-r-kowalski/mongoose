use rayon::prelude::*;
use std::collections::HashMap;

use crate::types::{Ast, AstKind, BasicBlock, Calls, Entities, Environment, Ir, TopLevel};

impl<'a> Entities<'a> {
    fn new() -> Entities<'a> {
        Entities {
            literals: HashMap::new(),
        }
    }
}

impl Calls {
    fn new() -> Calls {
        Calls {
            func: vec![],
            args: vec![],
        }
    }
}

impl BasicBlock {
    fn new() -> BasicBlock {
        BasicBlock {
            kinds: vec![],
            indices: vec![],
            calls: Calls::new(),
            returns: vec![],
        }
    }
}

impl<'a> Environment<'a> {
    fn new() -> Environment<'a> {
        Environment {
            basic_blocks: vec![BasicBlock::new()],
            entities: Entities::new(),
        }
    }
}

fn ast_string<'a>(ast: &'a Ast, index: usize) -> &'a str {
    assert_eq!(ast.kinds[index], AstKind::Symbol);
    return ast.strings[ast.indices[index]];
}

fn lower_top_level<'a>(ast: &'a Ast, top_level: usize) -> TopLevel<'a> {
    assert_eq!(ast.kinds[top_level], AstKind::Parens);
    let children = &ast.children[ast.indices[top_level]];
    assert_eq!(children.len(), 4);
    assert_eq!(ast_string(ast, children[0]), "let");
    let name = ast_string(ast, children[1]);
    let env = Environment::new();
    TopLevel { name, env }
}

pub fn lower<'a>(ast: &'a Ast) -> Ir<'a> {
    let top_level_indices = &ast.top_level;
    let top_level: Vec<TopLevel> = top_level_indices
        .par_iter()
        .map(|&index| lower_top_level(ast, index))
        .collect();
    Ir { top_level }
}
