mod valid;

use common::bail_s;
use proc_macro2::Span;
use std::marker::PhantomData;

use syn::{
    spanned::Spanned, Attribute, Data, DataEnum, DeriveInput, Error, Ident, Lit, LitStr, Meta,
    Path, Result,
};

pub enum Input<'a> {
    Enum(Enum<'a>),
}

impl<'a> Input<'a> {
    pub fn from_syn(node: &'a DeriveInput) -> Result<Input<'a>> {
        match &node.data {
            Data::Enum(data) => Enum::from_syn(node, data).map(Input::Enum),
            Data::Struct(_) => Err(Error::new_spanned(node, "Struct types are not supported")),
            Data::Union(_) => Err(Error::new_spanned(node, "Union types are not supported")),
        }
    }
}

pub struct Enum<'a> {
    pub original: &'a DeriveInput,
    pub variants: Vec<Variant<'a>>,
}

impl<'a> Enum<'a> {
    fn from_syn(node: &'a DeriveInput, data: &'a DataEnum) -> Result<Enum<'a>> {
        let variants = data
            .variants
            .iter()
            .map(|v| Variant::from_syn(&v))
            .collect::<Result<Vec<_>>>()?;
        Ok(Enum {
            original: node,
            variants,
        })
    }
}

pub struct Variant<'a> {
    pub original: &'a syn::Variant,
    pub ident: &'a Ident,
    pub attrs: Attrs<'a>,
}

impl<'a> Variant<'a> {
    fn from_syn(variant: &'a syn::Variant) -> Result<Variant<'a>> {
        match variant.fields {
            syn::Fields::Unit => (),
            _ => return Err(Error::new_spanned(variant, "Must be a unit variant")),
        }

        if variant.discriminant.is_some() {
            bail_s!(variant, "Cannot have explicit descriminant");
        }

        let attrs = Attrs::get(&variant.attrs)?;

        let res = Variant {
            original: variant,
            ident: &variant.ident,
            attrs,
        };
        Ok(res)
    }
}

#[derive(Default)]
pub struct Attrs<'a> {
    pub punct: Option<String>,
    pub kw: Option<Option<String>>,
    pub token: Option<Option<String>>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Attrs<'a> {
    pub fn get(input: &'a [Attribute]) -> Result<Attrs<'a>> {
        const LIT_STR_ERR: &str = "The value must be a string literal";

        let mut attrs = Attrs::default();

        for attr in input {
            let meta = attr.parse_meta()?;
            match meta {
                Meta::NameValue(name_value) => {
                    let key = strip_path(&name_value.path)?;

                    match &*key.to_string() {
                        "punct" => {
                            let value = get_lit_str(name_value.lit)
                                .ok_or_else(|| Error::new_spanned(attr, LIT_STR_ERR))?;
                            attrs.replace_punct(attr.span(), value)?;
                        }
                        "kw" => {
                            let value = get_lit_str(name_value.lit)
                                .ok_or_else(|| Error::new_spanned(attr, LIT_STR_ERR))?;
                            attrs.replace_kw(attr.span(), Some(value))?;
                        }
                        "token" => {
                            let value = get_lit_str(name_value.lit)
                                .ok_or_else(|| Error::new_spanned(attr, LIT_STR_ERR))?;
                            attrs.replace_token(attr.span(), Some(value))?;
                        }
                        _ => bail_s!(attr, "Invalid attribute key"),
                    };
                }
                Meta::Path(path) => {
                    let key = strip_path(&path)?;
                    match &*key.to_string() {
                        "kw" => attrs.replace_kw(attr.span(), None)?,
                        "token" => attrs.replace_token(attr.span(), None)?,
                        _ => bail_s!(attr, "Key without value can only be kw or token"),
                    }
                }
                _ => return Err(Error::new_spanned(attr, "Meta must be name value or path")),
            };
        }

        Ok(attrs)
    }

    pub fn replace_punct(&mut self, span: Span, lit: LitStr) -> Result<()> {
        if self.punct.is_some() {
            return Err(Error::new(span, "Duplicate punct attribute"));
        }
        self.punct.replace(lit.value());

        Ok(())
    }

    pub fn replace_kw(&mut self, span: Span, lit: Option<LitStr>) -> Result<()> {
        if self.kw.is_some() {
            return Err(Error::new(span, "Duplicate kw attribute"));
        }
        self.kw.replace(lit.map(|s| s.value()));

        Ok(())
    }

    pub fn replace_token(&mut self, span: Span, lit: Option<LitStr>) -> Result<()> {
        if self.token.is_some() {
            return Err(Error::new(span, "Duplicate token attribute"));
        }
        self.token.replace(lit.map(|s| s.value()));

        Ok(())
    }
}

fn strip_path(path: &Path) -> Result<&Ident> {
    let path = &path.segments;
    if path.len() != 1 {
        return Err(Error::new_spanned(
            path,
            "The key must be a single segment path",
        ));
    }
    let last = path.last().unwrap();
    if !last.arguments.is_empty() {
        bail_s!(path, "The key should not have angle bracketed arguments");
    }
    let ident = &last.ident;
    Ok(ident)
}

fn get_lit_str(lit: Lit) -> Option<LitStr> {
    match lit {
        Lit::Str(lit_str) => Some(lit_str),
        _ => None,
    }
}
