use crate::pip_mpu::core::pip_items::Interface;

pub struct Partition {
    pub stack_addr: *const u32,
    pub vidt_addr: *const u32,
    pub itf: *const Interface,
    pub stack_vidt_block_id: *const u32, //Block containing the stack & vidt
    pub interface_block_id: *const u32,  //Block containing the interface
    pub rom_block_id: *const u32,        //Block containing the used ROM

    //Branch partition attributes
    pub unused_ram_block_id: *const u32, //Block containing the unused RAM, NULL if this partition is a leaf partition
    pub unused_rom_block_id: *const u32, //Block containing the unused ROM, NULL if this partition is a leaf partition

    //Merge data - used when deleting a partition to merge it back to its parent
    pd_id: *const u32,
    kern_id: *const u32,
}

pub struct CreateReturn {
    pub partition: Partition,        //The created partition datas.
    pub parent_kern_id: Option<u32>, //A new kernel structure, if it was required to create the requested partition
}
