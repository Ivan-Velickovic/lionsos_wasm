//
// Copyright 2023, Colias Group, LLC
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![no_main]

use wasmi_interpreter::{WASM_REGION_SIZE};

use core::ptr::NonNull;

use sel4_externally_shared::{
    access::{ReadOnly, ReadWrite},
    ExternallySharedRef, ExternallySharedRefExt,
};

use sel4_microkit::{
    debug_print, debug_println, memory_region_symbol, protection_domain, Channel, Handler, Infallible, MessageInfo,
};

const INTERPRETER: Channel = Channel::new(0);

#[protection_domain(heap_size = 0x400000)]
fn init() -> HandlerImpl {
    debug_println!("hello world");

    let mut wasm_region: ExternallySharedRef<'static, [u8], ReadWrite> = unsafe {
        ExternallySharedRef::new(memory_region_symbol!(wasm_data: *mut [u8], n = WASM_REGION_SIZE))
    };

    let wasm = include_bytes!(concat!(env!("BUILD"), "/wasi_test.wasm"));

    wasmi_interpreter::write_to_region(wasm, wasm_region);

    INTERPRETER.notify();

    HandlerImpl {}
}

struct HandlerImpl {
}

impl Handler for HandlerImpl {
    type Error = Infallible;

}
