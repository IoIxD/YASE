// proc macro !
// from https://github.com/eonm-abes/proc-macro-issue-minimal-example/

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn block_derive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);
    let name = item_struct.ident.clone();

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    pub(crate) prev: Option<String>
                })
                .unwrap(),
        );
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    pub(crate) next: Option<String>
                })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct

        impl Block for #name {
            fn prev(&self) -> Option<String> {
                self.prev.clone()
            }
            fn next(&self) -> Option<String> {
                self.next.clone()
            }
        }
    }
    .into();
}
