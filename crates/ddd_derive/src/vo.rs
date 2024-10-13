use proc_macro::TokenStream;
use quote::quote;

pub fn derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl VO for #name {

        }
    };

    gen.into()
}
