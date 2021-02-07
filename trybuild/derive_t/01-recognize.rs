#[derive(macro_grammar::T)]
pub enum SyntaxKind {
    #[token]
    String,

    #[punct = "."]
    Dot,

    #[punct = ","]
    Comma,

    #[kw]
    Async,

    #[token]
    Comment,

    __LAST,
}

fn main() {}
