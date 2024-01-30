mod parsing;
mod runtime;
mod ast;

#[no_mangle]
fn main() {
    println!("Hello, world!");
}

#[no_mangle]
fn malloc(size:usize) -> *mut u8 {
    let mut vec = Vec::with_capacity(size);
    let ptr = vec.as_mut_ptr();
    core::mem::forget(vec);
    return ptr;
}

#[no_mangle]
fn run(ptr:usize, len:usize) -> f64 {
    let wasm_bytes = unsafe {
        Vec::from_raw_parts(ptr, len, len)
    };
    
}