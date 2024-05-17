use crate::pip_mpu::core::pip_core_mpu;

/// Brief. 
///     Creates a new child
/// 
/// Description
///     The [createPartition] system call creates a new child (sub-partition of the
///     current partition), e.g. initializes the block <idBlock> as a PD block and
///     sets the current partition as the parent partition.
///
/// *   `block_local_id` - The block to become the child partition descriptor
///
/// Returns
///     true if the operation was successful, false otherwise.
pub fn create_partition(block_local_id: &*const u32) -> bool {
    (pip_core_mpu::pip_create_partition(block_local_id) & 1) == 1
}