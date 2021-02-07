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

    #[punct = "("]
    LParen,

    #[punct = ")"]
    RParen,

    #[punct = "{"]
    RBrace,

    #[punct = "}"]
    LBrace,

    #[punct = "["]
    LBracket,

    #[punct = "]"]
    RBracket,

    __LAST,
}

fn main() {
    T![.];
    T![,];
    T![Async];
    T![String];
    T!['('];
    T![')'];
    T!['{'];
    T!['}'];
    T!['['];
    T![']'];
}
