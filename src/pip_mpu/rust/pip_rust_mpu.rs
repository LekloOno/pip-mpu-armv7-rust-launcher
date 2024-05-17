use crate::pip_mpu::core::pip_core_mpu;

/// Brief.
///     Creates a new child
///
/// Description.
///     The [createPartition] system call creates a new child (sub-partition of the
///     current partition), e.g. initializes the block `block_local_id` as a PD block and
///     sets the current partition as the parent partition.
///
/// *   `block_local_id` - The block to become the child partition descriptor
///
/// Returns
///     An Ok Result if the operation was successful, an emtpy Err otherwise.
pub fn create_partition(block_local_id: &*const u32) -> Result<&*const u32, ()> {
    if (pip_core_mpu::pip_create_partition(block_local_id) & 1) == 1 {
        Ok(block_local_id)
    } else {
        Err(())
    }
}

/// Brief.
///     Cuts the given memory block.
///
/// Description.
///     The [cutMemoryBlock] system call cuts the memory block `block_to_cut_local_id`
/// 	at `cut_addr` which creates a new subbblock at that address.
///     The new subblock is placed in the physical MPU region of the current partition
///     if the `mpu_region_nb` is a valid region number.
///
/// *   `block_to_cut_local_id` - The local id of the block to cut
/// *   `cut_addr`              - The adress at which to create the new subblock
/// *   `mpu_region_nb`         - The mpu region number
///
/// Returns
///     A Some Option containing the newly created subblock's id if the operation was successful, None otherwise.
pub fn cut_memory_block(
    block_to_cut_local_id: &*const u32,
    cut_addr: &*const u32,
    mpu_region_nb: i32,
) -> Option<*const u32> {
    let new_addr =
        pip_core_mpu::pip_cut_memory_block(block_to_cut_local_id, cut_addr, mpu_region_nb);
    new_addr.is_null().then(|| new_addr)
}

/// Brief.
///     Merge the given memory blocks to the given mpu region number.
///
/// Description.
///     The [mergeMemoryBlocks] system call merges `block_to_merge_1_local_id` and
///     `block_to_merge_2_local_id` together.
///     The two blocks have been cut before so @block_to_merge_1_local_id < @block_to_merge_2_local_id.
///     The merged block is placed in the physical MPU region of the current partition
///     if the `mpu_region_nb` is a valid region number.
///
/// *   `block_to_merge_1_local_id` - The local id of the first block to merge
/// *   `block_to_merge_2_local_id` - The local id of the second block to merge
/// *   `mpu_region_nb`             - The mpu regio number
///
/// Returns
///     A Some Option containing the newly created merged block id if the operation is successful, None otherwise.
pub fn merge_memory_blocks(
    block_to_merge_1_local_id: &*const u32,
    block_to_merge_2_local_id: &*const u32,
    mpu_region_nb: i32,
) -> Option<*const u32> {
    let new_addr = pip_core_mpu::pip_merge_memory_blocks(
        block_to_merge_1_local_id,
        block_to_merge_2_local_id,
        mpu_region_nb,
    );
    new_addr.is_null().then(|| new_addr)
}

/// Brief.
///     Prepares the current partition or child to receive a projected_slots_nb of blocks
///     
/// Description.
///     The [prepare] system call prepares the partition of `part_desc_block_id`
///		(current partition or child) to receive `projected_slots_nb` of blocks and use the
///		`requisitionned_block_local_id` as a metadata structure IF NEEDED => if there isn't enough
///                                                                     free slots in the current 
///                                                                     kernel structure.
///
///     e.g. this will prepare `requisitionned_block_local_id` to be a kernel structure added to the
///		kernel structure list of the partition `part_desc_block_id`
///        - if enough free slots to receive `projected_slots_nb` then won't do anything
///				- if not enough free slots then prepare the block
///        - if `projected_slots_nb` not specified then prepare the block whatever the nb of
///					free slots
///
/// *   `part_desc_block_id`            - The block to prepare within the current or child partition
/// *   `projected_slots_nb`            - The number of requested slots, None to force prepare
/// *   `requisitionned_block_local_id` - The block used as the new kernel structure
///
/// Returns
///     Ok() if the operation is valid, Err() Otherwise
///     TODO :
///     -   Ok(bool) - contains true if the requisitionned block was used, false otherwise
///     -   Err() - ..... Many cases to determine
pub fn prepare(
    part_desc_block_id: &*const u32,
    projected_slots_nb: Option<i32>,
    requisitionned_block_local_id: &*const u32,
) -> Result<(), ()> {
    let valid = pip_core_mpu::pip_prepare(
        part_desc_block_id,
        projected_slots_nb.unwrap_or_else(|| -1),
        requisitionned_block_local_id,
    ) & 1
        == 1;

    valid.then(|| {}).ok_or(())
}
