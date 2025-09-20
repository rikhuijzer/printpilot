use lopdf::Document;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response;
use web_sys::js_sys::Uint8Array;
use web_sys::window;

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
    data: Vec<u8>,
}

fn process_body_upload(body_upload: BodyUpload) -> Vec<u8> {
    let data = body_upload.data;
    console_log!("Data len: {:?}", data.len());
    let mut doc = Document::load_mem(&data).unwrap();
    for (_, page_id) in doc.get_pages() {
        let page_dict = doc
            .get_object_mut(page_id)
            .and_then(|obj| obj.as_dict_mut())
            .expect("Missing page!");

        // Get the current rotation if any; the default is 0
        let current_rotation = page_dict.get(b"Rotate").and_then(|obj| obj.as_i64()).unwrap_or(0);

        // Add the angle and update
        page_dict.set("Rotate", (current_rotation + 180) % 360);
    }
    let mut target = Vec::new();
    doc.save_to(&mut target).unwrap();
    target
}

fn open_pdf(data: Vec<u8>) {
    let data = web_sys::js_sys::Uint8Array::from(&data[..]);
    let blob_properties = web_sys::BlobPropertyBag::new();
    blob_properties.set_type("application/pdf");
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&data, &blob_properties).unwrap();
    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
    web_sys::window().unwrap().open_with_url(&url).unwrap();
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
        let resp = fetch("/static/bushido.pdf").await.unwrap();
        let body_output = document.get_element_by_id("body-output").unwrap();
        body_output.set_inner_html(&format!("File selected: {}", resp.status()));
        if resp.status() != 200 {
            console_log!("Failed to fetch bushido.pdf");
            return;
        }
        let data = resp.blob().unwrap();
        let data = JsFuture::from(data).await.unwrap();
        let blob = data.dyn_into::<web_sys::Blob>().unwrap();
        let array_buffer = JsFuture::from(blob.array_buffer()).await.unwrap();
        let uint8_array = Uint8Array::new(&array_buffer);
        let data = uint8_array.to_vec();
        BodyUpload {
            name: "bushido.pdf".to_string(),
            data,
        }
    } else {
        let file = files.item(0).expect("Failed to get file");
        let data = file.array_buffer();
        let data = JsFuture::from(data).await.unwrap();
        let uint8_array = Uint8Array::new(&data);
        let data = uint8_array.to_vec();
        BodyUpload {
            name: file.name().to_string(),
            data,
        }
    };
    let body_output = document.get_element_by_id("body-output").unwrap();
    body_output.set_inner_html(&format!("File selected: {}", body_upload.name));

    let data = process_body_upload(body_upload);
    open_pdf(data);
    
}
