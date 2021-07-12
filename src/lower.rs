use rayon::prelude::*;
use std::collections::HashMap;

use crate::types::{
    Ast, AstEntity, AstKind, BasicBlock, Calls, Entities, Environment, ExpressionKind, Ir,
    IrEntity, TopLevel,
};

impl<'a> Entities<'a> {
    fn new() -> Entities<'a> {
        Entities {
            name_to_entity: HashMap::new(),
            literals: HashMap::new(),
            next_entity: IrEntity(0),
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

fn ast_symbol<'a>(ast: &'a Ast, AstEntity(index): AstEntity) -> &'a str {
    assert_eq!(ast.kinds[index], AstKind::Symbol);
    return ast.strings[ast.indices[index]];
}

fn ast_int<'a>(ast: &'a Ast, AstEntity(index): AstEntity) -> &'a str {
    assert_eq!(ast.kinds[index], AstKind::Int);
    return ast.strings[ast.indices[index]];
}

fn ast_parens<'a>(ast: &'a Ast, AstEntity(index): AstEntity) -> &'a [AstEntity] {
    assert_eq!(ast.kinds[index], AstKind::Parens);
    return &ast.children[ast.indices[index]];
}

fn ast_brackets<'a>(ast: &'a Ast, AstEntity(index): AstEntity) -> &'a [AstEntity] {
    assert_eq!(ast.kinds[index], AstKind::Brackets);
    return &ast.children[ast.indices[index]];
}

fn fresh_entity(mut env: Environment) -> (Environment, IrEntity) {
    let entity = env.entities.next_entity;
    let IrEntity(index) = entity;
    env.entities.next_entity = IrEntity(index + 1);
    (env, entity)
}

fn lower_symbol<'a>(
    env: Environment<'a>,
    ast: &'a Ast,
    entity: AstEntity,
) -> (Environment<'a>, IrEntity) {
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
    entity: AstEntity,
) -> (Environment<'a>, IrEntity) {
    let int = ast_int(ast, entity);
    let (mut env, entity) = fresh_entity(env);
    env.entities.literals.try_insert(entity, int).unwrap();
    (env, entity)
}

fn lower_lambda<'a>(
    env: Environment<'a>,
    ast: &'a Ast,
    children: &'a [AstEntity],
) -> (Environment<'a>, IrEntity) {
    assert!(children.len() > 1);
    let (env, args) = ast_brackets(ast, children[0]).iter().fold(
        (env, Vec::<IrEntity>::new()),
        |(env, mut args), &entity| {
            let (env, arg) = lower_symbol(env, ast, entity);
            args.push(arg);
            (env, args)
        },
    );
    assert_eq!(args.len(), 0);
    let sentinel = IrEntity(0);
    let (mut env, entity) = children[1..]
        .iter()
        .fold((env, sentinel), |(env, _), &entity| {
            lower_expression(env, ast, entity)
        });
    assert_ne!(entity, sentinel);
    let basic_block = &mut env.basic_blocks[env.current_basic_block];
    basic_block.kinds.push(ExpressionKind::Return);
    basic_block.indices.push(basic_block.returns.len());
    basic_block.returns.push(entity);
    (env, entity)
}

fn lower_function_call<'a>(
    env: Environment<'a>,
    ast: &'a Ast,
    func: IrEntity,
    children: &'a [AstEntity],
) -> (Environment<'a>, IrEntity) {
    let (env, args) =
        children
            .iter()
            .fold((env, Vec::<IrEntity>::new()), |(env, mut args), &entity| {
                let (env, arg) = lower_expression(env, ast, entity);
                args.push(arg);
                (env, args)
            });
    let (mut env, entity) = fresh_entity(env);
    let basic_block = &mut env.basic_blocks[env.current_basic_block];
    basic_block.kinds.push(ExpressionKind::Call);
    basic_block.indices.push(basic_block.calls.functions.len());
    basic_block.calls.functions.push(func);
    basic_block.calls.arguments.push(args);
    basic_block.calls.returns.push(entity);
    (env, entity)
}

fn lower_call<'a>(
    env: Environment<'a>,
    ast: &'a Ast,
    entity: AstEntity,
) -> (Environment<'a>, IrEntity) {
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
    entity: AstEntity,
) -> (Environment<'a>, IrEntity) {
    assert_eq!(ast_brackets(ast, entity).len(), 0);
    let (env, entity) = fresh_entity(env);
    (env, entity)
}

fn lower_expression<'a>(
    env: Environment<'a>,
    ast: &'a Ast,
    entity: AstEntity,
) -> (Environment<'a>, IrEntity) {
    let AstEntity(index) = entity;
    match &ast.kinds[index] {
        AstKind::Symbol => lower_symbol(env, ast, entity),
        AstKind::Parens => lower_call(env, ast, entity),
        AstKind::Brackets => lower_array(env, ast, entity),
        AstKind::Int => lower_int(env, ast, entity),
        kind => panic!("lower expression unimplemented for {:?}", kind),
    }
}

fn lower_top_level<'a>(ast: &'a Ast, entity: AstEntity) -> TopLevel<'a> {
    let children = ast_parens(ast, entity);
    assert_eq!(children.len(), 4);
    assert_eq!(ast_symbol(ast, children[0]), "let");
    let name = ast_symbol(ast, children[1]);
    let env = Environment::new();
    let (mut env, type_entity) = lower_expression(env, ast, children[2]);
    env.current_basic_block = env.basic_blocks.len();
    env.basic_blocks.push(BasicBlock::new());
    let (env, value_entity) = lower_expression(env, ast, children[3]);
    TopLevel {
        name,
        type_entity,
        value_entity,
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
