use macro_grammar_impl::{Token, ast, Language};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Language)]
pub enum RustLanguage {}

#[repr(u16)]
#[derive(Debug, Token, PartialEq, Eq)]
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

#[ast]
pub struct Dot;

fn main() {}
