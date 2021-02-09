mod ast;

use common::bail_s;
use heck::SnakeCase;
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
        let ident = self.ident;

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
            impl From<u16> for #ident {
                fn from(d: u16) -> #ident {
                    assert!(d <= (SyntaxKind::__LAST as u16));
                    unsafe { std::mem::transmute::<u16, #ident>(d) }
                }
            }

            impl From<#ident> for u16 {
                fn from(k: #ident) -> u16 {
                    k as u16
                }
            }

            #[macro_export]
            macro_rules! T {
                #([#macro_matches] => { $crate::#ident::#variant_ident };)*
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
                misc: None,
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
                misc: None,
                ..
            } => {
                let ident_s = optional
                    .as_ref()
                    .unwrap_or(&self.ident.to_string())
                    .to_snake_case();
                let ident = format_ident!("{}", ident_s);
                quote! { #ident }
            }
            Attrs {
                token: Some(optional),
                punct: None,
                kw: None,
                misc: None,
                ..
            } => {
                let ident_s = optional
                    .as_ref()
                    .unwrap_or(&self.ident.to_string())
                    .to_snake_case();
                let ident = format_ident!("{}", ident_s);
                quote! { #ident }
            }
            Attrs {
                misc: Some(misc),
                token: None,
                punct: None,
                kw: None,
                ..
            } => {
                let ident_s = misc.to_snake_case();
                let ident = format_ident!("{}", ident_s);
                quote! { #ident }
            }
            _ => bail_s!(self.original, "Invalid combination of attributes"),
        };
        Ok(token_stream)
    }
}
