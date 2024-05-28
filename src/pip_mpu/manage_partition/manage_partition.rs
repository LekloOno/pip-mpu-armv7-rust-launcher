use crate::pip_mpu::core::pip_items::{BasicContext, Interface, VIDT};
use crate::pip_mpu::manage_partition::partition_items::CreateReturn;
use crate::pip_mpu::tools;
use core::mem;

pub fn m_create_partition(
    interface: Interface,
    root_ctx: *const BasicContext,
    stack_size: usize,
    vidt_size: usize,
    entry_point: *const u32,
    used_rom: usize,
    unused_ram: usize, // 0 if leaf partition
    unused_rom: usize, // 0 if leaf partition
) -> Result<CreateReturn, ()> {
    let success_output = CreateReturn::new();
    let pd_addr = tools::round((interface.ram_end as u32) - 1023, 512); //1023 is 512 - 511, to make sure we do have 512 bits after align
    let kern_addr = pd_addr - 512;
    let stack_addr = tools::round(
        interface.unused_ram_start as u32,
        tools::next_pow_of_2((stack_size + vidt_size).try_into().unwrap()),
    );
    let vidt_addr = stack_addr + stack_size as u32;
    tools::memset(
        interface.vidt_start as *mut u8,
        0,
        mem::size_of::<Interface>(),
    );
    unsafe {
        (*(interface.vidt_start as *mut VIDT)).contexts[0] = root_ctx as *const u8;
    }
    Err(())
}
