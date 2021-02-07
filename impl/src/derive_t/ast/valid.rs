use syn::Result;
use super::{Enum, Input};

pub(crate) trait Validate {
    fn validate(&self) -> Result<()>;
}

impl Validate for Input<'_> {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

impl Validate for Enum<'_> {
    fn validate(&self) -> Result<()> {
        todo!()
    }
}
