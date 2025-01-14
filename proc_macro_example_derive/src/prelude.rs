/*!
# Prelude

Commonly used type re-exports for the crate.
*/

pub use proc_macro::TokenStream;

pub use proc_macro2::TokenStream as TokenStream2;
//                                  ^----------- Personal preference

pub use quote::{quote, ToTokens};
pub use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Data, DeriveInput, Fields, FieldsNamed, Ident, Token, Variant,
};
