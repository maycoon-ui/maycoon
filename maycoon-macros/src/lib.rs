#![warn(missing_docs)]

//! Macros for Maycoon => See `maycoon` crate.
//!
//! Contains procedural macros.

mod assets;
mod state;
mod svg_icon;
mod val;

/// Derives the `State` trait for the given struct.
#[proc_macro_derive(State)]
pub fn derive_state(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(state::derive_state(proc_macro2::TokenStream::from(input)))
}

/// Creates a new `Val` from an expression.
#[proc_macro]
pub fn val(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(val::val(proc_macro2::TokenStream::from(input)))
}

/// Create a new `SvgIcon` from the given SVG source.
///
/// This will either read a file path or download the SVG from the given URL using [ureq].
/// After the data has been retrieved, it will be saved as a static asset file in a temporary directory (e.g. `%temp%` on windows).
/// When re-executing this macro, the file can be re-loaded for faster compilation times.
#[proc_macro]
pub fn svg_icon(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(svg_icon::svg_icon(proc_macro2::TokenStream::from(input)))
}
