use macro_grammar_impl::Token;

#[derive(Token)]
pub enum SyntaxKind {
    #[punct = ","]
    Comma,
}

fn main() { }
