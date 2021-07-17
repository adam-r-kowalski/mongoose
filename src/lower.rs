use rayon::prelude::*;
use std::collections::HashMap;

use crate::types::{
    ast::{self, Ast},
    ir::{self, Block, Calls, Entities, Entity, Environment, Ir, Kind, TopLevel},
};

impl<'a> Entities<'a> {
    fn new() -> Entities<'a> {
        Entities {
            name_to_entity: HashMap::new(),
            literals: HashMap::new(),
            next_entity: ir::Entity(0),
        }
    }
}

impl Calls {
    fn new() -> Calls {
        Calls {
            functions: vec![],
            arguments: vec![],
            returns: vec![],
        }
    }
}

impl Block {
    fn new() -> Block {
        Block {
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
            blocks: vec![Block::new()],
            entities: Entities::new(),
            current_block: 0,
        }
    }
}

fn ast_symbol<'a>(ast: &'a Ast, ast::Entity(index): ast::Entity) -> &'a str {
    assert_eq!(ast.kinds[index], ast::Kind::Symbol);
    return ast.strings[ast.indices[index]];
}

fn ast_int<'a>(ast: &'a Ast, ast::Entity(index): ast::Entity) -> &'a str {
    assert_eq!(ast.kinds[index], ast::Kind::Int);
    return ast.strings[ast.indices[index]];
}

fn ast_parens<'a>(ast: &'a Ast, ast::Entity(index): ast::Entity) -> &'a [ast::Entity] {
    assert_eq!(ast.kinds[index], ast::Kind::Parens);
    return &ast.children[ast.indices[index]];
}

fn ast_brackets<'a>(ast: &'a Ast, ast::Entity(index): ast::Entity) -> &'a [ast::Entity] {
    assert_eq!(ast.kinds[index], ast::Kind::Brackets);
    return &ast.children[ast.indices[index]];
}

fn fresh_entity(mut env: Environment) -> (Environment, Entity) {
    let entity = env.entities.next_entity;
    let Entity(index) = entity;
    env.entities.next_entity = Entity(index + 1);
    (env, entity)
}

fn lower_symbol<'a>(
    env: Environment<'a>,
    ast: &'a Ast,
    entity: ast::Entity,
) -> (Environment<'a>, Entity) {
    let symbol = ast_symbol(ast, entity);
    match env.entities.name_to_entity.get(symbol) {
        Some(&entity) => (env, entity),
        None => {
            let (mut env, entity) = fresh_entity(env);
            env.entities.literals.try_insert(entity, symbol).unwrap();
            env.entities
                .name_to_entity
                .try_insert(symbol, entity)
                .unwrap();
            (env, entity)
        }
    }
}

fn lower_int<'a>(
    env: Environment<'a>,
    ast: &'a Ast,
    entity: ast::Entity,
) -> (Environment<'a>, Entity) {
    let int = ast_int(ast, entity);
    let (mut env, entity) = fresh_entity(env);
    env.entities.literals.try_insert(entity, int).unwrap();
    (env, entity)
}

fn lower_lambda<'a>(
    env: Environment<'a>,
    ast: &'a Ast,
    children: &'a [ast::Entity],
) -> (Environment<'a>, Entity) {
    assert!(children.len() > 1);
    let (env, args) = ast_brackets(ast, children[0]).iter().fold(
        (env, Vec::<Entity>::new()),
        |(env, mut args), &entity| {
            let (env, arg) = lower_symbol(env, ast, entity);
            args.push(arg);
            (env, args)
        },
    );
    assert_eq!(args.len(), 0);
    let sentinel = Entity(0);
    let (mut env, entity) = children[1..]
        .iter()
        .fold((env, sentinel), |(env, _), &entity| {
            lower_expression(env, ast, entity)
        });
    assert_ne!(entity, sentinel);
    let block = &mut env.blocks[env.current_block];
    block.kinds.push(Kind::Return);
    block.indices.push(block.returns.len());
    block.returns.push(entity);
    (env, entity)
}

fn lower_function_call<'a>(
    env: Environment<'a>,
    ast: &'a Ast,
    func: Entity,
    children: &'a [ast::Entity],
) -> (Environment<'a>, Entity) {
    let (env, args) =
        children
            .iter()
            .fold((env, Vec::<Entity>::new()), |(env, mut args), &entity| {
                let (env, arg) = lower_expression(env, ast, entity);
                args.push(arg);
                (env, args)
            });
    let (mut env, entity) = fresh_entity(env);
    let block = &mut env.blocks[env.current_block];
    block.kinds.push(Kind::Call);
    block.indices.push(block.calls.functions.len());
    block.calls.functions.push(func);
    block.calls.arguments.push(args);
    block.calls.returns.push(entity);
    (env, entity)
}

fn lower_call<'a>(
    env: Environment<'a>,
    ast: &'a Ast,
    entity: ast::Entity,
) -> (Environment<'a>, Entity) {
    let children = ast_parens(ast, entity);
    assert!(children.len() > 0);
    let (env, func) = lower_expression(env, ast, children[0]);
    let children = &children[1..];
    match env.entities.literals.get(&func) {
        Some(&"fn") => lower_lambda(env, ast, children),
        _ => lower_function_call(env, ast, func, children),
    }
}

fn lower_array<'a>(
    env: Environment<'a>,
    ast: &'a Ast,
    entity: ast::Entity,
) -> (Environment<'a>, Entity) {
    assert_eq!(ast_brackets(ast, entity).len(), 0);
    let (env, entity) = fresh_entity(env);
    (env, entity)
}

fn lower_expression<'a>(
    env: Environment<'a>,
    ast: &'a Ast,
    entity: ast::Entity,
) -> (Environment<'a>, Entity) {
    let ast::Entity(index) = entity;
    match &ast.kinds[index] {
        ast::Kind::Symbol => lower_symbol(env, ast, entity),
        ast::Kind::Parens => lower_call(env, ast, entity),
        ast::Kind::Brackets => lower_array(env, ast, entity),
        ast::Kind::Int => lower_int(env, ast, entity),
        kind => panic!("lower expression unimplemented for {:?}", kind),
    }
}

fn lower_top_level<'a>(ast: &'a Ast, entity: ast::Entity) -> TopLevel<'a> {
    let children = ast_parens(ast, entity);
    assert_eq!(children.len(), 4);
    assert_eq!(ast_symbol(ast, children[0]), "let");
    let name = ast_symbol(ast, children[1]);
    let env = Environment::new();
    let type_block = env.current_block;
    let (mut env, type_entity) = lower_expression(env, ast, children[2]);
    env.current_block = env.blocks.len();
    env.blocks.push(Block::new());
    let value_block = env.current_block;
    let (env, value_entity) = lower_expression(env, ast, children[3]);
    TopLevel {
        name,
        type_entity,
        type_block,
        value_entity,
        value_block,
        environment: env,
    }
}

pub fn lower<'a>(ast: &'a Ast) -> Ir<'a> {
    let top_level_entities = &ast.top_level;
    let top_level: Vec<TopLevel> = top_level_entities
        .par_iter()
        .map(|&entity| lower_top_level(ast, entity))
        .collect();
    Ir { top_level }
}
