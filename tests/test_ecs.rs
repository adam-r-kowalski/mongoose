use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::AddAssign,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Entity(u64);

impl AddAssign<u64> for Entity {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

struct Storage<T> {
    data: Vec<T>,
    lookup: HashMap<Entity, usize>,
    inverse: Vec<Entity>,
}

impl<T> Storage<T> {
    fn new() -> Storage<T> {
        Storage {
            data: vec![],
            lookup: HashMap::new(),
            inverse: Vec::new(),
        }
    }

    fn set(&mut self, entity: Entity, component: T) {
        self.lookup.insert(entity, self.data.len());
        self.data.push(component);
        self.inverse.push(entity);
    }
}

struct ECS {
    components: HashMap<TypeId, Box<dyn Any>>,
    next_entity: Entity,
}

impl ECS {
    fn new() -> ECS {
        ECS {
            components: HashMap::new(),
            next_entity: Entity(0),
        }
    }

    fn create_entity(&mut self) -> Entity {
        let entity = self.next_entity;
        self.next_entity += 1;
        entity
    }

    fn set<C: 'static>(&mut self, entity: Entity, component: C) {
        self.components
            .entry(TypeId::of::<C>())
            .or_insert(Box::new(Storage::<C>::new()))
            .downcast_mut::<Storage<C>>()
            .unwrap()
            .set(entity, component);
    }
}

#[test]
fn test_any() {
    struct Name<'a>(&'a str);

    let mut ecs = ECS::new();
    let entity = ecs.create_entity();
    ecs.set(entity, Name("Joe"));
    assert_eq!(entity.get::<Name>(), Name("J"));
}
