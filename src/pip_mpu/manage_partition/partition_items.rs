use crate::pip_mpu::core::pip_items::Interface;
use crate::pip_mpu::rust::pip_rust_items::BlockId;
use core::ptr;

pub struct Partition {
    pub stack_vidt_block_id: BlockId, //Local Id of the block containing the stack & vidt
    pub ctx_itf_block_id: BlockId,    //Local Id of the block containing the interface
    pub rom_block_id: BlockId,        //Local Id of the block containing the used ROM

    //Branch partition attributes
    pub unused_ram_block_id: Option<BlockId>, //Local Id of the block containing the unused RAM, NULL if this partition is a leaf partition
    pub unused_rom_block_id: Option<BlockId>, //Local Id of the block containing the unused ROM, NULL if this partition is a leaf partition
}

impl Partition {
    pub fn new(
        stack_vidt: BlockId,
        ctx_itf: BlockId,
        rom: BlockId,
        unused_ram: Option<BlockId>,
        unused_rom: Option<BlockId>,
    ) -> Self {
        Partition {
            stack_vidt_block_id: stack_vidt,
            ctx_itf_block_id: ctx_itf,
            rom_block_id: rom,
            unused_ram_block_id: unused_ram,
            unused_rom_block_id: unused_rom,
        }
    }
}

pub struct Parent {
    pub child_in_parent: Partition, //The local ids within the parent of the block cut for the child.
    pub ram_head_block_id: Option<BlockId>, //The remaining part of the provided block after cutting the aligned stack/vidt. None if the provided block was already aligned.
    pub rom_head_block_id: Option<BlockId>, //The remaining part of the rom block containing the entry point address. None if the provided entry address was the start address of its block.
    pub rom_tail_block_id: Option<BlockId>, //The remaining part of the rom block after used and unused ram blocks have been cut.
    pub new_kern_block_id: Option<BlockId>, //A new kernel structure, if it was required to create the requested partition (For now, a new kernel structure will always be created)

    //Merge data - used when deleting a partition to merge it back to its parent
    //In this partition's life time, these datas are unaccessible as they belong to pip.
    pub pd_block_id: BlockId, //Local Id of the block containing the partition descriptor
    pub kern_block_id: BlockId, //Local Id of the block containing the kernel structure
}

impl Parent {
    pub fn new(
        child_in_parent: Partition,
        ram_head_block_id: Option<BlockId>,
        rom_head_block_id: Option<BlockId>,
        rom_tail_block_id: Option<BlockId>,
        new_kern_block_id: Option<BlockId>,
        pd_block_id: BlockId,
        kern_block_id: BlockId,
    ) -> Self {
        Parent {
            child_in_parent,
            ram_head_block_id,
            rom_head_block_id,
            rom_tail_block_id,
            new_kern_block_id,
            pd_block_id,
            kern_block_id,
        }
    }
}

pub struct CreateReturn {
    pub partition: Partition, //The created partition datas.
    pub parent_infos: Parent, //The informations of the partition creation related to the parent
}

impl CreateReturn {
    pub fn new(partition: Partition, parent_infos: Parent) -> Self {
        Self {
            partition,
            parent_infos,
        }
    }
}
