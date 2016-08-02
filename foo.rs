#![crate_type = "dylib"]

fn get_num() -> i32 {
    42
}

#[no_mangle]
pub fn hello() -> i32 {
    get_num()
}
