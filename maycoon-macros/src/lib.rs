#![warn(missing_docs)]

//! Macros for Maycoon => See `maycoon` crate.
//!
//! Contains procedural macros.

mod state;
mod val;

/// Derives the [State](maycoon::core::state::State) trait for the given struct.
#[proc_macro_derive(State)]
pub fn derive_state(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(state::derive_state(proc_macro2::TokenStream::from(input)))
}

/// Creates a new [Val](maycoon::core::state::Val) from an expression.
#[proc_macro]
pub fn val(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(val::val(proc_macro2::TokenStream::from(input)))
}
