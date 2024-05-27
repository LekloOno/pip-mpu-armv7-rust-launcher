use crate::pip_mpu::core::pip_items::Interface;
use crate::pip_mpu::manage_partition::partition_items::CreateReturn;
use crate::pip_mpu::tools;

pub fn m_create_partition(
    interface: Interface,
    stack_size: usize,
    entry_point: *const u32,
    used_rom: usize,
    unused_ram: usize, // 0 if leaf partition
    unused_rom: usize, // 0 if leaf partition
) -> Result<CreateReturn, ()> {
    let success_output = CreateReturn::new();
    let pd_addr = tools::round((interface.ram_end as u32) - 543); //543 is 512 - 31, to make sure we do have 512 bits after align
    Err(())
}
