use crate::prelude::*;

pub enum ReflectiveInputType<'a> {
    Fields(&'a Fields),
    Variants(&'a Punctuated<Variant, Token![,]>),
    Union(&'a FieldsNamed),
}

pub struct ReflectiveInput(pub DeriveInput);

impl ReflectiveInput {
    pub fn get_input_items(&self) -> ReflectiveInputType {
        use ReflectiveInputType as RIT;
        match self.0.data {
            Data::Struct(ref v) => RIT::Fields(&v.fields),
            Data::Enum(ref v) => RIT::Variants(&v.variants),
            Data::Union(ref v) => RIT::Union(&v.fields),
        }
    }
}

impl Parse for ReflectiveInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}
