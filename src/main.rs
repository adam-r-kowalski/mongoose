use std::{env, fs::File, io::Read};

use wasmer::{imports, Instance, Module, Store};

use ra::{codegen::codegen, parser::parse, tokenizer::tokenize, writer::write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1]).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let tokens = tokenize(&contents);
    let ast = parse(tokens);
    let wasm = codegen(ast);
    // let file = File::create("start.wasm").unwrap();
    let buffer = write(Vec::<u8>::new(), wasm).unwrap();
    let store = Store::default();
    let module = Module::new(&store, &buffer).unwrap();
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object).unwrap();
    let start = instance.exports.get_function("_start").unwrap();
    let result = start.call(&[]).unwrap();
    println!("{:?}", result[0]);
}
