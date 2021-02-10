use macro_grammar_impl::Token;

#[derive(Token)]
#[repr(u16)]
pub enum SyntaxKind {
    String,

    __LAST,
}

fn main() {}
