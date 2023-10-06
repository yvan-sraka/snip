#[no_mangle] // Preserve symbol from mangling (randomization)
pub fn add(left: i32, right: i32) -> i32 {
    return left + right;
}

#[no_mangle]
pub fn concat(left: String, right: String) -> String {
    return format!("{}{}", left, right);
}
