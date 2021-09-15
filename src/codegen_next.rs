use std::collections::HashMap;
use tokio::{runtime::Runtime, sync::mpsc};

use crate::{filesystem::FileSystem, parser, tokenizer::tokenize};

#[derive(Debug, PartialEq)]
pub struct Wasm {}

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

impl Function {
    fn new(ast_func: &parser::Function) -> Function {
        let mut locals = Vec::with_capacity(ast_func.arguments.len());
        let mut name_to_local = HashMap::new();
        for (i, &argument) in ast_func.arguments.iter().enumerate() {
            locals.push(format!("${}", ast_func.symbols[argument].clone()));
            name_to_local
                .try_insert(ast_func.symbols[argument].clone(), i)
                .unwrap();
        }
        Function {
            name: ast_func.name,
            instructions: vec![],
            operand_kinds: vec![],
            operands: vec![],
            locals,
            name_to_local,
            symbols: vec![],
            ints: vec![],
            arguments: ast_func.arguments.len(),
            next_label: 0,
        }
    }
}

fn codegen_function_call(
    dispatch: mpsc::Sender<CentralMessage>,
    mut wasm_func: Function,
    ast_func: &parser::Function,
    entity: usize,
) -> Function {
    assert_eq!(ast_func.kinds[entity], parser::Kind::FunctionCall);
    let function_call = ast_func.indices[entity];
    let name = ast_func.function_calls.names[function_call];
    assert_eq!(ast_func.kinds[name], parser::Kind::Symbol);
    for &parameter in &ast_func.function_calls.parameters[function_call] {
        wasm_func = codegen_expression(dispatch.clone(), wasm_func, ast_func, parameter);
    }
    wasm_func.instructions.push(Instruction::Call);
    wasm_func.operand_kinds.push(vec![OperandKind::Symbol]);
    wasm_func.operands.push(vec![ast_func.indices[name]]);
    wasm_func
}

fn codegen_expression(
    dispatch: mpsc::Sender<CentralMessage>,
    wasm_func: Function,
    ast_func: &parser::Function,
    entity: usize,
) -> Function {
    match ast_func.kinds[entity] {
        // parser::Kind::Int => codegen_int(wasm_func, ast_func, entity),
        // parser::Kind::BinaryOp => codegen_binary_op(tx, wasm_func, ast_func, entity),
        // parser::Kind::Assign => codegen_assignment(tx, wasm_func, ast_func, entity),
        // parser::Kind::Symbol => codegen_symbol(wasm_func, ast_func, entity),
        parser::Kind::FunctionCall => codegen_function_call(dispatch, wasm_func, ast_func, entity),
        // parser::Kind::If => codegen_if(tx, wasm_func, ast_func, entity),
        // parser::Kind::While => codegen_while(tx, wasm_func, ast_func, entity),
        // parser::Kind::Grouping => codegen_grouping(tx, wasm_func, ast_func, entity),
        kind => panic!("codegen expression {:?} not implemented", kind),
    }
}

fn codegen_function(
    dispatch: mpsc::Sender<CentralMessage>,
    ast_func: &parser::Function,
) -> Function {
    let mut wasm_func = Function::new(ast_func);
    for &expression in &ast_func.expressions {
        wasm_func = codegen_expression(dispatch.clone(), wasm_func, ast_func, expression);
    }
    wasm_func.symbols = ast_func.symbols.clone();
    wasm_func.ints = ast_func.ints.clone();
    wasm_func
}

#[derive(Debug)]
pub struct Modules {
    paths: Vec<HashMap<String, usize>>,
    leafs: Vec<HashMap<String, usize>>,
    channels: Vec<mpsc::Sender<ModuleMessage>>,
}

impl Modules {
    pub fn new() -> Modules {
        Modules {
            paths: vec![HashMap::new()],
            leafs: vec![HashMap::new()],
            channels: vec![],
        }
    }
}

#[derive(Debug)]
enum ModuleMessage {
    Call {
        function: String,
        dispatch: mpsc::Sender<CentralMessage>,
    },
}

#[derive(Debug)]
enum CentralMessage {
    Call { path: Vec<String>, function: String },
    Done,
}

async fn module_handle(
    modules: &mut Modules,
    path: Vec<String>,
    fs: &impl FileSystem,
) -> mpsc::Sender<ModuleMessage> {
    let mut index = 0;
    let last_index = path.len() - 1;
    for p in &path[..last_index] {
        let len = modules.paths.len();
        index = *modules.paths[index].entry(p.to_string()).or_insert(len);
        if index == len {
            modules.paths.push(HashMap::new());
            modules.leafs.push(HashMap::new());
        }
    }
    let len = modules.channels.len();
    let index = *modules.leafs[index]
        .entry(path[last_index].to_string())
        .or_insert(len);
    if index == len {
        let (tx, mut rx) = mpsc::channel(32);
        modules.channels.push(tx);
        let source = fs.read_file(path).await.unwrap();
        let tokens = tokenize(&source);
        let ast = parser::parse(tokens);
        let mut functions = vec![];
        let mut name_to_function = HashMap::new();
        tokio::spawn(async move {
            match rx.recv().await.unwrap() {
                ModuleMessage::Call { function, dispatch } => {
                    let index = *ast.top_level.get(&function).unwrap();
                    let ast_func = &ast.functions[index];
                    let i = functions.len();
                    functions.push(codegen_function(dispatch.clone(), ast_func));
                    name_to_function.try_insert(function, i).unwrap();
                    dispatch.send(CentralMessage::Done).await.unwrap();
                }
            }
        });
    }
    modules.channels[index].clone()
}

pub fn codegen(fs: &impl FileSystem, module: &str) -> Wasm {
    let wasm = Wasm {};
    Runtime::new().unwrap().block_on(async {
        let (tx, mut rx) = mpsc::channel(32);
        let mut modules = Modules::new();
        let mut in_flight = 0;
        tx.send(CentralMessage::Call {
            path: vec![module.to_string()],
            function: "start".to_string(),
        })
        .await
        .unwrap();
        loop {
            match rx.recv().await.unwrap() {
                CentralMessage::Call { path, function } => {
                    let module = module_handle(&mut modules, path, fs).await;
                    module
                        .send(ModuleMessage::Call {
                            function,
                            dispatch: tx.clone(),
                        })
                        .await
                        .unwrap();
                    in_flight += 1;
                }
                CentralMessage::Done => {
                    in_flight -= 1;
                    if in_flight == 0 {
                        break;
                    }
                }
            }
        }
    });
    wasm
}
