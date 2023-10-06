#[no_mangle] // Preserve symbol from mangling (randomization)
pub extern "C" fn add(left: i32, right: i32) -> i32 {
    return left + right;
}

#[no_mangle]
pub extern "C" fn concat(left: String, right: String) -> String {
//  ~~~~~~~~~~
//  Tell Rust to use the C Calling-Convention
//    (rather that the Rust one which is unstable!)
    return format!("{}{}", left, right);
}
