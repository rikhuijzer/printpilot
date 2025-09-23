#[macro_use]
mod tracing;

use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
pub fn reset_body_upload() {
    let document = window().unwrap().document().unwrap();
    let body_upload = document.get_element_by_id("body-upload").unwrap();
    let input = body_upload
        .dyn_ref::<web_sys::HtmlInputElement>()
        .expect("Element is not an HtmlInputElement");
    input.set_value("");
}

#[wasm_bindgen]
pub fn create_cover() {
    let document = window().unwrap().document().unwrap();
    let cover_title = document.get_element_by_id("cover-title").unwrap();
    let title = cover_title.inner_html();
}

fn main() {
    console_error_panic_hook::set_once();
    console_log!("Library loaded successfully");
}
