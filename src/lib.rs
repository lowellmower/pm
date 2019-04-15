extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_get_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // create an arbitrary public function on the AST
    // derived name and tokenize for return
    let result = quote! {
        impl #name {
            pub fn get_id() -> u8 { 9 }
        }
    };

    result.into()
}

#[proc_macro_derive(HelloFields)]
pub fn hello_fields_print_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident_name = &input.ident;
    // let data = &input.data;

    let result = quote! {
        impl #ident_name {
            fn print_name(&self) -> String {
                self.name.clone()
            }
        }
    };

    result.into()
}
