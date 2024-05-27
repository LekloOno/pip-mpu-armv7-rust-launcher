use crate::pip_mpu::core::pip_items::Interface;
use core::ptr;

pub struct Partition {
    pub stack_addr: *const u32,
    pub vidt_addr: *const u32,
    pub interface_addr: *const Interface,
    pub stack_vidt_block_id: *const u32, //Block containing the stack & vidt
    pub interface_block_id: *const u32,  //Block containing the interface
    pub rom_block_id: *const u32,        //Block containing the used ROM

    //Branch partition attributes
    pub unused_ram_block_id: *const u32, //Block containing the unused RAM, NULL if this partition is a leaf partition
    pub unused_rom_block_id: *const u32, //Block containing the unused ROM, NULL if this partition is a leaf partition

    //Merge data - used when deleting a partition to merge it back to its parent
    pub pd_block_id: *const u32,
    pub kern_block_id: *const u32,
}

impl Partition {
    pub fn new() -> Self {
        Partition {
            stack_addr: ptr::null(),
            vidt_addr: ptr::null(),
            interface_addr: ptr::null(),
            stack_vidt_block_id: ptr::null(),
            interface_block_id: ptr::null(),
            rom_block_id: ptr::null(),
            unused_ram_block_id: ptr::null(),
            unused_rom_block_id: ptr::null(),
            pd_block_id: ptr::null(),
            kern_block_id: ptr::null(),
        }
    }
}

pub struct CreateReturn {
    pub partition: Partition,            //The created partition datas.
    pub parent_new_kern_id: Option<u32>, //A new kernel structure, if it was required to create the requested partition
}

impl CreateReturn {
    pub fn new() -> Self {
        Self {
            partition: Partition::new(),
            parent_new_kern_id: None,
        }
    }
}
