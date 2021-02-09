use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DeriveInput, Result};

use common::bail_s;

pub fn derive(node: &DeriveInput) -> Result<TokenStream> {
    let token_stream = match node.data {
        Data::Enum(ref data_enum) => {
            if !data_enum.variants.is_empty() {
                bail_s!(node, "The enum must be empty");
            }

            let span = node.span();
            let proc_span = Span::call_site();
            let ident = &node.ident;

            let assert_syntax_kind_convert = quote_spanned! {proc_span=>
                struct __AssertSyntaxKindConvert where SyntaxKind:
                    From<u16>
                    + Into<u16>
                    + std::fmt::Debug;
            };

            let assert_traits = quote_spanned! {proc_span=>
                struct __AssertSyntaxKindTraits where #ident: Sized
                    + Clone
                    + Copy
                    + std::fmt::Debug
                    + Eq
                    + Ord
                    + std::hash::Hash;
            };

            quote! {
                #assert_syntax_kind_convert
                #assert_traits

                impl rowan::Language for #ident {
                    type Kind = SyntaxKind;

                    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
                        SyntaxKind::from(raw.0)
                    }

                    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
                        rowan::SyntaxKind(kind.into())
                    }
                }

                pub type SyntaxNode = rowan::SyntaxNode<#ident>;
                pub type SyntaxToken = rowan::SyntaxToken<#ident>;
                pub type SyntaxElement = rowan::SyntaxElement<#ident>;
                pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<#ident>;
                pub type SyntaxElementChildren = rowan::SyntaxElementChildren<#ident>;

                /// The main trait to go from untyped `SyntaxNode`  to a typed ast. The
                /// conversion itself has zero runtime cost: ast and syntax nodes have exactly
                /// the same representation: a pointer to the tree root and a pointer to the
                /// node itself.
                pub trait AstNode {
                    fn can_cast(kind: SyntaxKind) -> bool
                    where
                        Self: Sized;

                    fn cast(syntax: SyntaxNode) -> Option<Self>
                    where
                        Self: Sized;

                    fn syntax(&self) -> &SyntaxNode;
                }

                /// Like `AstNode`, but wraps tokens rather than interior nodes.
                pub trait AstToken {
                    fn can_cast(token: SyntaxKind) -> bool
                    where
                        Self: Sized;

                    fn cast(syntax: SyntaxToken) -> Option<Self>
                    where
                        Self: Sized;

                    fn syntax(&self) -> &SyntaxToken;

                    fn text(&self) -> smol_str::SmolStr {
                        self.syntax().text().into()
                    }
                }

                /// An iterator over `SyntaxNode` children of a particular AST type.
                #[derive(Debug, Clone)]
                pub struct AstChildren<N> {
                    inner: SyntaxNodeChildren,
                    ph: std::marker::PhantomData<N>,
                }

                impl<N> AstChildren<N> {
                    pub fn new(parent: &SyntaxNode) -> Self {
                        AstChildren {
                            inner: parent.children(),
                            ph: std::marker::PhantomData,
                        }
                    }
                }

                impl<N: AstNode> Iterator for AstChildren<N> {
                    type Item = N;
                    fn next(&mut self) -> Option<N> {
                        self.inner.find_map(N::cast)
                    }
                }

                pub mod lang {
                    pub use super::{SyntaxNode, SyntaxToken, SyntaxElement, SyntaxNodeChildren, SyntaxElementChildren, AstNode, AstToken, AstChildren};
                }
            }
        }
        _ => bail_s!(node, "The item must be an enum"),
    };

    Ok(token_stream)
}
