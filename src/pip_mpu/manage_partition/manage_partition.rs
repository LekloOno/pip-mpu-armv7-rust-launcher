use crate::pip_mpu::core::pip_items::{BasicContext, BlockOrError, Frame, Interface, VIDT};
use crate::pip_mpu::manage_partition::partition_items::{Block, CreateReturn};
use crate::pip_mpu::rust::pip_rust_mpu;
use crate::pip_mpu::tools;
use core::mem;
use ptr_bits_ops::{MutPtrBitsOps, PtrBitsOps};

pub fn m_create_partition(
    parent_itf: &Interface, //Structure describing the initial parent memory layout.
    parent_ctx: *const BasicContext, //The address of the space where the parent's context lies
    child_ram_block: &Block, //The parent's RAM block to use as child's RAM space.
    pip_block: Option<&Block>, //The parent's RAM block to use for pip's intern structure for the child. If none is specified, pip datas will be placed at the end of child_ram_block
    entry_point: *const u8,    //The entry point in ROM of the child.
    stack_size: usize,         //The desired size of the child's stack
    vidt_size: usize,          //The vidt size, depends on the architecrure. On dwm1001, 512.
    used_rom_size: usize,      //The size of the child's used ROM.
    unused_rom_size: usize,    //The size of the child's unused ROM.
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
    let pd_addr = actual_pip_block_addr
        .add_bits_offset(actual_pip_block_size - 1023)
        .bits_align(512);

    let kern_addr = pd_addr.sub_bits_offset(512);
    let parent_kern_addr = kern_addr.sub_bits_offset(512);
    // MPU BLOCK 0
    let stack_vidt_block_size =
        tools::next_pow_of_2((stack_size + vidt_size).try_into().unwrap()) as usize;

    let stack_addr = child_ram_block.address.bits_align(stack_vidt_block_size); // Set the stack address to the next aligned block with a minimum size of stack_size + vidt_size
    let vidt_addr = stack_addr.add_bits_offset(stack_size);

    // MPU BLOCK 1
    let ctx_itf_block_size =
        tools::next_pow_of_2((mem::size_of::<VIDT>() + mem::size_of::<Interface>()) as u32)
            as usize;

    let ctx_addr = stack_addr
        .add_bits_offset(stack_vidt_block_size)
        .bits_align(ctx_itf_block_size);
    let itf_addr = ctx_addr.add_bits_offset(mem::size_of::<BasicContext>()) as *mut Interface;

    let unused_ram_addr = ctx_addr.add_bits_offset(ctx_itf_block_size);

    // MPU BLOCK 2
    let unused_rom_addr = entry_point.add_bits_offset(used_rom_size);
    let rom_end_addr = unused_rom_addr.add_bits_offset(unused_rom_size);

    tools::memset(parent_itf.vidt_start as *mut u8, 0, mem::size_of::<VIDT>());
    unsafe {
        (*(parent_itf.vidt_start as *mut VIDT)).contexts[0] = parent_ctx as *const u8;
    }
    tools::memset(vidt_addr as *mut u8, 0, mem::size_of::<VIDT>());
    unsafe {
        (*(vidt_addr as *mut VIDT)).contexts[0] = ctx_addr;
    }
    //INIT CHILD INTERFACE
    let ram_end_addr = child_ram_block
        .address
        .add_bits_offset(child_ram_block.size) as *const u8;
    unsafe {
        (*itf_addr).stack_top = vidt_addr.add_bits_offset(4);
        (*itf_addr).stack_limit = stack_addr;
        (*itf_addr).vidt_start = vidt_addr;
        (*itf_addr).vidt_end = vidt_addr.add_bits_offset(512);
        (*itf_addr).entry_point = entry_point;
        (*itf_addr).unused_rom_start = unused_rom_addr;
        (*itf_addr).rom_end = rom_end_addr;
        (*itf_addr).unused_ram_start = unused_ram_addr as *mut u8;
        (*itf_addr).ram_end = ram_end_addr;
    }

    //INIT CHILD CONTEXT
    unsafe {
        (*(ctx_addr as *mut BasicContext))
            .frame
            .set_r0(itf_addr as u32);
        (*(ctx_addr as *mut BasicContext))
            .frame
            .set_pc((entry_point as u32) | 1);
        (*(ctx_addr as *mut BasicContext))
            .frame
            .set_sp(vidt_addr as u32 - 4);
        (*(ctx_addr as *mut BasicContext))
            .frame
            .set_xpsr(0x01000000);
        (*(ctx_addr as *mut BasicContext)).is_basic_frame = 1;
    }

    // __________________________
    //
    // CUT THE PARTITION'S BLOCKS
    // __________________________

    // PIP BLOCKS

    // parent new kernel structure
    let parent_kern_block_id = pip_rust_mpu::cut_memory_block(
        &actual_pip_block_local_id,
        &(parent_kern_addr as *const u32),
        None,
    )
    .unwrap();

    // prepare the parent's kernel structure
    pip_rust_mpu::prepare(
        &(parent_itf.part_desc_block_id as *const u32),
        None,
        &parent_kern_block_id,
    )
    .unwrap();

    // child's partition descriptor
    let pd_block_id =
        pip_rust_mpu::cut_memory_block(&parent_kern_block_id, &(pd_addr as *const u32), None)
            .unwrap();

    // child's first kernel structure
    let kern_block_id =
        pip_rust_mpu::cut_memory_block(&pd_block_id, &(kern_addr as *const u32), None).unwrap();

    // CHILD BLOCKS

    // Ram blocks

    //  * `stack_vidt_vlock_id` is the local id of the given child ram block if its start address is already aligned on stack/vidt size.
    //The first aligned block in child's ram block otherwise. It's the block containing the stack & vidt of the child's partition.
    //  * `ram_head_block_id` is the local id of the left over head if the start_address of the given child's ram block isn't aligned on the vidt/stack block's size.
    //i.e. the block between the child's block start address and the first address aligned on vidt/stack size
    let (stack_vidt_block_id, ram_head_block_id) =
        if stack_addr == child_ram_block.address as *const u8 {
            (child_ram_block.local_id, None)
        } else {
            (
                pip_rust_mpu::cut_memory_block(
                    &child_ram_block.local_id,
                    &(stack_addr as *const u32),
                    None,
                )
                .unwrap(),
                Some(child_ram_block.local_id),
            )
        };

    //The local id of the block containing the context & interface of the partition.
    let ctx_itf_block_id =
        pip_rust_mpu::cut_memory_block(&stack_vidt_block_id, &(ctx_addr as *const u32), None)
            .unwrap();
    //The left over tail once the vidt/stack and context/interface blocks have been cut. General purpose within the child partition.
    let unused_ram_block_id_option = if unused_ram_addr < ram_end_addr {
        Some(
            pip_rust_mpu::cut_memory_block(
                &ctx_itf_block_id,
                &(unused_ram_addr as *const u32),
                None,
            )
            .unwrap(),
        )
    } else {
        None
    };

    // Rom blocks
    let parent_rom_block_attr = pip_rust_mpu::find_block(
        &(parent_itf.part_desc_block_id as *const u32),
        &(entry_point as *const u32),
    )
    .unwrap();

    //  * `rom_block_id` is the local id of one of parent's rom block if its start address is the entry point of the child,
    //The local id of the newly cut block whose start address is the entry point of the child otherwise.
    //  * `rom_head_block_id` is the local id of the left over head if the entry point address is not the start address of the block containing it.
    //i.e. if the parent's block has to be cut
    let (rom_block_id, rom_head_block_id) =
        if parent_rom_block_attr.start_addr == entry_point as *const u32 {
            (parent_rom_block_attr.local_id, None)
        } else {
            (
                pip_rust_mpu::cut_memory_block(
                    &parent_rom_block_attr.local_id,
                    &(entry_point as *const u32),
                    None,
                )
                .unwrap(),
                Some(parent_rom_block_attr.local_id),
            )
        };

    //The left over tail, depending on the requested amount of rom. General purpose within the child partition.
    let unused_rom_id_option = if unused_rom_addr < parent_rom_block_attr.end_addr as *const u8 {
        Some(
            pip_rust_mpu::cut_memory_block(&rom_block_id, &(unused_ram_addr as *const u32), None)
                .unwrap(),
        )
    } else {
        None
    };

    //The left over tail, depending on the requested amount of rom. General purpose within the PARENT partition.
    let rom_end_block_id = if rom_end_addr < parent_rom_block_attr.end_addr as *const u8 {
        Some(
            pip_rust_mpu::cut_memory_block(
                &(unused_rom_id_option.unwrap()),
                &(rom_end_addr as *const u32),
                None,
            )
            .unwrap(),
        )
    } else {
        None
    };

    Err(())
}
