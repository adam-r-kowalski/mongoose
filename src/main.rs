use std::{env, fs::File, io::Read};

use ra::{codegen::codegen, parser::parse, tokenizer::tokenize, writer::write};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args = {:?}", args);
    let mut file = File::open(&args[1]).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let tokens = tokenize(&contents);
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let file = File::create("start.wasm").unwrap();
    write(file, wasm).unwrap();
}
