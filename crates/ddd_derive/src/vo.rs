use proc_macro::TokenStream;
use quote::quote;

pub fn derive(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let generics = &ast.generics;

    let gen = quote! {
        impl #generics ddd::prelude::v1::VO for #ident #generics {}
    };

    gen.into()
}
