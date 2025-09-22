use lopdf::Document;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;
use lopdf::content::Content;
use lopdf::content::Operation;
use lopdf::dictionary;
use lopdf::Object;
use lopdf::Stream;
use std::collections::HashMap;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response;
use web_sys::js_sys::Array;
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

fn convert_to_zero_based(i: i64) -> i64 {
    i - 1
}

fn ceildiv(a: u64, b: u64) -> u64 {
    u64::div_ceil(a, b)
}

fn pages_indices(doc: &Document) -> HashMap<u32, (u32, u16)> {
    let mut pages_indices = HashMap::new();
    for (i, page_id) in doc.get_pages() {
        pages_indices.insert(i, page_id);
    }
    pages_indices
}

fn python_range(end: u64) -> Vec<u64> {
    (0..end).collect()
}

#[test]
fn test_python_range() {
    assert_eq!(python_range(0), Vec::new());
    assert_eq!(python_range(5), vec![0, 1, 2, 3, 4]);
}

fn blank_page() -> Vec<u8> {
    vec![]
}

fn fake_document() -> Document {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Courier",
    });
    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! {
            "F1" => font_id,
        },
    });
    let content = Content {
        operations: vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 48.into()]),
            Operation::new("Td", vec![100.into(), 600.into()]),
            Operation::new("Tj", vec![Object::string_literal("Hello World!")]),
            Operation::new("ET", vec![]),
        ],
    };
    let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    });
    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![page_id.into()],
        "Count" => 1,
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages));
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);

    doc
}

fn process_body_upload(body_upload: BodyUpload) -> Vec<u8> {
    let data = body_upload.data;
    console_log!("Data len: {:?}", data.len());
    let mut doc = Document::load_mem(&data).unwrap();
    let n = doc.get_pages().len();
    console_log!("n: {:?}", n);
    let half = ceildiv(n as u64, 2);
    // If n is odd, we need to add one since we are printing duplex.
    // Can lead to 3 empty pages in the worst case.
    let half = if n % 2 == 0 { half } else { half + 1 };
    console_log!("half: {:?}", half);
    let indices = &pages_indices(&doc);

    // let mut out = fake_document();
    // let pages_id = out.new_object_id();
    doc.delete_pages(&(1..n as u32).collect::<Vec<u32>>());
    for i in python_range(ceildiv(half, 2)) {
        console_log!("i: {:?}", i);
        // half + 2 : 2 : n
        let left_index = (half + 2) as u32 + (2 * i as u32);
        console_log!("left_index: {:?}", left_index);
        // 2 : 2 : half
        // let right_index = convert_to_zero_based(2 + (2 * i));
        let left_content = if left_index < n as u32 {
            let left_page_id = indices[&left_index].clone();
            doc.get_page_content(left_page_id).unwrap()
        } else {
            blank_page()
        };
        let some_page_id = indices[&(i as u32 + 1)];
        doc.add_page_contents(some_page_id, left_content).unwrap();
    }
    let mut target = Vec::new();
    doc.save_to(&mut target).unwrap();
    target
}

fn create_pdf_link(data: Vec<u8>) -> String {
    let data = web_sys::js_sys::Uint8Array::from(data.as_slice());
    let data = Array::of1(&data);
    let blob_properties = web_sys::BlobPropertyBag::new();
    blob_properties.set_type("application/pdf");
    let blob =
        web_sys::Blob::new_with_u8_array_sequence_and_options(&data, &blob_properties).unwrap();
    web_sys::Url::create_object_url_with_blob(&blob).unwrap()
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
    let name = body_upload.name.clone();
    let body_output = document.get_element_by_id("body-output").unwrap();

    let data = process_body_upload(body_upload);
    let url = create_pdf_link(data);
    body_output.set_inner_html(&format!(
        r#"<a href="{url}" target="_blank">Open {name}</a>"#
    ));
}
