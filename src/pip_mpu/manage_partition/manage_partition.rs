use crate::pip_mpu::core::pip_items::{BasicContext, BlockOrError, Frame, Interface, VIDT};
use crate::pip_mpu::manage_partition::partition_items::{CreateReturn, Parent, Partition};
use crate::pip_mpu::rust::pip_rust_items::{Block, BlockId};
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
    let parent_pd_block_id = BlockId::new(parent_itf.part_desc_block_id as usize);
    // ________________________________
    //
    // PREPARE AND INITIALIZE ADDRESSES
    // ________________________________

    // PIP

    //Find the real block used for pip datas.
    //The pip blocks will be cut within the block given in parameters of m_create_partition.
    //If this block is None, they will be cut within the general child ram block.
    let (actual_pip_block_addr, actual_pip_block_size, actual_pip_block_local_id) = match pip_block
    {
        None => (
            child_ram_block.start_addr,
            child_ram_block.size(),
            child_ram_block.local_id,
        ),
        Some(block) => (block.start_addr, block.size(), block.local_id),
    };

    //Child partition descriptor address
    let pd_addr = actual_pip_block_addr
        .add_bits_offset(actual_pip_block_size - 1023)
        .bits_align(512);

    //Child first kernel structure address
    let kern_addr = pd_addr.sub_bits_offset(512);

    //Parent new kernel structure address
    let parent_kern_addr = kern_addr.sub_bits_offset(512);

    // CHILD

    // Stack and vidt - Always a physical block
    let stack_vidt_block_size =
        tools::next_pow_of_2((stack_size + vidt_size).try_into().unwrap()) as usize;

    let stack_addr = child_ram_block.start_addr.bits_align(stack_vidt_block_size); // Set the stack address to the next aligned block with a minimum size of stack_size + vidt_size
    let vidt_addr = stack_addr.add_bits_offset(stack_size);

    // Context and interface - Might be a virtual block
    let ctx_itf_block_size = mem::size_of::<VIDT>() + mem::size_of::<Interface>();

    let ctx_addr = stack_addr
        .add_bits_offset(stack_vidt_block_size)
        .bits_align(32);
    let itf_addr = ctx_addr.add_bits_offset(mem::size_of::<BasicContext>()) as *mut Interface;

    // Unused ram, general purpose within child - Might be a virtual block
    let unused_ram_addr = ctx_addr.add_bits_offset(ctx_itf_block_size).bits_align(32);

    // Rom
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
        .start_addr
        .add_bits_offset(child_ram_block.size()) as *const u8;
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
        parent_kern_addr as *const u32,
        None,
    )
    .unwrap();

    // prepare the parent's kernel structure
    pip_rust_mpu::prepare(&parent_pd_block_id, None, &parent_kern_block_id).unwrap();

    // child's partition descriptor
    let pd_block_id =
        pip_rust_mpu::cut_memory_block(&parent_kern_block_id, pd_addr as *const u32, None).unwrap();

    // child's first kernel structure
    let kern_block_id =
        pip_rust_mpu::cut_memory_block(&pd_block_id, kern_addr as *const u32, None).unwrap();

    // CHILD BLOCKS

    // Ram blocks

    //  * `stack_vidt_vlock_id` is the local id of the given child ram block if its start address is already aligned on stack/vidt size.
    //The first aligned block in child's ram block otherwise. It's the block containing the stack & vidt of the child's partition.
    //  * `ram_head_block_id` is the local id of the left over head if the start_address of the given child's ram block isn't aligned on the vidt/stack block's size.
    //i.e. the block between the child's block start address and the first address aligned on vidt/stack size
    let (stack_vidt_block_id, ram_head_block_id) =
        if stack_addr == child_ram_block.start_addr as *const u8 {
            (child_ram_block.local_id, None)
        } else {
            (
                pip_rust_mpu::cut_memory_block(
                    &child_ram_block.local_id,
                    stack_addr as *const u32,
                    None,
                )
                .unwrap(),
                Some(child_ram_block.local_id),
            )
        };

    //The local id of the block containing the context & interface of the partition.
    let ctx_itf_block_id =
        pip_rust_mpu::cut_memory_block(&stack_vidt_block_id, ctx_addr as *const u32, None).unwrap();
    //The left over tail once the vidt/stack and context/interface blocks have been cut. General purpose within the child partition.
    let unused_ram_block_id_option = if unused_ram_addr < ram_end_addr {
        Some(
            pip_rust_mpu::cut_memory_block(&ctx_itf_block_id, unused_ram_addr as *const u32, None)
                .unwrap(),
        )
    } else {
        None
    };

    // Rom blocks
    let parent_rom_block_attr =
        pip_rust_mpu::find_block(&parent_pd_block_id, entry_point as *const u32).unwrap();

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
                    entry_point as *const u32,
                    None,
                )
                .unwrap(),
                Some(parent_rom_block_attr.local_id),
            )
        };

    //The left over tail, depending on the requested amount of rom. General purpose within the child partition.
    let unused_rom_block_id_option =
        if unused_rom_addr < parent_rom_block_attr.end_addr as *const u8 {
            Some(
                pip_rust_mpu::cut_memory_block(&rom_block_id, unused_ram_addr as *const u32, None)
                    .unwrap(),
            )
        } else {
            None
        };

    //The left over tail, depending on the requested amount of rom. General purpose within the PARENT partition.
    let rom_end_block_id = if rom_end_addr < parent_rom_block_attr.end_addr as *const u8 {
        Some(
            pip_rust_mpu::cut_memory_block(
                &(unused_rom_block_id_option.unwrap()),
                rom_end_addr as *const u32,
                None,
            )
            .unwrap(),
        )
    } else {
        None
    };

    /// __________________________________
    ///
    /// CREATE PARTITION AND ASSIGN BLOCKS
    /// __________________________________
    pip_rust_mpu::create_partition(&pd_block_id).unwrap();
    pip_rust_mpu::prepare(&pd_block_id, None, &kern_block_id).unwrap();

    let child_stack_vidt_block_id =
        pip_rust_mpu::add_memory_block(&pd_block_id, &stack_vidt_block_id, true, true, false)
            .unwrap();
    let child_ctx_itf_block_id =
        pip_rust_mpu::add_memory_block(&pd_block_id, &ctx_itf_block_id, true, true, false).unwrap();
    let child_unused_ram_block_id_option = match unused_ram_block_id_option {
        Some(x) => {
            Some(pip_rust_mpu::add_memory_block(&pd_block_id, &x, true, true, false).unwrap())
        }
        _ => None,
    };

    let child_rom_block_id =
        pip_rust_mpu::add_memory_block(&pd_block_id, &rom_block_id, true, false, true).unwrap();
    let child_unused_rom_block_id_option = match unused_rom_block_id_option {
        Some(x) => {
            Some(pip_rust_mpu::add_memory_block(&pd_block_id, &x, true, false, true).unwrap())
        }
        _ => None,
    };

    let partition = Partition::new(
        child_stack_vidt_block_id,
        child_ctx_itf_block_id,
        child_rom_block_id,
        child_unused_ram_block_id_option,
        child_unused_rom_block_id_option,
    );
    let partition_in_parent = Partition::new(
        stack_vidt_block_id,
        ctx_itf_block_id,
        rom_block_id,
        unused_ram_block_id_option,
        unused_rom_block_id_option,
    );
    let parent_infos = Parent::new(
        partition_in_parent,
        ram_head_block_id,
        rom_head_block_id,
        rom_end_block_id,
        Some(parent_kern_block_id),
        pd_block_id,
        kern_block_id,
    );

    pip_rust_mpu::set_vidt(&pd_block_id, vidt_addr as *const u32).unwrap();

    Ok(CreateReturn::new(partition, parent_infos))
}

pub fn m_map_partition(partition_full_infos: CreateReturn) {
    pip_rust_mpu::map_mpu(
        &partition_full_infos.parent_infos.pd_block_id,
        &partition_full_infos.partition.stack_vidt_block_id,
        0,
    )
    .unwrap();
    pip_rust_mpu::map_mpu(
        &partition_full_infos.parent_infos.pd_block_id,
        &partition_full_infos.partition.ctx_itf_block_id,
        1,
    )
    .unwrap();
    pip_rust_mpu::map_mpu(
        &partition_full_infos.parent_infos.pd_block_id,
        &partition_full_infos.partition.rom_block_id,
        2,
    )
    .unwrap();

    match partition_full_infos.partition.unused_ram_block_id {
        Some(x) => {
            pip_rust_mpu::map_mpu(&partition_full_infos.parent_infos.pd_block_id, &x, 3).unwrap()
        }
        _ => {}
    }

    match partition_full_infos.partition.unused_rom_block_id {
        Some(x) => {
            pip_rust_mpu::map_mpu(&partition_full_infos.parent_infos.pd_block_id, &x, 4).unwrap()
        }
        _ => {}
    }
}

pub fn m_delete_partition(partition_full_infos: CreateReturn, parent: Partition) {
    let pd_id = partition_full_infos.parent_infos.pd_block_id;
    pip_rust_mpu::unset_vidt(&pd_id);

    match partition_full_infos.partition.unused_rom_block_id {
        Some(block) => {
            pip_rust_mpu::unmap_mpu(&pd_id, 4).unwrap();
            pip_rust_mpu::remove_memory_block(&block);
        }
        _ => {}
    }

    match partition_full_infos.partition.unused_ram_block_id {
        Some(block) => {
            pip_rust_mpu::unmap_mpu(&pd_id, 3).unwrap();
            pip_rust_mpu::remove_memory_block(&block);
        }
        _ => {}
    }

    pip_rust_mpu::unmap_mpu(&pd_id, 2).unwrap();
    pip_rust_mpu::unmap_mpu(&pd_id, 1).unwrap();
    pip_rust_mpu::unmap_mpu(&pd_id, 0).unwrap();

    pip_rust_mpu::remove_memory_block(&partition_full_infos.partition.stack_vidt_block_id).unwrap();
    pip_rust_mpu::remove_memory_block(&partition_full_infos.partition.ctx_itf_block_id).unwrap();
    pip_rust_mpu::remove_memory_block(&partition_full_infos.partition.rom_block_id).unwrap();

    pip_rust_mpu::collect(&pd_id).unwrap();
    pip_rust_mpu::delete_partition(&pd_id).unwrap();

    // MERGE RAM

    // Base Ram

    let child_ram_block_id = match partition_full_infos.parent_infos.ram_head_block_id {
        Some(block) => pip_rust_mpu::merge_memory_blocks(
            &block,
            &partition_full_infos.partition.stack_vidt_block_id,
            None,
        )
        .unwrap(),
        _ => partition_full_infos.partition.stack_vidt_block_id,
    };

    let child_ram_block_id = pip_rust_mpu::merge_memory_blocks(
        &child_ram_block_id,
        &partition_full_infos.partition.ctx_itf_block_id,
        None,
    )
    .unwrap();

    let child_ram_block_id = match partition_full_infos.partition.unused_ram_block_id {
        Some(block) => {
            pip_rust_mpu::merge_memory_blocks(&child_ram_block_id, &block, None).unwrap()
        }
        _ => child_ram_block_id,
    };

    // Pip Ram

    let pip_ram_block_id = pip_rust_mpu::merge_memory_blocks(
        &partition_full_infos.parent_infos.kern_block_id,
        &partition_full_infos.parent_infos.pd_block_id,
        None,
    )
    .unwrap();

    // Tries to merge pip & base ram. Might not work, and still be valid, as the pip block might or might not have been built within child block.
    let (child_ram_block_id, pip_ram_block_id) =
        match pip_rust_mpu::merge_memory_blocks(&child_ram_block_id, &pip_ram_block_id, None) {
            Ok(block) => (block, None),
            _ => (child_ram_block_id, Some(pip_ram_block_id)),
        };


    // MERGE ROM

    let rom_block_id = match partition_full_infos.parent_infos.rom_head_block_id {
        Some(block) => pip_rust_mpu::merge_memory_blocks(&block, partition_full_infos.partition.rom, None).unwrap(),
        _ => partition_full_infos.partition.rom,
    };

    let rom_block_id = match partition_full_infos.partition.unused_rom_block_id {
        Some(block) => pip_rust_mpu::merge_memory_blocks(&rom_block_id, &block, None).unwrap(),
        _ => rom_block_id,
    };

    let rom_block_id = match partition_full_infos.parent_infos.rom_tail_block_id {
        Some(block) => pip_rust_mpu::merge_memory_blocks(&rom_block_id, &block, None).unwrap(),
        _= => rom_block_id,
    };
}
