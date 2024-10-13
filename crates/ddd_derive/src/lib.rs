mod aggregate_root;
mod entity;
mod vo;

use proc_macro::TokenStream;

#[proc_macro_derive(AggregateRoot)]
pub fn aggregate_root_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("unable to parse input token stream");

    aggregate_root::derive(&ast)
}

#[proc_macro_derive(Entity)]
pub fn entity_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("unable to parse input token stream");

    entity::derive(&ast)
}

#[proc_macro_derive(VO)]
pub fn vo_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("unable to parse input token stream");

    vo::derive(&ast)
}
