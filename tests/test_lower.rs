use ra::{ir::lower, parser::parse};

#[test]
fn lower_start() {
    let source = "(let start (Fn [] I32) (fn [] 0))";
    let ast = parse(source);
    let _ = lower(&ast);
}
