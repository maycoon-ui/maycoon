mod state;

#[proc_macro_derive(State)]
pub fn derive_state(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(state::derive_state(proc_macro2::TokenStream::from(input)))
}

#[proc_macro]
pub fn val(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(state::val(proc_macro2::TokenStream::from(input)))
}
