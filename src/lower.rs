use std::{
    collections::HashMap,
    sync::mpsc::{self, Sender},
};

use crate::{
    parser::{parse, Ast},
    tokenizer::tokenize,
};

pub trait FileSystem {
    fn read_file<'a>(&'a self, file_name: &str) -> Option<&'a str>;
}

pub struct AstStore {
    asts: Vec<Ast>,
    names: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Function {}

#[derive(Debug, PartialEq)]
pub struct IR {
    pub functions: Vec<Function>,
}

struct Spawn<'a> {
    module: &'a str,
    function: &'a str,
    types: Vec<usize>,
}

struct Done {}

enum Message<'a> {
    Spawn(Spawn<'a>),
    Done(Done),
}

pub fn lower<F: FileSystem>(fs: F, file_name: &str) -> IR {
    let source = fs.read_file(file_name).unwrap();
    let tokens = tokenize(source);
    let ast = parse(tokens);
    let module = &file_name[..file_name.len() - 4];
    let _ast_store = AstStore {
        asts: vec![ast],
        names: vec![module.to_string()],
    };
    let ir = IR { functions: vec![] };
    let mut in_flight = 0;
    let (tx, rx) = mpsc::channel();
    tx.send(Message::Spawn(Spawn {
        module,
        function: "start",
        types: vec![],
    }))
    .unwrap();
    loop {
        match rx.recv().unwrap() {
            Message::Spawn(_spawn) => {
                in_flight += 1;
                let local_tx = tx.clone();
                rayon::scope(|s| {
                    s.spawn(move |_| {
                        local_tx.send(Message::Done(Done {})).unwrap();
                    });
                });
            }
            Message::Done(_done) => {
                in_flight -= 1;
                if in_flight == 0 {
                    break;
                }
            }
        }
    }
    ir
}
