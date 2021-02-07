use std::marker::PhantomData;

use rowan::{Language, SyntaxKind, SyntaxNode, SyntaxNodeChildren, SyntaxToken};
use smol_str::SmolStr;

/// The main trait to go from untyped `SyntaxNode`  to a typed ast. The
/// conversion itself has zero runtime cost: ast and syntax nodes have exactly
/// the same representation: a pointer to the tree root and a pointer to the
/// node itself.
pub trait AstNode {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast<L: Language>(syntax: SyntaxNode<L>) -> Option<Self>
    where
        Self: Sized;

    fn syntax<L: Language>(&self) -> &SyntaxNode<L>;
}

/// Like `AstNode`, but wraps tokens rather than interior nodes.
pub trait AstToken {
    fn can_cast(token: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast<L: Language>(syntax: SyntaxToken<L>) -> Option<Self>
    where
        Self: Sized;

    fn syntax<L: Language>(&self) -> &SyntaxToken<L>;

    fn text<L: Language>(&self) -> SmolStr {
        self.syntax::<L>().text().into()
    }
}

/// An iterator over `SyntaxNode` children of a particular AST type.
#[derive(Debug, Clone)]
pub struct AstChildren<N, L: Language> {
    inner: SyntaxNodeChildren<L>,
    ph: PhantomData<N>,
}

impl<N, L: Language> AstChildren<N, L> {
    fn new(parent: &SyntaxNode<L>) -> Self {
        AstChildren {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<N: AstNode, L: Language> Iterator for AstChildren<N, L> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}
