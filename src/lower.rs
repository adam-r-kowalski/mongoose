use crate::{
    parser::{parse, Ast},
    tokenizer::tokenize,
};

#[derive(Debug)]
pub struct Ir {
    pub asts: Vec<Ast>,
    pub file_names: Vec<String>,
}

pub trait FileSystem {
    fn read_file<'a>(&'a self, file_name: &str) -> Option<&'a str>;
}

pub fn lower<F: FileSystem>(fs: F, file_name: &str) -> Ir {
    let source = fs.read_file(file_name).unwrap();
    let tokens = tokenize(source);
    let ast = parse(tokens);
    Ir {
        asts: vec![ast],
        file_names: vec![file_name.to_string()],
    }
}
