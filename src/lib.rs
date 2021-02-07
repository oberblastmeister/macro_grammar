mod derive_t;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(T, attributes(punct, kw, token))]
pub fn derive_t(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_t::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

// #[proc_macro]
// pub fn ast(attr: TokenStream, input: TokenStream) -> TokenStream {
//     todo!()
// }
