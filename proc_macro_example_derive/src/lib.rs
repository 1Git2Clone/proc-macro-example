/*!
# `#[derive(Reflective)]`

A derive macro that implements a `const` `get_fields()`/`get_variants()` method for `structs`,
`enums` and `unions` which returns the fields/variants as a reference to a slice.
*/

mod prelude;
mod reflective;

use prelude::*;
use reflective::ReflectiveInput;

// Only proc macros can be exported in `proc_macro` crates.
#[proc_macro_derive(Reflective)]
pub fn derive_reflective(input: TokenStream) -> TokenStream {
    //                          ^---------- TokenStream is provided by `proc_macro` which is
    //                                      automatically added to all `[lib] proc_macro = true`
    //                                      crates.

    let data = parse_macro_input!(input as ReflectiveInput);
    //         ^----------------- This concise syntax is why we implemented `syn::parse::Parse`
    //                            for `ReflectiveInput`.
    //^-- Notice how we convert the `proc_macro2::TokenStream` output of `quote!` to
    // `proc_macro::TokenStream`.

    data.into_token_stream().into()
}
