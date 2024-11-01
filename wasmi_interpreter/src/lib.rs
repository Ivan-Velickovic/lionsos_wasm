#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;

use sel4_externally_shared::{
    access::{ReadOnly, ReadWrite},
    ExternallySharedRef,
};

pub const WASM_REGION_SIZE: usize = 1024 * 1024 * 4;

const DATA_OFFSET: usize = core::mem::size_of::<u64>();

pub fn write_to_region(wasm: &[u8], mut region: ExternallySharedRef<'static, [u8], ReadWrite>) {
    let size_of_u64 = core::mem::size_of::<u64>();
    let size = (wasm.len() as u64).to_le_bytes();

    region.as_mut_ptr().index(0..size_of_u64).copy_from_slice(&size);
    region.as_mut_ptr().index(size_of_u64..size_of_u64 + wasm.len()).copy_from_slice(wasm);
}

pub fn read_from_region(region: ExternallySharedRef<'static, [u8], ReadOnly>) -> Box<[u8]> {
    let mut size_bytes: [u8; DATA_OFFSET] = [0; DATA_OFFSET];
    region.as_ptr().index(0..DATA_OFFSET).copy_into_slice(&mut size_bytes);
    let size: usize = u64::from_le_bytes(size_bytes).try_into().unwrap();

    let mut wasm_buf = vec![0; size];

    region.as_ptr().index(DATA_OFFSET..DATA_OFFSET + size).copy_into_slice(&mut wasm_buf);

    wasm_buf.into_boxed_slice()
}
