use compiler::ecs::ECS;

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

#[test]
fn test_iterate() {
    let mut ecs = ECS::new();
    let entity = ecs.create_entity();
    ecs.set(entity, Name("Joe"));
    let entity = ecs.create_entity();
    ecs.set(entity, Name("Bob"));
    let mut names = vec![];
    for (_, name) in ecs.iterate::<Name>() {
        names.push(name);
    }
    assert_eq!(names, vec![&Name("Joe"), &Name("Bob")]);
}
