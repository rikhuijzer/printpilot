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
