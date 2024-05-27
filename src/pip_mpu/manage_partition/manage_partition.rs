use crate::pip_mpu::manage_partition::partition_items::CreateReturn;

pub fn m_create_partition(
    stack_size: usize,
    entry_point: *const u32,
    used_rom: usize,
    unused_ram: usize,
    unused_rom: usize,
) -> Result<CreateReturn, ()> {
    Err(())
}
