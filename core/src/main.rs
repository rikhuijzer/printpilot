mod transfer;
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use web_sys;

#[derive(Deserialize, Serialize)]
struct File {
    name: String,
    typ: String,
    size: u64,
    data: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
struct BookBody {
    file: File,
}

#[unsafe(no_mangle)]
pub extern "C" fn hi(ptr: *mut u8) {
    let text = transfer::u8_to_string(ptr);
    let result = format!("{text} with something extra");
    transfer::write_to_ptr(ptr, &result.to_string());
}

/// Apply `processor` to the incoming cbor data and return the result.
fn handle_cbor<T, F, U>(input: *mut u8, processor: F) -> Result<Vec<u8>, String>
where
    T: serde::de::DeserializeOwned,
    U: serde::Serialize,
    F: Fn(T) -> Result<U, String>,
{
    let input = unsafe { std::slice::from_raw_parts(input, 1024) };
    let data = from_reader::<T, _>(input).map_err(|e| e.to_string())?;
    let output = processor(data)?;

    let mut buf = Vec::new();
    match into_writer(&output, &mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Serialize)]
struct BookBodyResult {
    msg: String,
}

fn book_body_impl(_body: BookBody) -> Result<BookBodyResult, String> {
    Ok(BookBodyResult {
        msg: "done".to_string(),
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn book_body(ptr: *mut u8) {
    let result = handle_cbor::<BookBody, _, _>(ptr, book_body_impl);
    if result.is_err() {
        transfer::write_to_ptr(ptr, "2 error");
        return;
    }
    // transfer::write_to_ptr(ptr, "done");
}

#[unsafe(no_mangle)]
pub extern "C" fn submit_body_upload() {
    // log("submit_body_upload");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p").unwrap();
    val.set_text_content(Some("Hello from Rust!"));
    document.body().unwrap().set_text_content(Some("from wasm"));

    body.append_child(&val).unwrap();
}

pub fn main() {}
