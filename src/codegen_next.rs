use async_trait::async_trait;
use tokio::{runtime::Runtime, sync::mpsc};

use crate::{parser::parse, tokenizer::tokenize};

#[derive(Debug, PartialEq)]
pub struct Wasm {}

#[async_trait]
pub trait FileSystem {
    async fn read_file(&self, path: Vec<String>) -> Option<String>;
}

#[derive(Debug)]
struct Spawn {
    path: Vec<String>,
    function: String,
}

#[derive(Debug)]
enum Message {
    Spawn(Spawn),
    Done,
}

pub fn codegen(fs: &impl FileSystem, module: &str) -> Wasm {
    Runtime::new().unwrap().block_on(async {
        let (tx, mut rx) = mpsc::channel(32);
        let wasm = Wasm {};
        let mut in_flight = 0;
        tx.send(Message::Spawn(Spawn {
            path: vec![module.to_string()],
            function: "start".to_string(),
        }))
        .await
        .unwrap();
        loop {
            match rx.recv().await.unwrap() {
                Message::Spawn(spawn) => {
                    let source = fs.read_file(spawn.path).await.unwrap();
                    let tokens = tokenize(&source);
                    let ast = parse(tokens);
                    in_flight += 1;
                    let tx2 = tx.clone();
                    tokio::spawn(async move {
                        tx2.send(Message::Done).await.unwrap();
                    });
                }
                Message::Done => {
                    println!("new spawn done");
                    in_flight -= 1;
                    if in_flight == 0 {
                        break;
                    }
                }
            }
        }
        wasm
    })
}
