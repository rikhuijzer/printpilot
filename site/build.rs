use std::process::Command;

fn main() {
    let root_dir = std::env::current_dir().expect("Failed to get current dir").join("..");

    std::env::set_current_dir("core").unwrap();

    let status = Command::new("cargo")
        .args(&["build", "--target", "wasm32-unknown-unknown"])
        .status()
        .expect("Failed to execute cargo build");

    if !status.success() {
        panic!("cargo build failed");
    }

    let target = root_dir.join("_public/core.wasm");
    std::fs::copy(
        "target/wasm32-unknown-unknown/debug/core.wasm",
        target,
    )
    .expect("Failed to copy core.wasm");
}
