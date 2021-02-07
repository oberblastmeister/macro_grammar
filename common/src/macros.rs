/// Same as `bail!` in anyhow but for syn Errors. Turns tokens into span.
#[macro_export]
macro_rules! bail_s {
    ($to_tokens:expr, $mes:expr) => {
        return Err(syn::Error::new_spanned($to_tokens, $mes))
    };
}

/// Same as `bail!` in anyhow but for syn Errors. Must provide span
#[macro_export]
macro_rules! bail {
    ($span:expr, $mes:expr) => {
        return Err(syn::Error::new($span, $mes))
    }
}
