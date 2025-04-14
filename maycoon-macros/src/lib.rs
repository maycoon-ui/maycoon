#![warn(missing_docs)]

//! Macros for Maycoon => See `maycoon` crate.
//!
//! Contains procedural macros.

mod assets;
mod svg_icon;

/// Create a new `SvgIcon` from the given SVG source.
///
/// This is equivalent to `SvgIcon::new(static_asset!(url))` and works as a convenience macro.
#[proc_macro]
pub fn svg_icon(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(svg_icon::svg_icon(proc_macro2::TokenStream::from(input)))
}

/// Creates a static asset from the given path/url and caches the file for faster compilation times.
///
/// This will either read a file path or download the file from the given URL using [ureq].
/// After the data has been retrieved, it will be saved as a static asset file in a temporary directory (e.g. `%temp%` on windows).
/// When re-executing this macro, the file can be re-loaded for faster compilation times.
#[proc_macro]
pub fn static_asset(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(assets::static_asset(proc_macro2::TokenStream::from(input)))
}
