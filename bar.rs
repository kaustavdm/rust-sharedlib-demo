#![crate_type = "dylib"]

use std::collections::HashMap;

fn num() -> i32 {
    println!("In num in bar");
    84
}

#[no_mangle]
pub fn hashfoo() -> HashMap<i32, i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(0, 1);
    map.insert(1, 2);
    map
}

#[no_mangle]
pub fn hello() -> i32 {
    println!("In bar");
    num()
}
