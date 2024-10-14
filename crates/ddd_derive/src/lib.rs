mod aggregate_root;
mod command;
mod entity;
mod query;
mod vo;

use proc_macro::TokenStream;

#[proc_macro_derive(AggregateRoot, attributes(aggregate_root))]
pub fn aggregate_root_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("unable to parse input token stream");

    aggregate_root::derive(&ast)
}

#[proc_macro_derive(Command, attributes(command))]
pub fn command_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("unable to parse input token stream");

    command::derive(&ast)
}

#[proc_macro_derive(Entity, attributes(entity))]
pub fn entity_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("unable to parse input token stream");

    entity::derive(&ast)
}

#[proc_macro_derive(Query, attributes(query))]
pub fn query_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("unable to parse input token stream");

    query::derive(&ast)
}

#[proc_macro_derive(VO, attributes(vo))]
pub fn vo_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("unable to parse input token stream");

    vo::derive(&ast)
}
