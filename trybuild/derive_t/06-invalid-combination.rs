#[derive(macro_grammar::T)]
pub enum SyntaxKind {
    #[token]
    #[kw]
    String,

    __LAST,
}

#[derive(macro_grammar::T)]
pub enum SyntaxKind2 {
    #[kw]
    #[punct = "."]
    Return,

    __LAST,
}

#[derive(macro_grammar::T)]
pub enum SyntaxKind3 {
    #[token]
    #[punct = "."]
    Return,

    __LAST,
}

fn main() {}
