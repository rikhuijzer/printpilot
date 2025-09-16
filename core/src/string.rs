///
/// Logic to transfer strings between Javascript and WebAssembly.
/// Thanks to Richard L. Apodaca at https://depth-first.com.
///
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_void;

#[unsafe(no_mangle)]
pub extern "C" fn alloc() -> *mut c_void {
    let mut buf = Vec::with_capacity(1024);
    let ptr = buf.as_mut_ptr();

    mem::forget(buf);

    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dealloc(ptr: *mut c_void) {
    unsafe {
        Vec::from_raw_parts(ptr, 0, 1024);
    }
}

pub fn u8_to_string(ptr: *mut u8) -> String {
    let mut text = unsafe { CStr::from_ptr(ptr as *const i8) }
        .to_str()
        .unwrap()
        .to_string();
    match text.find("<END>") {
        Some(index) => text.truncate(index),
        None => panic!("Expected <END> but couldn't find it."),
    }
    text
}

pub fn write_to_ptr(ptr: *mut u8, text: &str) {
    let c_headers = CString::new(text).unwrap();
    let bytes = c_headers.as_bytes_with_nul();
    let header_bytes = unsafe { std::slice::from_raw_parts_mut(ptr, 1024) };
    header_bytes[..bytes.len()].copy_from_slice(bytes);
}

pub fn _json(text: &str) -> Option<Value> {
    match serde_json::from_str(text) {
        Ok(parsed) => Some(parsed),
        Err(error) => {
            eprintln!("Failed to parse json: {error:?}");
            None
        }
    }
}
