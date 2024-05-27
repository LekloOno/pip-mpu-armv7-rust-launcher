use crate::pip_mpu::core::pip_items::Interface;

pub struct Partition {
    pub stack_addr: *const u32,
    pub vidt_addr: *const u32,
    pub itf: *const Interface,
    pub block0_id: *const u32, //Block containing the stack & vidt
    pub block1_id: *const u32, //Block containing the interface
    pub block2_id: *const u32, //Block containing the used ROM

    //Branch partition attributes
    pub block3_id: *const u32, //Block containing the unused RAM, NULL if this partition is a leaf partition
    pub block4_id: *const u32, //Block containing the unused ROM, NULL if this partition is a leaf partition
}
