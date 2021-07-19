use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::iter::FromIterator;

use ra::{
    lower::lower,
    parser::parse,
    types::ir::{Block, Calls, Entities, Entity, Environment, Ir, Kind, TopLevel},
};

#[test]
fn lower_trivial() {
    let source = "(let start (Fn [] I32) (fn [] 0))";
    let ast = parse(source);
    let ir = lower(&ast);
    assert_eq!(
        ir,
        Ir {
            top_level: vec![TopLevel {
                name: "start",
                environment: Environment {
                    blocks: vec![
                        Block {
                            kinds: vec![Kind::Call],
                            indices: vec![0],
                            calls: Calls {
                                functions: vec![Entity(0)],
                                arguments: vec![vec![Entity(1), Entity(2)],],
                                returns: vec![Entity(3)],
                            },
                            returns: vec![],
                        },
                        Block {
                            kinds: vec![Kind::Return],
                            indices: vec![0],
                            calls: Calls {
                                functions: vec![],
                                arguments: vec![],
                                returns: vec![],
                            },
                            returns: vec![Entity(5)],
                        },
                    ],
                    entities: Entities {
                        name_to_entity: HashMap::from_iter([
                            ("Fn", Entity(0)),
                            ("I32", Entity(2)),
                            ("fn", Entity(4))
                        ]),
                        literals: HashMap::from_iter([
                            (Entity(0), "Fn"),
                            (Entity(2), "I32"),
                            (Entity(4), "fn"),
                            (Entity(5), "0")
                        ]),
                        next_entity: Entity(6)
                    },
                    current_block: 1
                },
                type_entity: Entity(3),
                type_block: 0,
                value_entity: Entity(5),
                value_block: 1,
            }],
            entities: Entities {
                name_to_entity: HashMap::from_iter([("start", Entity(0)),]),
                literals: HashMap::from_iter([(Entity(0), "start"),]),
                next_entity: Entity(1)
            }
        },
    )
}
