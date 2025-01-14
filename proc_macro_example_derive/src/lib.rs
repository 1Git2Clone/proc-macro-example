/*!
# `#[derive(Reflective)]`

A derive macro that implements a `const` `get_fields()`/`get_variants()` method for `structs`,
`enums` and `unions` which returns the fields/variants as a reference to a slice.
*/

mod prelude;
mod reflective;

use prelude::*;
use reflective::{ReflectiveInput, ReflectiveInputType};

// Only proc macros can be exported in `proc_macro` crates.
#[proc_macro_derive(Reflective)]
pub fn derive_reflective(input: TokenStream) -> TokenStream {
    //                          ^---------- TokenStream is provided by `proc_macro` which is
    //                                      automatically added to all `[lib] proc_macro = true`
    //                                      crates.
    use ReflectiveInputType as RIT;

    let data = parse_macro_input!(input as ReflectiveInput);
    //         ^----------------- This concise syntax is why we implemented `syn::parse::Parse`
    //                            for `ReflectiveInput`.
    let item_name = &data.0.ident;

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

    match data.get_input_items() {
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
    }
    .into()
    //^-- Notice how we convert the `proc_macro2::TokenStream` output of `quote!` to
    // `proc_macro::TokenStream`.
}
