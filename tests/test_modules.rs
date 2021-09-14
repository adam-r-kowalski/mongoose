use tokio::runtime::Runtime;

use compiler::{
    filesystem::MockFileSystem,
    modules::{Call, Message, Modules},
};

#[test]
fn test_module() {
    let mut fs = MockFileSystem::new();
    fs.new_file(vec!["foo"], "fn start() -> i64: baz()");
    let mut modules = Modules::new(fs);
    Runtime::new().unwrap().block_on(async {
        let tx = modules.module(vec!["foo".to_string()]).await;
        tx.send(Message::Call(Call {
            function: "start".to_string(),
            argument_types: vec![],
        }))
        .await
        .unwrap();
        tx.send(Message::Call(Call {
            function: "start".to_string(),
            argument_types: vec![],
        }))
        .await
        .unwrap();
        tx.send(Message::Done).await.unwrap();
    });
}
