use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn derive_state(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics may_core::state::State for #name #ty_generics #where_clause {}
    };

    proc_macro::TokenStream::from(expanded)
}
