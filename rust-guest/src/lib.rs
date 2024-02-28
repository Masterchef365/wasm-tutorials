#[no_mangle]
pub fn add_three(number: i32) -> u32 {
    let b = vec![10, number as u8, 2, 30, 23, 234, 15, 6, 61, 31];
    //let number = unsafe { add_one(number) };
    //let number = unsafe { add_one(number) };
    //let number = unsafe { add_one(number) };
    let ptr = b.as_ptr() as _;
    std::mem::forget(b);
    ptr
}

#[link(wasm_import_module = "imported_fns")]
extern "C" {
    fn add_one(number: i32) -> i32;
}
