/*!
# Reflective utility types

These utility types are here to systematize the implementation of the proc macro and do extract
certain logic parts into their own data structures. While this isn't needed, I strongly recommend
you work with proc macros this way because otherwise it'd quickly become a mess to navigate through.
*/

use crate::prelude::*;

/// Represents a item's fields/variants depending on the item type.
///
/// Example:
///
/// ```rust
/// struct SomeStruct {
///     field_1: u32,
///     field_2: u32,
///     field_3: u32,
/// }// ^------ This enum will capture the `Fields` variant since it's a `struct`.
/// enum SomeEnum {
///     VariantOne,
///     VariantTwo,
/// }// ^------ This enum will capture the `Variants` variant since it's an `enum`.
/// union SomeUnion {
///     field_1: u32,
///     field_2: u32,
///     field_3: u32,
/// }// ^------ This enum will capture the `Union` variant since it's a `union`.
/// ```
pub enum ReflectiveInputType<'a> {
    Fields(&'a Fields),
    // `syn::punctuated::Punctuated` is also a type that saves boilerplate. For a declarative macro
    // equivalent it'd be parsing: `$($x:Variant),+ $(,)?` (if declarative macros had `Variant` as
    // a fragment specifier that is). This extra comma token makes it so there could or could not
    // be one more type after the last comma token parsed. We take these things for granted as
    // users but this has to be addressed while we're parsing Rust code.
    Variants(&'a Punctuated<Variant, Token![,]>),
    UnionFields(&'a FieldsNamed),
}

/// Wrapper around `syn::DeriveInput` with extra reflection functionality.
pub struct ReflectiveInput(pub DeriveInput);

impl ReflectiveInput {
    /// Gets the item's fields/variants.
    pub fn get_input_items(&self) -> ReflectiveInputType {
        use ReflectiveInputType as RIT;
        match &self.0.data {
            Data::Struct(v) => RIT::Fields(&v.fields),
            Data::Enum(v) => RIT::Variants(&v.variants),
            Data::Union(v) => RIT::UnionFields(&v.fields),
        }
    }
}

// The implementation is done in order to use syn's `parse_macro_input!` macro in the derive macro.
impl Parse for ReflectiveInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}
