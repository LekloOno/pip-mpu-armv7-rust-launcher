use crate::pip_mpu::core::pip_items::Interface;
use core::ptr;

pub struct Partition {
    pub stack_addr: *const u32,           //Address of this partition's stack
    pub vidt_addr: *const u32,            //Address of this partition's vidt
    pub interface_addr: *const Interface, //Address of this partition's interface
    pub stack_vidt_block_id: *const u32,  //Local Id of the block containing the stack & vidt
    pub ctx_itf_block_id: *const u32,     //Local Id of the block containing the interface
    pub rom_block_id: *const u32,         //Local Id of the block containing the used ROM

    //Branch partition attributes
    pub unused_ram_block_id: *const u32, //Local Id of the block containing the unused RAM, NULL if this partition is a leaf partition
    pub unused_rom_block_id: *const u32, //Local Id of the block containing the unused ROM, NULL if this partition is a leaf partition

    //Merge data - used when deleting a partition to merge it back to its parent
    //In this partition's life time, these datas are unaccessible as they belong to pip.
    pub pd_block_id: *const u32, //Local Id of the block containing the partition descriptor
    pub kern_block_id: *const u32, //Local Id of the block containing the kernel structure
}

pub struct Parent {
    pub child_in_parent: Partition, //The local ids within the parent of the block cut for the child.
    pub ram_left_over_block_id: Option<*const u32>, //The remaining part of the provided block after cutting the aligned stack/vidt. None if the provided block was already aligned.
    pub rom_left_over_block_id: Option<*const u32>, //The remaining part of the provided block after cutting the flash block. None if the provided entry address was the start address of its block.
    pub new_kern_block_id: Option<*const u32>, //A new kernel structure, if it was required to create the requested partition (For now, a new kernel structure will always be created)
}

impl Partition {
    pub fn new() -> Self {
        Partition {
            stack_addr: ptr::null(),
            vidt_addr: ptr::null(),
            interface_addr: ptr::null(),
            stack_vidt_block_id: ptr::null(),
            ctx_itf_block_id: ptr::null(),
            rom_block_id: ptr::null(),
            unused_ram_block_id: ptr::null(),
            unused_rom_block_id: ptr::null(),
            pd_block_id: ptr::null(),
            kern_block_id: ptr::null(),
        }
    }
}

pub struct CreateReturn {
    pub partition: Partition, //The created partition datas.
    pub parent_new_kern_block_id: Option<*const u32>,
    pub ram_end_block_id: Option<*const u32>, //If there is RAM left out of the partition creation, its block local id
    pub rom_end_block_id: Option<*const u32>, //If there is ROM left out of the partition creation, its block local id
}

impl CreateReturn {
    pub fn new() -> Self {
        Self {
            partition: Partition::new(),
            parent_new_kern_block_id: None,
            ram_end_block_id: None,
            rom_end_block_id: None,
        }
    }
}

pub struct Block {
    pub local_id: *const u32, //Local id of the block
    pub address: *const u32,  //Start Address of the block
    pub size: usize,          //Size of the block
}
