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
///     true if the operation was successful, false otherwise.
pub fn create_partition(block_local_id: &*const u32) -> bool {
    (pip_core_mpu::pip_create_partition(block_local_id) & 1) == 1
}

/// Brief.
///     Cuts the given memory block
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
///     An Option which's Some variant contains the newly created subblock's id

pub fn cut_memory_block(
    block_to_cut_local_id: &*const u32,
    cut_addr: &*const u32,
    mpu_region_nb: i32,
) -> Option<*const u32> {
    let new_addr =
        pip_core_mpu::pip_cut_memory_block(block_to_cut_local_id, cut_addr, mpu_region_nb);
    new_addr.is_null().then(|| new_addr)
}
