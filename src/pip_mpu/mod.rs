mod pip_items;
use core::arch::asm;

#[inline]
pub fn pip_create_partition(block_local_id: &*const u32) -> u32 {
    let r0: u32;
    unsafe {
        //no memory clobber, it is default behavior in rust
        asm!(
            "svc #0",
            inout("r0") block_local_id => r0,
        );
    }

    r0
}

#[inline]
pub fn pip_cut_memory_block(
    block_to_cut_local_id: &*const u32,
    cut_addr: &*const u32,
    mpu_region_nb: i32,
) -> *const u32 {
    let r0: *const u32;
    unsafe {
        asm!(
            "svc #1",
            inout("r0") block_to_cut_local_id => r0,
            in("r1") cut_addr,
            in("r2") mpu_region_nb,
        )
    }

    r0
}

#[inline]
pub fn pip_merge_memory_blocks(
    block_to_merge_1_local_id: *const u32,
    block_to_merge_2_local_id: *const u32,
    mpu_region_nb: i32,
) -> *const u32 {
    let r0: *const u32;
    unsafe {
        asm!(
            "svc #2",
            inout("r0") block_to_merge_1_local_id => r0,
            in("r1") block_to_merge_2_local_id,
            in("r2") mpu_region_nb,
        )
    }

    r0
}

#[inline]
pub fn pip_prepare(
    part_desc_block_id: *const u32,
    projected_slots_nb: i32,
    requisitionned_block_local_id: *const u32,
) -> u32 {
    let r0: u32;
    unsafe {
        asm!(
            "svc #3",
            inout("r0") part_desc_block_id => r0,
            in("r1") projected_slots_nb,
            in("r2") requisitionned_block_local_id,
        )
    }

    r0
}
