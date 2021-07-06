use pretty_assertions::assert_eq;

use ra::parser::{parse, Ast, Kind};

#[test]
fn parse_int() {
    let ast = parse("10 -20 350");
    assert_eq!(
        ast,
        Ast {
            index: vec![0, 1, 2],
            kind: vec![Kind::Int, Kind::Int, Kind::Int],
            strings: vec!["10", "-20", "350"],
            children: vec![],
            top_level: vec![0, 1, 2],
        }
    )
}

#[test]
fn parse_float() {
    let ast = parse("10.5 -20.2 .350");
    assert_eq!(
        ast,
        Ast {
            index: vec![0, 1, 2],
            kind: vec![Kind::Float, Kind::Float, Kind::Float],
            strings: vec!["10.5", "-20.2", ".350"],
            children: vec![],
            top_level: vec![0, 1, 2],
        }
    )
}

#[test]
fn parse_symbol() {
    let ast = parse("foo bar baz .");
    assert_eq!(
        ast,
        Ast {
            index: vec![0, 1, 2, 3],
            kind: vec![Kind::Symbol, Kind::Symbol, Kind::Symbol, Kind::Symbol],
            strings: vec!["foo", "bar", "baz", "."],
            children: vec![],
            top_level: vec![0, 1, 2, 3],
        }
    )
}

#[test]
fn parse_keyword() {
    let ast = parse(":foo :bar :baz");
    assert_eq!(
        ast,
        Ast {
            index: vec![0, 1, 2],
            kind: vec![Kind::Keyword, Kind::Keyword, Kind::Keyword],
            strings: vec![":foo", ":bar", ":baz"],
            children: vec![],
            top_level: vec![0, 1, 2],
        }
    )
}

#[test]
fn parse_brackets() {
    let ast = parse("[[a b] [c d]]");
    assert_eq!(
        ast,
        Ast {
            index: vec![0, 1, 0, 2, 3, 1, 2],
            kind: vec![
                Kind::Symbol,
                Kind::Symbol,
                Kind::Brackets,
                Kind::Symbol,
                Kind::Symbol,
                Kind::Brackets,
                Kind::Brackets
            ],
            strings: vec!["a", "b", "c", "d"],
            children: vec![vec![0, 1], vec![3, 4], vec![2, 5]],
            top_level: vec![6],
        }
    )
}

#[test]
fn parse_parens() {
    let ast = parse("(add (mul 1 2) (div 3 4))");
    assert_eq!(
        ast,
        Ast {
            index: vec![0, 1, 2, 3, 0, 4, 5, 6, 1, 2],
            kind: vec![
                Kind::Symbol,
                Kind::Symbol,
                Kind::Int,
                Kind::Int,
                Kind::Parens,
                Kind::Symbol,
                Kind::Int,
                Kind::Int,
                Kind::Parens,
                Kind::Parens,
            ],
            strings: vec!["add", "mul", "1", "2", "div", "3", "4"],
            children: vec![vec![1, 2, 3], vec![5, 6, 7], vec![0, 4, 8]],
            top_level: vec![9],
        }
    )
}
