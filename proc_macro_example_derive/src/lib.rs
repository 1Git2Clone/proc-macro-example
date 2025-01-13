mod prelude;
mod reflective;

use prelude::*;
use reflective::{ReflectiveInput, ReflectiveInputType};

#[proc_macro_derive(Reflective)]
pub fn derive_reflective(input: TokenStream) -> TokenStream {
    use ReflectiveInputType as RIT;

    let data = parse_macro_input!(input as ReflectiveInput);
    let item_name = &data.0.ident;

    let get_item = |fn_name: TokenStream2, iter_fields: Box<dyn Iterator<Item = String>>| {
        quote! {
            impl #item_name {
                pub const fn #fn_name() -> &'static [&'static str] {
                    &[ #(#iter_fields),* ]
                }
            }
        }
    };

    match data.get_input_items() {
        RIT::Fields(fields) => get_item(
            quote!(get_fields),
            Box::new(fields.iter().flat_map(|f| &f.ident).map(Ident::to_string)),
        ),
        RIT::Variants(variants) => get_item(
            quote!(get_variants),
            Box::new(variants.iter().map(|v| v.ident.to_string())),
        ),
        RIT::Union(union) => get_item(
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
}
