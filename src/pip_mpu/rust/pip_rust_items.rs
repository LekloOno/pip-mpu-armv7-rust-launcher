use crate::pip_mpu::core::pip_items::BlockAttr;

#[derive(Clone, Copy)]
pub struct BlockId {
    id: usize,
}

impl BlockId {
    pub fn new(val: usize) -> BlockId {
        BlockId { id: val }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

pub struct Block {
    pub local_id: BlockId,
    pub start_addr: *const u32,
    pub end_addr: *const u32,
    pub read: bool,
    pub write: bool,
    pub exec: bool,
    pub accessible: bool,
}

impl Block {
    pub fn fromCore(block_attr: BlockAttr) -> Self {
        Self {
            local_id: BlockId::new(block_attr.local_id as usize),
            start_addr: block_attr.start_addr,
            end_addr: block_attr.end_addr,
            read: block_attr.read & 1 == 1,
            write: block_attr.write & 1 == 1,
            exec: block_attr.exec & 1 == 1,
            accessible: block_attr.accessible & 1 == 1,
        }
    }

    pub fn new() -> Self {
        Self {
            local_id: BlockId::new(0),
            start_addr: core::ptr::null(),
            end_addr: core::ptr::null(),
            read: false,
            write: false,
            exec: false,
            accessible: false,
        }
    }

    pub fn size(&self) -> usize {
        self.end_addr as usize - self.start_addr as usize
    }
}
