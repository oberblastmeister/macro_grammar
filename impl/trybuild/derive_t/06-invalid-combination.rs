use macro_grammar_impl::Token;

#[derive(Token)]
#[repr(u16)]
pub enum SyntaxKind {
    #[token]
    #[kw]
    String,

    __LAST,
}

#[derive(Token)]
#[repr(u16)]
pub enum SyntaxKind2 {
    #[kw]
    #[punct = "."]
    Return,

    __LAST,
}

#[derive(Token)]
#[repr(u16)]
pub enum SyntaxKind3 {
    #[token]
    #[punct = "."]
    Return,

    __LAST,
}

fn main() {}
