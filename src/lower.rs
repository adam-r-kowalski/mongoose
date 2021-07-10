use rayon::prelude::*;
use std::collections::HashMap;

use crate::types::{
    Ast, AstEntity, AstKind, BasicBlock, Calls, Entities, Environment, Ir, IrEntity, TopLevel,
};

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
            current_basic_block: 0,
        }
    }
}

fn ast_string<'a>(ast: &'a Ast, AstEntity(index): AstEntity) -> &'a str {
    assert_eq!(ast.kinds[index], AstKind::Symbol);
    return ast.strings[ast.indices[index]];
}

fn lower_call<'a>(
    env: Environment<'a>,
    ast: &'a Ast,
    AstEntity(index): AstEntity,
) -> (Environment<'a>, IrEntity) {
    panic!("lower call");
}

fn lower_expression<'a>(
    env: Environment<'a>,
    ast: &'a Ast,
    entity: AstEntity,
) -> (Environment<'a>, IrEntity) {
    let AstEntity(index) = entity;
    match ast.kinds[index] {
        AstKind::Parens => lower_call(env, ast, entity),
        _ => panic!("unimplemented"),
    }
}

fn lower_top_level<'a>(ast: &'a Ast, AstEntity(index): AstEntity) -> TopLevel<'a> {
    assert_eq!(ast.kinds[index], AstKind::Parens);
    let children = &ast.children[ast.indices[index]];
    assert_eq!(children.len(), 4);
    assert_eq!(ast_string(ast, children[0]), "let");
    let name = ast_string(ast, children[1]);
    let env = Environment::new();
    let (env, type_entity) = lower_expression(env, ast, children[2]);
    TopLevel { name, env }
}

pub fn lower<'a>(ast: &'a Ast) -> Ir<'a> {
    let top_level_entities = &ast.top_level;
    let top_level: Vec<TopLevel> = top_level_entities
        .par_iter()
        .map(|&entity| lower_top_level(ast, entity))
        .collect();
    Ir { top_level }
}
