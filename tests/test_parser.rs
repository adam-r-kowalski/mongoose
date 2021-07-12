use pretty_assertions::assert_eq;

use ra::{
    parser::parse,
    types::{Ast, AstEntity, AstKind},
};

#[test]
fn parse_int() {
    let ast = parse("10 -20 350");
    assert_eq!(
        ast,
        Ast {
            indices: vec![0, 1, 2],
            kinds: vec![AstKind::Int, AstKind::Int, AstKind::Int],
            strings: vec!["10", "-20", "350"],
            children: vec![],
            top_level: vec![AstEntity(0), AstEntity(1), AstEntity(2)],
        }
    )
}

#[test]
fn parse_float() {
    let ast = parse("10.5 -20.2 .350");
    assert_eq!(
        ast,
        Ast {
            indices: vec![0, 1, 2],
            kinds: vec![AstKind::Float, AstKind::Float, AstKind::Float],
            strings: vec!["10.5", "-20.2", ".350"],
            children: vec![],
            top_level: vec![AstEntity(0), AstEntity(1), AstEntity(2)],
        }
    )
}

#[test]
fn parse_symbol() {
    let ast = parse("foo bar baz . -");
    assert_eq!(
        ast,
        Ast {
            indices: vec![0, 1, 2, 3, 4],
            kinds: vec![
                AstKind::Symbol,
                AstKind::Symbol,
                AstKind::Symbol,
                AstKind::Symbol,
                AstKind::Symbol
            ],
            strings: vec!["foo", "bar", "baz", ".", "-"],
            children: vec![],
            top_level: vec![
                AstEntity(0),
                AstEntity(1),
                AstEntity(2),
                AstEntity(3),
                AstEntity(4)
            ],
        }
    )
}

#[test]
fn parse_keyword() {
    let ast = parse(":foo :bar :baz");
    assert_eq!(
        ast,
        Ast {
            indices: vec![0, 1, 2],
            kinds: vec![AstKind::Keyword, AstKind::Keyword, AstKind::Keyword],
            strings: vec![":foo", ":bar", ":baz"],
            children: vec![],
            top_level: vec![AstEntity(0), AstEntity(1), AstEntity(2)],
        }
    )
}

#[test]
fn parse_brackets() {
    let ast = parse("[[a b] [c d]]");
    assert_eq!(
        ast,
        Ast {
            indices: vec![0, 1, 0, 2, 3, 1, 2],
            kinds: vec![
                AstKind::Symbol,
                AstKind::Symbol,
                AstKind::Brackets,
                AstKind::Symbol,
                AstKind::Symbol,
                AstKind::Brackets,
                AstKind::Brackets
            ],
            strings: vec!["a", "b", "c", "d"],
            children: vec![
                vec![AstEntity(0), AstEntity(1)],
                vec![AstEntity(3), AstEntity(4)],
                vec![AstEntity(2), AstEntity(5)]
            ],
            top_level: vec![AstEntity(6)],
        }
    )
}

#[test]
fn parse_parens() {
    let ast = parse("(add (mul 1 2) (div 3 4))");
    assert_eq!(
        ast,
        Ast {
            indices: vec![0, 1, 2, 3, 0, 4, 5, 6, 1, 2],
            kinds: vec![
                AstKind::Symbol,
                AstKind::Symbol,
                AstKind::Int,
                AstKind::Int,
                AstKind::Parens,
                AstKind::Symbol,
                AstKind::Int,
                AstKind::Int,
                AstKind::Parens,
                AstKind::Parens,
            ],
            strings: vec!["add", "mul", "1", "2", "div", "3", "4"],
            children: vec![
                vec![AstEntity(1), AstEntity(2), AstEntity(3)],
                vec![AstEntity(5), AstEntity(6), AstEntity(7)],
                vec![AstEntity(0), AstEntity(4), AstEntity(8)]
            ],
            top_level: vec![AstEntity(9)],
        }
    )
}
