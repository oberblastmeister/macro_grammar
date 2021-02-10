use macro_grammar_impl::Token;

#[derive(Token)]
#[repr(u16)]
pub enum SyntaxKind {
    #[token]
    #[token]
    String,

    __LAST,
}

#[derive(Token)]
#[repr(u16)]
pub enum SyntaxKind2 {
    #[punct = "."]
    #[punct = ".."]
    Dot,

    __LAST,
}

#[derive(Token)]
#[repr(u16)]
pub enum SyntaxKind3 {
    #[kw]
    #[kw]
    Return,

    __LAST,
}

fn main() { }
