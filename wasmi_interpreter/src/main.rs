//
// Copyright 2023, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;

use wasmi::{Engine, Module, Store, Func, Caller, Linker};

use wasmi_interpreter::{read_from_region, WASM_REGION_SIZE};

use core::ptr::NonNull;
use sel4_externally_shared::{
    access::{ReadOnly, ReadWrite},
    ExternallySharedRef, ExternallySharedRefExt,
};

use sel4_microkit::{
    debug_println, debug_print, memory_region_symbol, protection_domain, Channel, Handler, Infallible, MessageInfo,
};

fn wasm_init(wasm: &[u8]) -> Result<(), wasmi::Error> {
    let engine = Engine::default();
    let module = Module::new(&engine, wasm)?;

    // All Wasm objects operate within the context of a `Store`.
    // Each `Store` has a type parameter to store host-specific data,
    // which in this case we are using `42` for.
    type HostState = u32;
    let mut store = Store::new(&engine, 42);
    let host_hello = Func::wrap(&mut store, |caller: Caller<'_, HostState>, param: i32| {
        debug_println!("Got {param} from WebAssembly");
        debug_println!("My host state is: {}", caller.data());
    });

    let fd_write = Func::wrap(&mut store, |caller: Caller<'_, HostState>, a: i32, b: i32, c: i32, d: i32| -> i32 {
        debug_println!("fd_write call, param: {:?}", (a, b, c, d));

        return 0;
    });

    let environ_get = Func::wrap(&mut store, |caller: Caller<'_, HostState>, a: i32, b: i32| -> i32 {
        debug_println!("environ_get call, param: {:?}", (a, b));

        return 0;
    });
    let environ_sizes_get = Func::wrap(&mut store, |caller: Caller<'_, HostState>, a: i32, b: i32| -> i32 {
        debug_println!("environ_sizes_get call, param: {:?}", (a, b));

        return 0;
    });

    let proc_exit = Func::wrap(&mut store, |caller: Caller<'_, HostState>, a: i32| {
        debug_println!("proc_exit call, param: {:?}", (a));
    });

    // In order to create Wasm module instances and link their imports
    // and exports we require a `Linker`.
    let mut linker = <Linker<HostState>>::new(&engine);
    // Instantiation of a Wasm module requires defining its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    //
    // Also before using an instance created this way we need to start it.
    linker.define("host", "hello", host_hello)?;
    linker.define("wasi_snapshot_preview1", "fd_write", fd_write)?;
    linker.define("wasi_snapshot_preview1", "environ_get", environ_get)?;
    linker.define("wasi_snapshot_preview1", "environ_sizes_get", environ_sizes_get)?;
    linker.define("wasi_snapshot_preview1", "proc_exit", proc_exit)?;
    let instance = linker
        .instantiate(&mut store, &module)?
        .start(&mut store)?;
    let main = instance.get_typed_func::<(), ()>(&store, "_start")?;

    // And finally we can call the wasm!
    main.call(&mut store, ())?;

    Ok(())
}

#[protection_domain(heap_size = 0x400000)]
fn init() -> HandlerImpl {
    debug_println!("WASMI|INFO: start");

    let mut wasm_region = unsafe {
        ExternallySharedRef::new(memory_region_symbol!(wasm_data: *mut [u8], n = WASM_REGION_SIZE))
    };

    HandlerImpl {
        wasm_region
    }
}

struct HandlerImpl {
    wasm_region: ExternallySharedRef<'static, [u8], ReadOnly>,
}

impl Handler for HandlerImpl {
    type Error = Infallible;

    fn notified(&mut self, channel: Channel) -> Result<(), Self::Error> {
        debug_println!("notified!\n");

        let wasm_buf = read_from_region(self.wasm_region);

        wasm_init(&wasm_buf).unwrap();

        Ok(())
    }
}
