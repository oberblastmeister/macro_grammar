use std::marker::PhantomData;

use rowan::{Language, SyntaxNode, SyntaxNodeChildren, SyntaxToken};
use smol_str::SmolStr;

pub use macro_grammar_impl::{ast, Language, Token};

pub trait Kind {}

impl<T> Kind for T where T: From<u16> + Into<u16> + PartialEq<T> {}
