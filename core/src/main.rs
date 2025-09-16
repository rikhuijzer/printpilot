mod string;

#[unsafe(no_mangle)]
pub extern "C" fn hi(ptr: *mut u8) {
    let text = string::u8_to_string(ptr);
    let result = format!("{text} with something extra");
    string::write_to_ptr(ptr, &result.to_string());
}

pub fn main() {}
