#![warn(clippy::all, clippy::perf, clippy::pedantic)]

extern crate proc_macro;

use crate::proc_macro::TokenStream;

/// Adds the function scope to the scoper recording
/// Takes optional header attribute
/// # Panics
///
/// Panics if not used with functions
#[proc_macro_attribute]
pub fn record(attr: TokenStream, input: TokenStream) -> TokenStream
{
    //let mut attr = attr.into_iter();
    //let header = attr.next().map(|t| t.to_string());

    let mut input: syn::ItemFn = syn::parse2(input.into()).expect("Use #[record] only on functions.");
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

    let mut ext: syn::ItemFn = 
    if attr.is_empty()
    {
        syn::parse_quote! {
            fn bla()
            {
                record_scope!(#name);
            }
        }
    }
    else
    {
        let header = attr.to_string();
        syn::parse_quote! {
            fn bla()
            {
                record_scope!(#header, #name);
            }
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
