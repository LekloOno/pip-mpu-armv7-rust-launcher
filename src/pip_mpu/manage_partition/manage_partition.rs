use crate::pip_mpu::core::pip_items::{BasicContext, Interface, VIDT};
use crate::pip_mpu::manage_partition::partition_items::CreateReturn;
use crate::pip_mpu::tools;
use core::mem;

pub fn m_create_partition(
    root_itf: Interface,
    root_ctx: *const BasicContext,
    stack_size: usize,
    vidt_size: usize,
    entry_point: *const u32,
    used_rom: usize,
    unused_ram: usize, // 0 if leaf partition
    unused_rom: usize, // 0 if leaf partition
) -> Result<CreateReturn, ()> {
    let success_output = CreateReturn::new();
    let pd_addr = tools::round((root_itf.ram_end as u32) - 1023, 512) as *const u8; //1023 is 512 + 511, to make sure we do have 512 bits after align
    let kern_addr = pd_addr.wrapping_sub(512);

    // MPU BLOCK 0
    let stack_vidt_block_size = tools::next_pow_of_2((stack_size + vidt_size).try_into().unwrap());

    let stack_addr =
        tools::round(root_itf.unused_ram_start as u32, stack_vidt_block_size) as *const u8; // Set the stack address to the next aligned block with a minimum size of stack_size + vidt_size
    let vidt_addr = stack_addr.wrapping_add(stack_size);

    // MPU BLOCK 1
    tools::memset(root_itf.vidt_start as *mut u8, 0, mem::size_of::<VIDT>());
    unsafe {
        (*(root_itf.vidt_start as *mut VIDT)).contexts[0] = root_ctx as *const u8;
    }
    let ctx_itf_block_size =
        tools::next_pow_of_2((mem::size_of::<VIDT>() + mem::size_of::<Interface>()) as u32)
            as usize;

    let ctx_addr = tools::round(
        stack_addr as u32 + stack_vidt_block_size,
        ctx_itf_block_size as u32,
    ) as *const u8;
    let itf_addr = ctx_addr.wrapping_add(mem::size_of::<BasicContext>()) as *mut Interface;

    let unused_ram_addr = ctx_addr.wrapping_add(ctx_itf_block_size);
    let ram_end = unused_ram_addr.wrapping_add(unused_ram);

    // MPU BLOCK 2
    let text_addr = root_itf.unused_rom_start;
    let unused_rom_addr = text_addr.wrapping_add(used_rom);
    let rom_end = unused_rom_addr.wrapping_add(unused_rom);

    //INIT CHILD INTERFACE
    unsafe {
        (*itf_addr).stack_top = vidt_addr.wrapping_sub(4);
        (*itf_addr).stack_limit = stack_addr;
        (*itf_addr).vidt_start = vidt_addr;
        (*itf_addr).vidt_end = vidt_addr.wrapping_add(512);
        (*itf_addr).entry_point = text_addr;
        (*itf_addr).unused_rom_start = unused_rom_addr;
        (*itf_addr).rom_end = rom_end;
        (*itf_addr).unused_ram_start = unused_ram_addr as *mut u8;
        (*itf_addr).ram_end = ram_end;
    }

    //INIT CHILD CONTEXT
    tools::memset(vidt_addr as *mut u8, 0, mem::size_of::<VIDT>());
    unsafe {
        (*(vidt_addr as *mut VIDT)).contexts[0] = ctx_addr;
    }

    Err(())
}
