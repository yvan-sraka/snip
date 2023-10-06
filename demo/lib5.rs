use std::ffi::{CString, c_char, CStr};

#[no_mangle] // Preserve symbol from mangling (randomization)
pub extern "C" fn add(left: i32, right: i32) -> i32 {
//  ~~~~~~~~~~
//  Tell Rust to use the C Calling-Convention
//    (rather that the Rust one which is unstable!)
    return left + right;
}

#[no_mangle]
pub extern "C"
fn concat(left: *const c_char, right: *const c_char) -> *const c_char {
    //          ~~~~~~~~~~~~~         ~~~~~~~~~~~~~     ~~~~~~~~~~~~~
    //          Use FFI-Safe types (compatible with C Calling-Convention)
    //            ... rather than Rust String!
    let left: String = c_char_to_string(left);
    let right: String = c_char_to_string(right);

    let result: String = format!("{}{}", left, right);
    return string_to_c_char(result);
}

// *** Boring conversion functions *** //

fn c_char_to_string(s: *const c_char) -> String {
    let c_str: &CStr = unsafe { CStr::from_ptr(s) };
    let r_str: &str = c_str.to_str().unwrap();
    return r_str.to_string();
}

fn string_to_c_char(s: String) -> *const c_char {
    let c_str = CString::new(s).unwrap();
    let p = c_str.as_ptr();
    return p;
}
