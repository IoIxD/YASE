// proc macro !
// from https://github.com/eonm-abes/proc-macro-issue-minimal-example/

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parser};
use syn::{parse, parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn block_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    prev: Option<String>
                })
                .unwrap(),
        );
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    next: Option<String>
                })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}