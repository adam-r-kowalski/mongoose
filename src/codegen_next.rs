use std::{
    collections::HashMap,
    sync::mpsc::{self, Sender},
};

use crate::{
    parser::{parse, Ast},
    tokenizer::tokenize,
};

pub trait FileSystem {
    fn read_file(&self, path: Vec<&str>) -> Option<String>;
}

pub struct Context<'a> {
    asts: Vec<Ast>,
    roots: HashMap<&'a str, usize>,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    I64Const,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64RemS,
    I64And,
    I64Xor,
    I64Or,
    I64Eq,
    I64Neq,
    I64Shl,
    I64ShrS,
    I64LtS,
    I64LeS,
    I64GtS,
    I64GeS,
    I32Eqz,
    SetLocal,
    GetLocal,
    Call,
    If,
    Else,
    Block,
    Loop,
    End,
    BrIf,
    Br,
}

#[derive(Debug, PartialEq)]
pub enum OperandKind {
    IntLiteral,
    Local,
    Symbol,
    Label,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: usize,
    pub instructions: Vec<Instruction>,
    pub operand_kinds: Vec<Vec<OperandKind>>,
    pub operands: Vec<Vec<usize>>,
    pub locals: Vec<String>,
    pub name_to_local: HashMap<String, usize>,
    pub symbols: Vec<String>,
    pub ints: Vec<String>,
    pub arguments: usize,
    pub next_label: usize,
}

#[derive(Debug, PartialEq)]
pub struct Wasm {
    pub functions: Vec<Function>,
}

#[derive(Debug)]
struct Spawn<'a> {
    path: Vec<&'a str>,
    function: &'a str,
    types: Vec<usize>,
}

struct Done {}

enum Message<'a> {
    Spawn(Spawn<'a>),
    Done(Done),
}

pub fn codegen(fs: impl FileSystem, module: &str) -> Wasm {
    let context = Context {
        asts: vec![],
        roots: HashMap::new(),
    };
    let wasm = Wasm { functions: vec![] };
    let mut in_flight = 0;
    let (tx, rx) = mpsc::channel();
    tx.send(Message::Spawn(Spawn {
        path: vec![module],
        function: "start",
        types: vec![],
    }))
    .unwrap();
    loop {
        match rx.recv().unwrap() {
            Message::Spawn(spawn) => {
                let source = fs.read_file(spawn.path);
                println!("\n\n\nsource\n\n\n{:?}\n\n\n", source);
                // let tokens = tokenize(source);
                // let ast = parse(tokens);
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
    wasm
}
