extern crate proc_macro;

use crate::proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn record(_attr: TokenStream, input: TokenStream) -> TokenStream
{
    input
}
