#[no_mangle]
pub extern "C" fn rust_add_one(x: i32) -> i32 {
    x + 1
}

extern "C" {
    fn go_add_two(x: i32) -> i32;
}

pub fn call_go_add_two(x: i32) -> i32 {
    unsafe { go_add_two(x) }
}
