mod pip_items;
use core::arch::asm;

#[inline]
pub fn pip_create_partition(mut block_local_id: *const u32) -> *const u32 {
    let r0: u32;
    unsafe {
        //no memory clobber, it is default behavior in rust
        asm!(
            "svc #0",
            inout("r0") block_local_id,
        );
    }

    block_local_id
}
