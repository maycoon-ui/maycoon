use crate::assets;
use proc_macro2::TokenStream;
use quote::quote;
use std::path::Path;
use syn::{Expr, Lit};

#[inline]
pub fn svg_icon(input: TokenStream) -> TokenStream {
    let input: Expr = syn::parse2(input).expect("failed to parse input");

    match input {
        Expr::Lit(expr_lit) => match expr_lit.lit {
            Lit::Str(lit) => {
                let lit = lit.value();
                let source = Path::new(&lit);

                let file_name = source
                    .file_name()
                    .expect("Failed to get file name")
                    .to_str()
                    .unwrap();

                let svg = assets::get_or_create_asset(Path::new("icons"), file_name, || {
                    if source.starts_with("http") || source.starts_with("https") {
                        ureq::get(source.to_str().unwrap())
                            .call()
                            .expect("Failed to download file")
                            .into_body()
                            .read_to_string()
                            .expect("Failed to read file")
                    } else {
                        std::fs::read_to_string(source).expect("Failed to read file")
                    }
                });

                quote! {
                    maycoon::widgets::icon::SvgIcon::new(
                        #svg
                    ).expect("Failed to parse SVG")
                }
            },

            _ => panic!("Expected string literal"),
        },

        _ => panic!("Expected literal"),
    }
}
