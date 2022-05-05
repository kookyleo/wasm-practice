extern "C" {
    fn host_f_stdout(ptr: *const u8, len: usize);
}

#[no_mangle]
pub fn wasm_f_say_hi(name_ptr: *mut u8, name_len: u8, age: u8) {
    let name = unsafe { Vec::from_raw_parts(name_ptr, name_len as usize, name_len as usize) };
    let name = String::from_utf8(name).unwrap();
    let greeting = format!("hi all, i am {}, i am {} years old.", name, age);
    let greeting = greeting.as_bytes();
    unsafe {
        host_f_stdout(greeting.as_ptr(), greeting.len());
    }
}

#[no_mangle]
pub fn wasm_f_malloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    return ptr;
}

// #[no_mangle]
// pub fn wasm_f_free(ptr: *mut u8, size: usize) {
//     std::mem::drop(unsafe { Vec::from_raw_parts(ptr, size, size) });
// }
