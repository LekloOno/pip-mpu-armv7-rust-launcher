use crate::pip_mpu::core::pip_core_mpu;

pub fn create_partition(block_local_id: &*const u32) -> bool {
    (pip_core_mpu::pip_create_partition(block_local_id) & 1) == 1
}
