mod ast;

use common::bail_s;
use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{format_ident, quote};
use syn::{DeriveInput, Result};

use ast::{Attrs, Enum, Input, Variant};

pub fn derive(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node)?;

    Ok(match input {
        Input::Enum(input) => input.gen_t()?,
        #[allow(unreachable_patterns)]
        _ => unreachable!(),
    })
}

impl Enum<'_> {
    fn gen_t(self) -> Result<TokenStream> {
        let macro_matches = self
            .variants
            .iter()
            .map(|variant| variant.gen_macro_match())
            .collect::<Result<Vec<_>>>()?;

        let variant_ident = self
            .variants
            .iter()
            .map(|Variant { ident, .. }| quote! {#ident});

        let tokens = quote! {
            #[macro_export]
            macro_rules! T {
                #([#macro_matches] => { $crate::SyntaxKind::#variant_ident };)*
            }
        };
        Ok(tokens)
    }
}

impl Variant<'_> {
    fn gen_macro_match(&self) -> Result<TokenStream> {
        if self.ident == "__LAST" {
            return Ok(quote! {});
        }

        if self.attrs.all_none() {
            bail_s!(self.original, "Each variant must have one attribute");
        }

        let token_stream = match &self.attrs {
            Attrs {
                punct: Some(lit),
                kw: None,
                token: None,
                ..
            } => match &**lit {
                "[" => quote! { '[' },
                "]" => quote! { ']' },
                "(" => quote! { '(' },
                ")" => quote! { ')' },
                "{" => quote! { '{' },
                "}" => quote! { '}' },
                _ => {
                    let cs = lit.chars().map(|c| Punct::new(c, Spacing::Joint));
                    quote! { #(#cs)* }
                }
            },
            Attrs {
                kw: Some(optional),
                punct: None,
                token: None,
                ..
            } => {
                let ident =
                    format_ident!("{}", optional.as_ref().unwrap_or(&self.ident.to_string()));
                quote! { #ident }
            }
            Attrs {
                token: Some(optional),
                punct: None,
                kw: None,
                ..
            } => {
                let ident =
                    format_ident!("{}", optional.as_ref().unwrap_or(&self.ident.to_string()));
                quote! { #ident }
            }
            _ => bail_s!(self.original, "Invalid combination of attributes"),
        };
        Ok(token_stream)
    }
}
