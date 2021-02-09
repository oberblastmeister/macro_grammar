mod ast;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{DeriveInput, Result};

use ast::{Enum, Input, Struct};

pub fn transform(attr: TokenStream, node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node)?;

    Ok(match input {
        Input::Enum(input) => input.transform()?,
        Input::Struct(input) => input.transform()?,
        #[allow(unreachable_patterns)]
        _ => unreachable!(),
    })
}

impl Enum<'_> {
    fn transform(self) -> Result<TokenStream> {
        todo!()
    }
}

impl Struct<'_> {
    fn transform(self) -> Result<TokenStream> {
        let proc_span = Span::call_site();

        let stream = match self {
            Struct::Named(strukt) => {
                todo!()
            }

            Struct::Unnamed(strukt) => {
                let kind = strukt.kind;
                let ident = strukt.ident;

                quote! {
                    pub struct #ident(SyntaxToken);

                    impl AstToken for #ident {
                        fn can_cast(kind: SyntaxKind) -> bool { kind == #kind }

                        fn cast(syntax: SyntaxToken) -> Option<Self> {
                            if #ident::can_cast(syntax.kind()) {
                                Some(Self(syntax))
                            } else {
                                None
                            }
                        }

                        fn syntax(&self) -> &SyntaxToken { &self.0 }
                    }
                }
            }
        };
        Ok(stream)
    }
}
