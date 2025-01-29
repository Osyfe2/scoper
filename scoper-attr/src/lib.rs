#![warn(clippy::all, clippy::perf, clippy::pedantic)]

extern crate proc_macro;

use crate::proc_macro::TokenStream;

/// Creates a trace that can be recorded for the runtime of the function.
///
/// # Panics
///
/// Panics if not used with functions or - for now - with additional attributes.
#[proc_macro_attribute]
pub fn trace(attr: TokenStream, input: TokenStream) -> TokenStream
{
    assert!(attr.is_empty(), "Use #[trace] without further attributes."); //TODO support header

    let mut input: syn::ItemFn = syn::parse2(input.into()).expect("Use #[trace] only on functions.");
    let name = input.sig.ident.to_string();

    /* //Todo generics
    for generics in input.sig.generics.params
    {
        match generics
        {
            syn::GenericParam::Lifetime(lifetime_param) => "".to_string(),
            syn::GenericParam::Type(type_param) => type_param.,
            syn::GenericParam::Const(const_param) => todo!(),
        }
        let gname = generics.to_string();
    }*/

    let mut ext: syn::ItemFn = syn::parse_quote! {
        fn bla()
        {
            trace_scope!(#name);
        }
    };

    ext.block.stmts.append(&mut input.block.stmts);
    std::mem::swap(&mut ext.block, &mut input.block);

    let mut output = quote::quote!();

    output.extend(quote::quote! {
        #input
    });

    TokenStream::from(output)
}
