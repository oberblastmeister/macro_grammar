use macro_grammar_impl::Token;

#[derive(Token)]
#[repr(u16)]
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

    #[token]
    Ident,

    __LAST,
}

fn main() {
    T![.];
    T![,];
    T![async];
    T![string];
    T!['('];
    T![')'];
    T!['{'];
    T!['}'];
    T!['['];
    T![']'];
    T![ident];
    T![comment];
}
