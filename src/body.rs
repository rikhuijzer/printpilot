use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Uint8Array;
use web_sys::window;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response;

async fn fetch(url: &str) -> Result<web_sys::Response, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request));
    let resp: Response = resp_value.await?.dyn_into()?;

    Ok(resp)
}

#[wasm_bindgen]
pub fn reset_body_upload() {
    let document = window().unwrap().document().unwrap();
    let body_upload = document.get_element_by_id("body-upload").unwrap();
    let input = body_upload
        .dyn_ref::<web_sys::HtmlInputElement>()
        .expect("Element is not an HtmlInputElement");
    input.set_value("");
}

struct BodyUpload {
    // The filename.
    name: String,
    // The file blob.
    data: JsValue,
}

impl BodyUpload {
    #[allow(dead_code)]
    fn array_buffer(&self) -> Uint8Array {
        web_sys::js_sys::Uint8Array::new(&self.data)
    }
}

fn process_body_upload(body_upload: BodyUpload) {
    let data = body_upload.array_buffer();
    console_log!("Data: {:?}", data);
}

#[wasm_bindgen]
pub async fn submit_body_upload() {
    let document = window().unwrap().document().unwrap();
    let body_upload = document.get_element_by_id("body-upload").unwrap();
    let input = body_upload
        .dyn_ref::<web_sys::HtmlInputElement>()
        .expect("Element is not an HtmlInputElement");
    let files = input.files().expect("No files property on input element");
    let body_upload = if files.length() == 0 {
        let resp = fetch("/bushido.pdf").await.unwrap();
        let body_output = document.get_element_by_id("body-output").unwrap();
        body_output.set_inner_html(&format!("File selected: {}", resp.status()));
        if resp.status() != 200 {
            console_log!("Failed to fetch bushido.pdf");
            return;
        }
        let data = resp.blob().unwrap();
        let data = wasm_bindgen_futures::JsFuture::from(data).await.unwrap();
        BodyUpload {
            name: "bushido.pdf".to_string(),
            data,
        }
    } else {
        let file = files.item(0).expect("Failed to get file");
        let data = file.array_buffer();
        let data = wasm_bindgen_futures::JsFuture::from(data).await.unwrap();
        BodyUpload {
            name: file.name().to_string(),
            data,
        }
    };
    let body_output = document.get_element_by_id("body-output").unwrap();
    body_output.set_inner_html(&format!("File selected: {}", body_upload.name));
    process_body_upload(body_upload);
}
