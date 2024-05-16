mod pip_items;
use crate::pip_mpu::pip_items::pip_items::BlockOrError;
use core::arch::asm;

#[inline]
pub fn pip_create_partition(block_local_id: &*const u32) -> u32 {
    let could_create_partition: u32;
    unsafe {
        //no memory clobber, it is default behavior in rust
        asm!(
            "svc #0",
            inout("r0") block_local_id => could_create_partition,
        );
    }

    could_create_partition
}

#[inline]
pub fn pip_cut_memory_block(
    block_to_cut_local_id: &*const u32,
    cut_addr: &*const u32,
    mpu_region_nb: i32,
) -> *const u32 {
    let id_new_sub_block: *const u32;
    unsafe {
        asm!(
            "svc #1",
            inout("r0") block_to_cut_local_id => id_new_sub_block,
            in("r1") cut_addr,
            in("r2") mpu_region_nb,
        );
    }

    id_new_sub_block
}

#[inline]
pub fn pip_merge_memory_blocks(
    block_to_merge_1_local_id: &*const u32,
    block_to_merge_2_local_id: &*const u32,
    mpu_region_nb: i32,
) -> *const u32 {
    let id_block_to_merge_1: *const u32;
    unsafe {
        asm!(
            "svc #2",
            inout("r0") block_to_merge_1_local_id => id_block_to_merge_1,
            in("r1") block_to_merge_2_local_id,
            in("r2") mpu_region_nb,
        );
    }

    id_block_to_merge_1
}

#[inline]
pub fn pip_prepare(
    part_desc_block_id: &*const u32,
    projected_slots_nb: i32,
    requisitionned_block_local_id: &*const u32,
) -> u32 {
    let could_prepare: u32;
    unsafe {
        asm!(
            "svc #3",
            inout("r0") part_desc_block_id => could_prepare,
            in("r1") projected_slots_nb,
            in("r2") requisitionned_block_local_id,
        );
    }

    could_prepare
}

#[inline]
pub fn pip_add_memory_block(
    child_part_desc_block_local_id: &*const u32,
    block_to_share_local_id: &*const u32,
    r: u32,
    w: u32,
    e: u32,
) -> *const u32 {
    let block_to_share_child_entry_addr: *const u32;
    unsafe {
        asm!(
            "svc #4",
            inout("r0") child_part_desc_block_local_id => block_to_share_child_entry_addr,
            in("r1") block_to_share_local_id,
            in("r2") ((r & 1) << 2) | ((w & 1) << 1) | (e & 1),
        );
    }

    block_to_share_child_entry_addr
}

#[inline]
pub fn pip_remove_memory_block(block_to_remove_local_id: &*const u32) -> u32 {
    let could_remove_memory_block: u32;
    unsafe {
        asm!(
            "svc #5",
            inout("r0") block_to_remove_local_id => could_remove_memory_block,
        );
    }

    could_remove_memory_block
}

#[inline]
pub fn pip_delete_partition(child_part_desc_block_local_id: &*const u32) -> u32 {
    let could_delete_partition: u32;
    unsafe {
        asm!(
            "svc #6",
            inout("r0") child_part_desc_block_local_id => could_delete_partition,
        );
    }

    could_delete_partition
}

#[inline]
pub fn pip_collect(part_desc_block_id: &*const u32) -> *const u32 {
    let collected_structure_block_id: *const u32;
    unsafe {
        asm!(
            "svc #7",
            inout("r0") part_desc_block_id => collected_structure_block_id,
        );
    }

    collected_structure_block_id
}

#[inline]
pub fn pip_map_mpu(
    part_desc_block_id: &*const u32,
    block_to_map_local_id: &*const u32,
    mpu_region_nb: i32,
) -> u32 {
    let could_map_mpu: u32;
    unsafe {
        asm!(
            "svc #8",
            inout("r0") part_desc_block_id => could_map_mpu,
            in("r1") block_to_map_local_id,
            in("r2") mpu_region_nb,
        );
    }

    could_map_mpu
}

#[inline]
pub fn pip_read_mpu(part_desc_block_id: &*const u32, mpu_region_nb: i32) -> *const u32 {
    let id_block: *const u32;
    unsafe {
        asm!(
            "svc #9",
            inout("r0") part_desc_block_id => id_block,
            in("r1") mpu_region_nb,
        );
    }

    id_block
}

#[inline]
pub fn pip_find_block(
    part_desc_block_id: &*const u32,
    addr_in_block: &*const u32,
    block_addr: &*const BlockOrError,
) -> u32 {
    let could_find_block: u32;
    unsafe {
        asm!(
            "svc #10",
            inout("r0") part_desc_block_id => could_find_block,
            in("r1") addr_in_block,
            in("r2") block_addr,
        );
    }

    could_find_block
}

#[inline]
pub fn pip_set_vidt(part_desc_block_id: &*const u32, vidt_block_local_id: &*const u32) -> u32 {
    let vidt_block_added: u32;
    unsafe {
        asm!(
            "svc #11",
            inout("r0") part_desc_block_id => vidt_block_added,
            in("r1") vidt_block_local_id,
        );
    }

    vidt_block_added
}

#[inline]
pub fn pip_yield(
    callee_part_desc_block_id: &*const u32,
    user_target_interrupt: u32,
    user_caller_context_save_index: u32,
    flags_on_yield: u32,
    flags_on_wake: u32,
) -> u32 {
    let could_yield: u32;
    unsafe {
        asm!(
            "svc #12",
            inout("r0") callee_part_desc_block_id => could_yield,
            in("r1") user_target_interrupt,
            in("r2") user_caller_context_save_index,
            in("r3") flags_on_yield,
            in("r4") flags_on_wake,
        );
    }

    could_yield
}

#[inline]
pub fn pip_get_int_state(child_part_desc_block_local_id: *const u32) -> u32 {
    let interrupt_state: u32;
    unsafe {
        asm!(
            "svc #13",
            inout("r0") child_part_desc_block_local_id => interrupt_state,
        );
    }

    interrupt_state
}

#[inline]
pub fn pip_get_self_int_state() -> u32 {
    let interrupt_state: u32;
    unsafe {
        asm!(
            "svc #14",
            out("r0") interrupt_state,
        );
    }

    interrupt_state
}

#[inline]
pub fn pip_set_int_state(interrupt_state: u32) {
    unsafe {
        asm!(
            "svc #15",
            in("r0") interrupt_state,
        );
    }
}
