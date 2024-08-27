use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Expr};

#[inline]
pub fn derive_state(input: TokenStream) -> TokenStream {
    let input = syn::parse2::<DeriveInput>(input.clone()).unwrap();

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics maycoon::core::state::State for #name #ty_generics #where_clause {}
    }
}

#[inline]
pub fn val(input: TokenStream) -> TokenStream {
    let output: TokenStream = if input.is_empty() {
        // empty val! means empty StateVal
        quote! {
            maycoon::core::state::StateVal::new(|_| ())
        }
    } else if let Ok(expr) = syn::parse2::<Expr>(input.clone()) {
        match expr {
            // StateVal from valid closure
            Expr::Closure(closure) => {
                quote! {
                    maycoon::core::state::StateVal::new(#closure)
                }
            },

            // Everything else is a normal expression
            _ => {
                quote! {
                    maycoon::core::state::StateVal::new(|_| #expr)
                }
            },
        }
    } else {
        quote! {
            maycoon::core::state::StateVal::new(#input)
        }
    };

    output
}
