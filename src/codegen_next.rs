use tokio::{runtime::Runtime, sync::mpsc};

// use crate::{
//     parser::{parse, Ast},
//     tokenizer::tokenize,
// };

#[derive(Debug, PartialEq)]
pub struct Wasm {}

pub trait FileSystem {
    fn read_file(&self, path: Vec<&str>) -> Option<String>;
}

#[derive(Debug)]
enum Message {
    Spawn,
    Done,
}

pub fn codegen(_fs: &impl FileSystem, _module: &str) -> Wasm {
    Runtime::new().unwrap().block_on(async {
        let (tx, mut rx) = mpsc::channel(32);
        let wasm = Wasm {};
        let mut in_flight = 0;

        tx.send(Message::Spawn).await.unwrap();

        loop {
            match rx.recv().await.unwrap() {
                Message::Spawn => {
                    println!("new spawn message");
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
