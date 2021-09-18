use std::{
    any::{Any, TypeId},
    collections::hash_map::{Entry, HashMap},
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
        match self.lookup.entry(entity) {
            Entry::Occupied(entry) => self.data[*entry.get()] = component,
            Entry::Vacant(entry) => {
                entry.insert(self.data.len());
                self.data.push(component);
                self.inverse.push(entity);
            }
        }
    }

    fn get<'a>(&'a self, entity: Entity) -> Option<&'a T> {
        self.lookup.get(&entity).map(|&index| &self.data[index])
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

    fn set<T: 'static>(&mut self, entity: Entity, component: T) {
        self.components
            .entry(TypeId::of::<T>())
            .or_insert(Box::new(Storage::<T>::new()))
            .downcast_mut::<Storage<T>>()
            .unwrap()
            .set(entity, component);
    }

    fn get<'a, T: 'static>(&'a self, entity: Entity) -> Option<&'a T> {
        self.components
            .get(&TypeId::of::<T>())
            .unwrap()
            .downcast_ref::<Storage<T>>()
            .unwrap()
            .get(entity)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Name<'a>(&'a str);

#[test]
fn test_get_and_set() {
    let mut ecs = ECS::new();
    let entity = ecs.create_entity();
    ecs.set(entity, Name("Joe"));
    assert_eq!(ecs.get::<Name>(entity).unwrap(), &Name("Joe"));
    ecs.set(entity, Name("Bob"));
    assert_eq!(ecs.get::<Name>(entity).unwrap(), &Name("Bob"));
}
