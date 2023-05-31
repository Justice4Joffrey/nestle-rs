use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_derive(Nestle, attributes(nestle))]
pub fn derive_leaf(input: TokenStream) -> TokenStream {
    nestle_core::macros::derive_nestle(input.into()).into()
}
