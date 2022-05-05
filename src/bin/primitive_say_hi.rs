use std::{env, str};
use wasmtime::*;

/// Run: bin/primitive_say_hi.sh
fn main() -> anyhow::Result<()> {
    let (test_name, test_age) = (b"Han meimei", 12);
    let test_name_len = test_name.len() as u32;

    let wasm: String = env::var("WASM")?;

    let mut store = Store::<()>::default();
    let host_f_stdout = Func::wrap(
        &mut store,
        |mut caller: Caller<'_, ()>, s_ptr: u32, s_len: u32| {
            // dbg!(s_ptr, s_len);
            if let Some(Extern::Memory(mem)) = caller.get_export("memory") {
                let out = str::from_utf8(
                    mem.data(&caller)
                        .get(s_ptr as usize..)
                        .and_then(|arr| arr.get(..s_len as usize))
                        .unwrap(),
                );
                assert_eq!(out.unwrap(), "hi all, i am Han meimei, i am 12 years old.");
                println!("\n👩 {}", out.unwrap());
            }
        },
    );
    let module = Module::from_file(store.engine(), wasm)?;
    let instance = Instance::new(&mut store, &module, &[host_f_stdout.into()])?;

    let wasm_f_alloc = instance.get_typed_func::<u32, u32, _>(&mut store, "wasm_f_malloc")?;
    let test_name_ptr = wasm_f_alloc.call(&mut store, test_name_len)? as usize;
    // dbg!(test_name_ptr);

    let wasm_linear_mem = instance.get_memory(&mut store, "memory").unwrap();
    wasm_linear_mem.write(&mut store, test_name_ptr, test_name)?;

    let wasm_f_say_hi =
        instance.get_typed_func::<(u32, u32, u32), (), _>(&mut store, "wasm_f_say_hi")?;
    wasm_f_say_hi.call(&mut store, (test_name_ptr as u32, test_name_len, test_age))?;

    Ok(())
}
