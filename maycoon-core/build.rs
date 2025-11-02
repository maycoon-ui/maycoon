fn main() {
    cfg_aliases::cfg_aliases! {
        web: { target_arch = "wasm32" },
        native: { not(target_arch = "wasm32") },
    }
}
