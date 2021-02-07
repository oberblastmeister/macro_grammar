use macro_grammar_impl::Token;

#[derive(Token)]
pub enum SyntaxKind {
    #[token]
    #[token]
    String,

    __LAST,
}

#[derive(Token)]
pub enum SyntaxKind2 {
    #[punct = "."]
    #[punct = ".."]
    Dot,

    __LAST,
}

#[derive(Token)]
pub enum SyntaxKind3 {
    #[kw]
    #[kw]
    Return,

    __LAST,
}

fn main() { }
