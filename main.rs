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

    #[punct = "{"]
    LParen,
}

fn main() {
    T![.];
    T![,];
    T![Async];
    T![String];
}
