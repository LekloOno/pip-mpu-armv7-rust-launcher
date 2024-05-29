use crate::pip_mpu::core::pip_items::{BasicContext, BlockOrError, Frame, Interface, VIDT};
use crate::pip_mpu::manage_partition::partition_items::{Block, CreateReturn};
use crate::pip_mpu::rust::pip_rust_mpu;
use crate::pip_mpu::tools;
use core::mem;

pub fn m_create_partition(
    root_itf: Interface,
    child_ram_block: Block,
    child_rom_block: Block,
    pip_block: Option<Block>, //If none is specified, pip datas will be placed at the end of child_ram_block
    root_ctx: *const BasicContext,
    stack_size: usize,
    vidt_size: usize,
    entry_point: *const u32,
    used_rom: usize,
    unused_ram: usize, // 0 if leaf partition
    unused_rom: usize, // 0 if leaf partition
) -> Result<CreateReturn, ()> {
    let success_output = CreateReturn::new();

    let (actual_pip_block_addr, actual_pip_block_size, actual_pip_block_local_id) = match pip_block
    {
        None => (
            child_ram_block.address,
            child_ram_block.size,
            child_ram_block.local_id,
        ),
        Some(block) => (block.address, block.size, block.local_id),
    };

    //Base pip datas address depending on wether or not a block has already been cut to contain them.
    let pd_addr = tools::round(
        (actual_pip_block_addr.wrapping_add(actual_pip_block_size) as u32) - 1023,
        512,
    ) as *const u8;

    let kern_addr = pd_addr.wrapping_sub(512);
    let root_kern_addr = kern_addr.wrapping_sub(512);
    // MPU BLOCK 0
    let stack_vidt_block_size = tools::next_pow_of_2((stack_size + vidt_size).try_into().unwrap());

    let stack_addr =
        tools::round(child_ram_block.address as u32, stack_vidt_block_size) as *const u8; // Set the stack address to the next aligned block with a minimum size of stack_size + vidt_size
    let vidt_addr = stack_addr.wrapping_add(stack_size);

    // MPU BLOCK 1
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
    let text_addr = child_rom_block.address as *const u8;
    let unused_rom_addr = text_addr.wrapping_add(used_rom);
    let rom_end = unused_rom_addr.wrapping_add(unused_rom);

    tools::memset(root_itf.vidt_start as *mut u8, 0, mem::size_of::<VIDT>());
    unsafe {
        (*(root_itf.vidt_start as *mut VIDT)).contexts[0] = root_ctx as *const u8;
    }
    tools::memset(vidt_addr as *mut u8, 0, mem::size_of::<VIDT>());
    unsafe {
        (*(vidt_addr as *mut VIDT)).contexts[0] = ctx_addr;
    }
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
    unsafe {
        (*(ctx_addr as *mut BasicContext))
            .frame
            .set_r0(itf_addr as u32);
        (*(ctx_addr as *mut BasicContext))
            .frame
            .set_pc((child_rom_block.address as u32) | 1);
        (*(ctx_addr as *mut BasicContext))
            .frame
            .set_sp(vidt_addr as u32 - 4);
        (*(ctx_addr as *mut BasicContext))
            .frame
            .set_xpsr(0x01000000);
        (*(ctx_addr as *mut BasicContext)).is_basic_frame = 1;
    }

    let root_block_id_1: *const u32 = pip_rust_mpu::find_block(
        &(root_itf.part_desc_block_id as *const u32),
        &(root_kern_addr as *const u32),
    )
    .unwrap()
    .local_id;
    let root_kernel_id = pip_rust_mpu::cut_memory_block(
        &actual_pip_block_local_id,
        &(root_kern_addr as *const u32),
        None,
    )
    .unwrap();

    Err(())
}
