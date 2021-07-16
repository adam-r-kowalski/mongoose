use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::iter::FromIterator;

use ra::{
    lower::lower,
    parser::parse,
    types::{IrBlock, Calls, Entities, Environment, ExpressionKind, Ir, IrEntity, TopLevel},
};

#[test]
fn lower_start_explicitly_typed() {
    let source = "(let start (Fn [] I32) (fn [] 0))";
    let ast = parse(source);
    let ir = lower(&ast);
    assert_eq!(
        ir,
        Ir {
            top_level: vec![TopLevel {
                name: "start",
                environment: Environment {
                    basic_blocks: vec![
                        IrBlock {
                            kinds: vec![ExpressionKind::Call],
                            indices: vec![0],
                            calls: Calls {
                                functions: vec![IrEntity(0)],
                                arguments: vec![vec![IrEntity(1), IrEntity(2)],],
                                returns: vec![IrEntity(3)],
                            },
                            returns: vec![],
                        },
                        IrBlock {
                            kinds: vec![ExpressionKind::Return],
                            indices: vec![0],
                            calls: Calls {
                                functions: vec![],
                                arguments: vec![],
                                returns: vec![],
                            },
                            returns: vec![IrEntity(5)],
                        },
                    ],
                    entities: Entities {
                        name_to_entity: HashMap::from_iter([
                            ("Fn", IrEntity(0)),
                            ("I32", IrEntity(2)),
                            ("fn", IrEntity(4))
                        ]),
                        literals: HashMap::from_iter([
                            (IrEntity(0), "Fn"),
                            (IrEntity(2), "I32"),
                            (IrEntity(4), "fn"),
                            (IrEntity(5), "0")
                        ]),
                        next_entity: IrEntity(6)
                    },
                    current_basic_block: 1
                },
                type_entity: IrEntity(3),
                type_basic_block: 0,
                value_entity: IrEntity(5),
                value_basic_block: 1,
            }]
        }
    )
}
