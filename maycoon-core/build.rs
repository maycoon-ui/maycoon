fn main() {
    if cfg!(not(any(feature = "tokio-runner", feature = "dummy-runner"))) {
        panic!(
            "No valid task runner feature selected. Please select a `-runner` feature (e.g. `tokio-runner`)."
        );
    }

    cfg_aliases::cfg_aliases! {
        web: { target_arch = "wasm32" },
        native: { not(target_arch = "wasm32") },
    }
}
