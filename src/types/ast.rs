#[derive(Debug, PartialEq)]
pub enum Kind {
    Int,
    Float,
    Symbol,
    Keyword,
    Brackets,
    Parens,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Entity(pub usize);

#[derive(Debug, PartialEq)]
pub struct Ast<'a> {
    pub indices: Vec<usize>,
    pub kinds: Vec<Kind>,
    pub strings: Vec<&'a str>,
    pub children: Vec<Vec<Entity>>,
    pub top_level: Vec<Entity>,
}
