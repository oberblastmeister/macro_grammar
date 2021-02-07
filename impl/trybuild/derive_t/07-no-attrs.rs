use macro_grammar_impl::Token;

#[derive(Token)]
pub enum SyntaxKind {
    String,

    __LAST,
}

fn main() {}
