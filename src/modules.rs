use std::collections::HashMap;
use tokio::sync::mpsc;

use crate::{filesystem::FileSystem, parser::parse, tokenizer::tokenize};

#[derive(Debug)]
pub struct Call {
    pub function: String,
    pub argument_types: Vec<usize>,
}

#[derive(Debug)]
pub enum Message {
    Call(Call),
    Done,
}

#[derive(Debug)]
pub struct Modules<F: FileSystem> {
    paths: Vec<HashMap<String, usize>>,
    leafs: Vec<HashMap<String, usize>>,
    channels: Vec<mpsc::Sender<Message>>,
    file_system: F,
}

impl<F: FileSystem> Modules<F> {
    pub fn new(file_system: F) -> Modules<F> {
        Modules {
            paths: vec![HashMap::new()],
            leafs: vec![HashMap::new()],
            channels: vec![],
            file_system,
        }
    }

    pub async fn module(&mut self, path: Vec<String>) -> mpsc::Sender<Message> {
        let mut index = 0;
        let last_index = path.len() - 1;
        for p in &path[..last_index] {
            let len = self.paths.len();
            index = *self.paths[index].entry(p.to_string()).or_insert(len);
            if index == len {
                self.paths.push(HashMap::new());
                self.leafs.push(HashMap::new());
            }
        }
        let len = self.channels.len();
        let index = *self.leafs[index]
            .entry(path[last_index].to_string())
            .or_insert(len);
        if index == len {
            let (tx, mut rx) = mpsc::channel(32);
            self.channels.push(tx);
            let source = self.file_system.read_file(path).await.unwrap();
            let tokens = tokenize(&source);
            let ast = parse(tokens);
            tokio::spawn(async move {
                loop {
                    match rx.recv().await.unwrap() {
                        Message::Call(call) => {
                            let index = *ast.top_level.get(&call.function).unwrap();
                            let ast_func = &ast.functions[index];
                            println!("{:?}", ast_func);
                        }
                        Message::Done => break,
                    }
                }
            });
        }
        self.channels[index].clone()
    }
}
