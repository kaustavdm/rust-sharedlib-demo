extern crate sharedlib;

use std::path::PathBuf;
use std::env;
use sharedlib::{Lib, Func, Symbol};

#[cfg(any(target_os = "linux",
          target_os = "freebsd",
          target_os = "dragonfly",
          target_os = "redox"))]
fn lib_name(name: String) -> String {
    format!("target/lib{}.so", name)
}

#[cfg(target_os = "macos")]
fn lib_name(name: String) -> String {
    format!("target/lib{}.dylib", name)
}

#[cfg(target_os = "windows")]
fn lib_name(name: String) -> String {
    format!("target/lib{}.dll", name)
}

fn get_lib(name: String) -> Lib {
    let libpath = PathBuf::from(lib_name(name));
    unsafe { Lib::new(&libpath).unwrap() }
}

fn get_num(lib: Lib) -> i32 {
    let hello_sym: Func<fn() -> i32> = unsafe { lib.find_func("hello\0").unwrap() };

    unsafe {
        let hello = hello_sym.get();
        hello()
    }
}

fn main() {
    let name = env::args().nth(1).unwrap_or("hello".to_string());

    println!("Loading: {}", &name);

    let expected = match name.as_ref() {
        "hello" => 100,
        "bar" => 84,
        "foo" | _ => 42,
    };

    // Do the loading
    let lib = get_lib(name);
    let hello = get_num(lib);

    println!("Hello is: {}", &hello);
    assert_eq!(hello, expected);
}
