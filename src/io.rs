//! Simple IO module that uses arm semihostting
//! to ouput bytes and strings
use core::arch::asm;

const SYS_WRITE0: u8 = 0x4;
const SYS_WRITEC: u8 = 0x3;
const ANGEL_SWI: u8 = 0xab;

use core::ffi::CStr;

/// An object that implements [`core::fmt::Write`]
/// to perform formatted output
///
/// [`core::fmt::Write`]: https://doc.rust-lang.org/core/fmt/trait.Write.html
pub struct Formatter;

impl core::fmt::Write for Formatter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        puts(s);
        Ok(())
    }
}

/// Ouputs a c string using arm semihostting calls.
///
/// This uses the [`SYS_WRITE0`] call and thus requires a C style zero
/// terminated string [`CStr`]
///
/// [`CStr`]: https://doc.rust-lang.org/core/ffi/CStr/index.html
pub fn putcs(s: &CStr) {
    unsafe {
        asm!(
            "movs r0, {syswrite0}",
            "movs r1, {s}",
            "bkpt {angel_swi}",
            syswrite0 = const SYS_WRITE0,
            s = in(reg) s.as_ptr(),
            angel_swi = const ANGEL_SWI,
            out("r0") _,
            out("r1") _,
        );
    }
}

/// Outputs a string using arm semihostting calls.
///
/// It simply iterates over the str slice seen as a bytes slices
/// and calls [`putc`].
///
/// It can thus be easily used with rust [`&str`] type but it very slow
/// due to the multiple semihostting calls (one per byte)
///
/// If you can build a [`CStr`], prefer a call to [`putcs`]
/// that will be much faster
///
/// [`CStr`]: https://doc.rust-lang.org/core/ffi/CStr/index.html
/// [`&str`]: https://doc.rust-lang.org/core/primitive.str.html
pub fn puts(s: &str) {
    s.as_bytes().iter().for_each(|c| putc(*c));
}

/// Ouputs a character byte using semihostting calls.
///
/// This uses the [`SYS_WRITEC`] call. Using this to output
/// a string by looping on bytes would be *very* slow.
pub fn putc(c: u8) {
    unsafe {
        asm!(
            "movs r0, {syswrite0}",
            "movs r1, {c}",
            "bkpt {angel_swi}",
            syswrite0 = const SYS_WRITEC,
            c = in(reg) core::ptr::addr_of!(c),
            angel_swi = const ANGEL_SWI,
            out("r0") _,
            out("r1") _,
        );
    }
}
