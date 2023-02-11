use proc_macro::TokenStream;

mod deserialize;
mod serialize;

#[proc_macro_derive(DeserializeField, attributes(protocol))]
pub fn deserialize(input: TokenStream) -> TokenStream {
    deserialize::derive_impl(input)
}

#[proc_macro_derive(SerializeField, attributes(protocol))]
pub fn serialize(input: TokenStream) -> TokenStream {
    serialize::derive_impl(input)
}
