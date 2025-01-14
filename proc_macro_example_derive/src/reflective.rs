/*!
# Reflective utility types

These utility types are here to systematize the implementation of the proc macro and do extract
certain logic parts into their own data structures. While this isn't needed, I strongly recommend
you work with proc macros this way because otherwise it'd quickly become a mess to navigate through.
*/

use quote::ToTokens;

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

impl ToTokens for ReflectiveInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        use ReflectiveInputType as RIT;

        let item_name = &self.0.ident;

        let get_item = |fn_name: TokenStream2, iter_fields: Box<dyn Iterator<Item = String>>| {
            //          ^--------------------  ^---------- Boxing since it's dynamically dispatched.
            //          |                      You can do it without a callback, this just saves code
            //          |                      writing.
            //          |
            //          |> This could be any token stream. It's unhygenic so we should be careful what
            //          we call it with as it's supposed to be an indentifier. Fortunately, due to
            //          everything working in the rust compiler, we'd get Rust compiler errors in case
            //          we call it with something wrong. That's why it's important to have integration
            //          tests with proc macros since they can quickly get complicated and mistakes like
            //          this could be missed.
            //
            //          NOTE: Notice how it's `TokenStream2` instead of `TokenStream`. This is because
            //          the `quote` crate works with `TokenStream2` and the reason is that the
            //          `proc_macro2` crate doesn't require the user to have a proc macro crate. This is
            //          useful for making libraries that work with proc macro code without being proc
            //          macros themselves (since you can only export proc macros in proc macro crates).

            // This syntax is very similar to the traditional declarative macro syntax. The main
            // difference is instead of `$` we use `#`. Additionally we can use variables from our
            // #[proc_macro] function like so: `#variable`.
            quote! {
                impl #item_name {
                    pub const fn #fn_name() -> &'static [&'static str] {
                        //       ^------- We can even substitute identifiers.

                        &[ #(#iter_fields),* ]
                        // ^- ********** --- `#(),*` expands similarly to how a declarative macro would
                        //                           be expanded.
                    }
                }
            }
        };

        let res = match self.get_input_items() {
            RIT::Fields(fields) => get_item(
                quote!(get_fields),
                //^--------------- we need to quote the identifier to use it.
                Box::new(fields.iter().flat_map(|f| &f.ident).map(Ident::to_string)),
                //                     ^------- You can do it with `filter_map()` as well. `flat_map()`
                //                     is just more powerful though, since it combines: mapping,
                //                     filtering and flattening into one callback.
                //
                //                     Also, we flat_map it since the struct could also just be a tuple
                //                     struct so `Field.ident` is `Option<Ident>` because of it.
            ),
            RIT::Variants(variants) => get_item(
                quote!(get_variants),
                Box::new(variants.iter().map(|v| v.ident.to_string())),
            ),
            RIT::UnionFields(union) => get_item(
                quote!(get_fields),
                Box::new(
                    union
                        .named
                        .iter()
                        .flat_map(|f| &f.ident)
                        .map(Ident::to_string),
                ),
            ),
        };

        tokens.extend(res);
    }
}
