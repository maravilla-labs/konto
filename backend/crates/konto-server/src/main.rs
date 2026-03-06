#[allow(clippy::expect_used)]
fn main() {
    // Use a larger stack size to handle deep Axum router types in debug builds
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(8 * 1024 * 1024)
        .build()
        .expect("Failed to build tokio runtime");

    rt.block_on(konto_server::startup::run_standalone());
}
