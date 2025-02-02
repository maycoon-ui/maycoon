use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

#[inline]
pub fn val(input: TokenStream) -> TokenStream {
    let output: TokenStream = if input.is_empty() {
        // empty val! means empty StateVal
        quote! {
            maycoon::core::state::Val::new_val(())
        }
    } else if let Ok(expr) = syn::parse2::<Expr>(input.clone()) {
        match expr {
            // StateVal from valid closure
            Expr::Closure(closure) => {
                quote! {
                    maycoon::core::state::Val::new_state(#closure)
                }
            },

            // Everything else should be a "constant" expression
            _ => {
                quote! {
                    maycoon::core::state::Val::new_val(#expr)
                }
            },
        }
    } else {
        // FIXME: Expr::Closure doesn't catch all closures, so we need to handle them here
        quote! {
            maycoon::core::state::Val::new_state(#input)
        }
    };

    output
}
