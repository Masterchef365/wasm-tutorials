use std::fmt::Write;

use wasm_bridge::*;

static GUEST_BYTES: &[u8] =
    include_bytes!("../../rust-guest/target/wasm32-unknown-unknown/release/rust_guest.wasm");

pub async fn get_text() -> String {
    let mut store = Store::<()>::default();

    let module = new_module_async(&store.engine(), GUEST_BYTES)
        .await
        .expect("should create module");

    let mut linker = Linker::new(store.engine());

    linker
        .func_wrap("imported_fns", "add_one", |_: Caller<()>, num: i32| num + 1)
        .expect("should add import function");

    let instance = instantiate_async(&mut store, &linker, &module)
        .await
        .expect("should create instance");

    let mut text = String::new();

    let Some(mem) = instance.get_memory(&mut store, "memory") else {
        writeln!(text, "Ain't go no memory").unwrap();
        return text;
    };
    //println!("Hewwo???");

    let rate_number = instance
        .get_typed_func::<i32, u32>(&mut store, "add_three")
        .expect("should get add_three exported function");

    let result = rate_number
        .call(&mut store, 42)
        .expect("should call add_three");

    writeln!(text, "Result: {result:X}").unwrap();

    let mut buf = vec![0; 10];

    if let Err(e) = mem.read(&mut store, result as _, &mut buf) {
        // Handle error
        writeln!(text, "{}", e).unwrap();
        return text;
    }

    writeln!(text, "And now the bytes:").unwrap();

    for row in buf.chunks(50) {
        for num in row {
            write!(text, "{} ", num).unwrap();
        }
        writeln!(text).unwrap();
    }


    text
}
