use std::{env, fs::File, io::Read};

use wasmer::{imports, Instance, Module, Store};

use yeti::{codegen::codegen, parser::parse, tokenizer::tokenize, writer::write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1]).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let tokens = tokenize(&contents);
    let ast = parse(tokens);
    let wasm = codegen(ast);
    match args.get(2) {
        Some(s) if s == "--emit-wasm" => {
            let file = File::create(&args[3]).unwrap();
            write(file, wasm).unwrap();
        }
        _ => {
            let code = write(Vec::<u8>::new(), wasm).unwrap();
            let store = Store::default();
            let module = Module::new(&store, &code).unwrap();
            let import_object = imports! {};
            let instance = Instance::new(&module, &import_object).unwrap();
            let start = instance.exports.get_function("_start").unwrap();
            let result = start.call(&[]).unwrap();
            println!("{:?}", result[0]);
        }
    }
}
