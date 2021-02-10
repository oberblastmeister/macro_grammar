use macro_grammar_impl::Language;

#[derive(Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    Variant,
}

impl From<u16> for SyntaxKind {
    fn from(value: u16) -> SyntaxKind {
        SyntaxKind::Variant
    }
}

impl From<SyntaxKind> for u16 {
    fn from(value: SyntaxKind) -> u16 {
        0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Language)]
pub enum RustLanguage {}

fn main() {}
