use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[inline]
pub fn derive_state(input: TokenStream) -> TokenStream {
    let input = syn::parse2::<DeriveInput>(input.clone()).unwrap();

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics may_core::state::State for #name #ty_generics #where_clause {}
    }
}

#[inline]
pub fn val(input: TokenStream) -> TokenStream {
    let output: TokenStream = if input.is_empty() {
        // empty val! means empty StateVal
        quote! {
            may_core::state::StateVal::new(|_| ())
        }
    } else if let Ok(expr) = syn::parse2::<syn::Expr>(input.clone()) {
        // return StateVal from raw expression without state access
        quote! {
            may_core::state::StateVal::new(|_| #expr)
        }
    } else if let Ok(lit) = syn::parse2::<syn::Lit>(input.clone()) {
        // return StateVal from raw literal without state access
        quote! {
            may_core::state::StateVal::new(|_| #lit)
        }
    } else {
        // expect a valid closure
        quote! {
            may_core::state::StateVal::new(#input)
        }
    };

    output
}
