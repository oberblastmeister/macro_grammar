use common::{bail, bail_s};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed, Ident, Path, Result, Type, Visibility, parse_quote};

#[derive(Debug)]
pub enum Input<'a> {
    Enum(Enum<'a>),
    Struct(Struct<'a>),
}

impl<'a> Input<'a> {
    pub fn from_syn(node: &'a DeriveInput) -> Result<Input<'a>> {
        match &node.data {
            Data::Enum(data) => Enum::from_syn(node, data).map(Input::Enum),
            Data::Struct(data) => Struct::from_syn(node, data).map(Input::Struct),
            Data::Union(_) => bail_s!(node, "Union types are not supported"),
        }
    }
}

#[derive(Debug)]
pub struct Enum<'a> {
    pub original: &'a DeriveInput,
    pub variants: Vec<Variant<'a>>,
}

impl<'a> Enum<'a> {
    fn from_syn(node: &'a DeriveInput, data: &'a DataEnum) -> Result<Enum<'a>> {
        if data.variants.is_empty() {
            bail_s!(node, "The enum must not be empty");
        }

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

#[derive(Debug)]
pub struct Variant<'a> {
    pub ident: &'a Ident,
}

impl<'a> Variant<'a> {
    fn from_syn(variant: &'a syn::Variant) -> Result<Variant<'a>> {
        Ok(Variant {
            ident: &variant.ident,
        })
    }
}

#[derive(Debug)]
pub enum Struct<'a> {
    Named(NamedStruct<'a>),
    Unnamed(UnnamedStruct<'a>),
}

impl<'a> Struct<'a> {
    fn from_syn(node: &'a DeriveInput, data: &'a DataStruct) -> Result<Struct<'a>> {
        match &data.fields {
            Fields::Named(fields) => NamedStruct::from_syn(&node.ident, fields).map(Struct::Named),
            Fields::Unnamed(fields) => {
                UnnamedStruct::from_syn(&node.ident, Some(fields)).map(Struct::Unnamed)
            }
            Fields::Unit => UnnamedStruct::from_syn(&node.ident, None).map(Struct::Unnamed),
        }
    }
}

#[derive(Debug)]
pub struct NamedStruct<'a> {
    pub fields: Vec<Field<'a>>,
}

impl<'a> NamedStruct<'a> {
    fn from_syn(ident: &'a Ident, fields: &'a FieldsNamed) -> Result<NamedStruct<'a>> {
        let fields = fields
            .named
            .iter()
            .map(|field| Field::from_syn(field))
            .collect::<Result<Vec<_>>>()?;

        Ok(NamedStruct { fields })
    }
}

#[derive(Debug)]
pub struct Field<'a> {
    pub vis: &'a Visibility,
    pub ident: &'a Ident,
    pub ty: &'a Type,
}

impl<'a> Field<'a> {
    fn from_syn(field: &'a syn::Field) -> Result<Field<'a>> {
        Ok(Field {
            vis: &field.vis,
            ident: field
                .ident
                .as_ref()
                .expect("Should only be calling this method for named struct"),
            ty: &field.ty,
        })
    }
}

#[derive(Debug)]
pub struct UnnamedStruct<'a> {
    pub ident: &'a Ident,
    pub kind: Path,
}

impl<'a> UnnamedStruct<'a> {
    fn from_syn(ident: &'a Ident, fields: Option<&'a FieldsUnnamed>) -> Result<UnnamedStruct<'a>> {
        let ty: Path = if let Some(fields) = fields {
            let fields = &fields.unnamed;

            if fields.len() != 1 {
                bail_s!(fields, "Must have one unnamed field");
            }

            let ty = &fields.last().unwrap().ty;
            parse_quote! { SyntaxKind::#ty }
        } else {
            parse_quote! { SyntaxKind::#ident }
        };

        Ok(UnnamedStruct { ident, kind: ty })
    }
}
