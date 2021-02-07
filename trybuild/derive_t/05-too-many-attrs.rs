#[derive(macro_grammar::T)]
pub enum SyntaxKind {
    #[token]
    #[token]
    String,

    __LAST,
}

#[derive(macro_grammar::T)]
pub enum SyntaxKind2 {
    #[punct = "."]
    #[punct = ".."]
    Dot,

    __LAST,
}

#[derive(macro_grammar::T)]
pub enum SyntaxKind3 {
    #[kw]
    #[kw]
    Return,

    __LAST,
}

fn main() { }
