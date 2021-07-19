use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Kind {
    Call,
    Return,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Entity(pub usize);

#[derive(Debug, PartialEq)]
pub struct Entities<'a> {
    pub name_to_entity: HashMap<&'a str, Entity>,
    pub literals: HashMap<Entity, &'a str>,
    pub next_entity: Entity,
}

#[derive(Debug, PartialEq)]
pub struct Calls {
    pub functions: Vec<Entity>,
    pub arguments: Vec<Vec<Entity>>,
    pub returns: Vec<Entity>,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub kinds: Vec<Kind>,
    pub indices: Vec<usize>,
    pub calls: Calls,
    pub returns: Vec<Entity>,
}

#[derive(Debug, PartialEq)]
pub struct Environment<'a> {
    pub blocks: Vec<Block>,
    pub entities: Entities<'a>,
    pub current_block: usize,
}

#[derive(Debug, PartialEq)]
pub struct TopLevel<'a> {
    pub name: &'a str,
    pub environment: Environment<'a>,
    pub type_entity: Entity,
    pub type_block: usize,
    pub value_entity: Entity,
    pub value_block: usize,
}

#[derive(Debug, PartialEq)]
pub struct Ir<'a> {
    pub top_level: Vec<TopLevel<'a>>,
    pub entities: Entities<'a>,
}
