#[macro_use]
mod tracing;
mod body;

fn main() {
    console_error_panic_hook::set_once();
    console_log!("Library loaded successfully");
}
