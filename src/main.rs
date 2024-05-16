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

use core::fmt::Write;
#[no_mangle]
extern "C" fn start() -> ! {
    bkpt();
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
