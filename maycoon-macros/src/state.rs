use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[inline]
pub fn derive_state(input: TokenStream) -> TokenStream {
    let input = syn::parse2::<DeriveInput>(input.clone()).unwrap();

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics maycoon::core::state::State for #name #ty_generics #where_clause {}
    }
}
