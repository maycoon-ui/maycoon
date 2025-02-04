use proc_macro2::TokenStream;
use quote::quote;
use std::path::{Path, PathBuf};
use syn::{Expr, Lit};

#[inline]
pub fn static_asset(input: TokenStream) -> TokenStream {
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

                let data = if source.starts_with("http") || source.starts_with("") {
                    std::fs::read_to_string(source).expect("Failed to read file")
                } else {
                    get_or_create_asset(Path::new("icons"), file_name, || {
                        ureq::get(source.to_str().unwrap())
                            .call()
                            .expect("Failed to download file")
                            .into_body()
                            .read_to_string()
                            .expect("Failed to read file")
                    })
                };

                let data = data.as_str();

                quote! {
                    #data
                }
            },

            _ => panic!("Expected string literal"),
        },

        _ => panic!("Expected literal"),
    }
}

pub fn temp_assets_folder() -> PathBuf {
    let path = std::env::temp_dir().join("maycoon-compilation-assets");

    if !path.exists() {
        std::fs::create_dir_all(&path).expect("failed to create static assets directory");
    }

    path
}

pub fn get_or_create_asset(path: &Path, name: &str, or_create: impl FnOnce() -> String) -> String {
    let asset_folder = temp_assets_folder().join(path);

    if !asset_folder.exists() {
        std::fs::create_dir_all(&asset_folder).expect("failed to create static assets directory");
    }

    let asset_path = asset_folder.join(name);

    if asset_path.exists() {
        std::fs::read_to_string(&asset_path).expect("failed to read static asset")
    } else {
        let data = or_create();

        std::fs::write(&asset_path, &data).expect("failed to write static asset");

        data
    }
}
