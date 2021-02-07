use macro_grammar::Token;

#[derive(Token)]
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

    __LAST,
}

fn main() {
    T![.];
    T![,];
    T![Async];
    T![String];
}
