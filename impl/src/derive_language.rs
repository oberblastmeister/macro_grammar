use proc_macro2::TokenStream;
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
            let ident = &node.ident;

            let assert_syntax_kind_convert = quote_spanned! {span=>
                struct __AssertSyntaxKindConvert where SyntaxKind:
                    From<u16>
                    + Into<u16>
                    + std::fmt::Debug;
            };

            let assert_traits = quote_spanned! {span=>
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

                fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
                    SyntaxKind::from(raw.0)
                }

                fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
                    rowan::SyntaxKind(kind.into())
                }

                pub type SyntaxNode = rowan::SyntaxNode<#ident>;
                pub type SyntaxToken = rowan::SyntaxToken<#ident>;
                pub type SyntaxElement = rowan::SyntaxElement<#ident>;
                pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<#ident>;
                pub type SyntaxElementChildren = rowan::SyntaxElementChildren<#ident>;
            }
        }
        _ => bail_s!(node, "The item must be an enum"),
    };

    Ok(token_stream)
}
