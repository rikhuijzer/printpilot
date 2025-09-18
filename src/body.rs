use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
pub fn submit_body_upload() {
    console_log!("hi 4");
    let document = window().unwrap().document().unwrap();
    let body_upload = document.get_element_by_id("body-upload").unwrap();
    let input = body_upload
        .dyn_ref::<web_sys::HtmlInputElement>()
        .expect("Element is not an HtmlInputElement");
    let files = input.files().expect("No files property on input element");
    if files.length() == 0 {
        console_log!("No file selected");
    } else {
        let file = files.item(0).expect("Failed to get file");
        console_log!("File selected: {}", file.name());
        let body_output = document.get_element_by_id("body-output").unwrap();
        body_output.set_inner_html(&format!("File selected: {}", file.name()));
    }
}
