use std::process::Command;

fn main() {
    let root_dir = std::env::current_dir().unwrap();

    std::env::set_current_dir("core").expect("Failed to set dir to core");

    let status = Command::new("cargo")
        .args(&["build", "--target", "wasm32-unknown-unknown"])
        .status()
        .expect("Failed to execute cargo build");

    if !status.success() {
        panic!("cargo build failed");
    }

    std::fs::copy(
        "target/wasm32-unknown-unknown/debug/core.wasm",
        root_dir.join("_public/core.wasm"),
    )
    .expect("Failed to copy core.wasm");
}
