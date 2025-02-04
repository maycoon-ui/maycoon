use crate::assets;
use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

#[inline]
pub fn svg_icon(input: TokenStream) -> TokenStream {
    let data = syn::parse2::<LitStr>(assets::static_asset(input))
        .expect("failed to parse input")
        .value();

    quote! {
        maycoon::widgets::icon::svg::SvgIcon::new(#data).expect("failed to create SVG icon")
    }
}
