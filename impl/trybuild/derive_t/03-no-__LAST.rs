use macro_grammar_impl::Token;

#[derive(Token)]
#[repr(u16)]
pub enum SyntaxKind {
    #[punct = ","]
    Comma,
}

fn main() { }
