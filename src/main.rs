#![doc = include_str!("../README.md")]
#![no_std]
#![no_main]
#![feature(asm_const)]

mod pip_mpu;

pub mod io;
use core::cell::Cell;
use core::ffi::CStr;
use mini_format::*;

use io::{putc, putcs, puts};

use crate::pip_mpu::core::pip_items::{BasicContext, Interface};
use crate::pip_mpu::manage_partition::manage_partition::{
    m_create_partition, m_delete_partition, m_map_partition,
};
use crate::pip_mpu::rust::pip_rust_items::{Block, BlockId};
use crate::pip_mpu::rust::pip_rust_mpu::find_block;
use core::fmt::Write;
#[no_mangle]
extern "C" fn start(interface: &Interface) -> ! {
    bkpt();
    let root_pd_block_id = BlockId::new(interface.part_desc_block_id as usize);
    let root_ctx: BasicContext = Default::default();
    let block_0 = find_block(
        &root_pd_block_id,
        (interface.unused_ram_start as *const u32),
    )
    .unwrap();

    let size = ((block_0.end_addr as u32) - (block_0.start_addr as u32)) as usize;

    let partition_result = m_create_partition(
        interface,
        &root_ctx as *const BasicContext,
        &block_0,
        None,
        interface.unused_rom_start,
        512,
        512,
        512,
        0,
    )
    .unwrap();

    m_map_partition(&partition_result);
    let delete_result = m_delete_partition(&partition_result);

    loop {}
}

const BKPT: u8 = 0x3;

pub fn bkpt() {
    unsafe {
        core::arch::asm!("bkpt {number}",
        number = const BKPT
                     );
    }
}

#[cfg(not(debug_assertions))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    puts("\n\n\nPANIC\n");
    loop {}
}

#[cfg(debug_assertions)]
#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    puts("\n\n\nPANIC\n\n\n");
    if let Some(location) = panic_info.location() {
        format_dec(location.line(), |c| putc(c as u8));
    }
    loop {}
}
