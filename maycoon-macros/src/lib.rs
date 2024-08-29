#![warn(missing_docs)]

//! Macros for Maycoon => See `maycoon` crate.
//!
//! Contains procedural macros.

mod state;

/// Derives the [State](may_core::state::State) trait for the given struct.
#[proc_macro_derive(State)]
pub fn derive_state(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(state::derive_state(proc_macro2::TokenStream::from(input)))
}

/// Creates a new [StateVal](may_core::state::Val) from an expression.
#[proc_macro]
pub fn val(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(state::val(proc_macro2::TokenStream::from(input)))
}
