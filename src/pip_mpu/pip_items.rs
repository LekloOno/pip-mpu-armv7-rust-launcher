//use core::slice;

pub mod pip_items {
    const BASIC_FRAME_SIZE: usize = 17;
    const EXTENDED_FRAME_SIZE: usize = 50;

    #[repr(C)]
    pub struct BlockAttr {
        entry_addr: *const u32,
        start_addr: *const u32,
        end_addr: *const u32,
        read: u32,
        write: u32,
        exec: u32,
        accessible: u32,
    }

    #[repr(C)]
    pub struct BlockOrError {
        error: i32,
        block_attr: BlockAttr,
    }
    /*
        #[repr(C)]
        pub BasicFrame {
            sp: u32,
            r4: u32,
            r5: u32,
            r6: u32,
            r7: u32,
            r8: u32,
            r9: u32,
            r10: u32,
            r11: u32,
            r0: u32,
            r1: u32,
            r2: u32,
            r3: u32,
            r12: u32,
            lr: u32,
            pc: u32,
            xpsr: u32,
        }
    */
    // Frame trait allows for Generic Frame, replacing StackedFrame
    trait Frame {
        fn sp(&mut self) -> u32;
        fn r4(&mut self) -> u32;
        fn r5(&mut self) -> u32;
        fn r6(&mut self) -> u32;
        fn r7(&mut self) -> u32;
        fn r8(&mut self) -> u32;
        fn r9(&mut self) -> u32;
        fn r10(&mut self) -> u32;
        fn r11(&mut self) -> u32;
        fn r0(&mut self) -> u32;
        fn r1(&mut self) -> u32;
        fn r2(&mut self) -> u32;
        fn r3(&mut self) -> u32;
        fn r12(&mut self) -> u32;
        fn lr(&mut self) -> u32;
        fn pc(&mut self) -> u32;
        fn xpsr(&mut self) -> u32;
    }

    pub struct BasicFrame {
        pub registers: [u32; BASIC_FRAME_SIZE],
    }

    impl Frame for BasicFrame {
        fn sp(&mut self) -> u32 {
            self.registers[0]
        }
        fn r4(&mut self) -> u32 {
            self.registers[1]
        }
        fn r5(&mut self) -> u32 {
            self.registers[2]
        }
        fn r6(&mut self) -> u32 {
            self.registers[3]
        }
        fn r7(&mut self) -> u32 {
            self.registers[4]
        }
        fn r8(&mut self) -> u32 {
            self.registers[5]
        }
        fn r9(&mut self) -> u32 {
            self.registers[6]
        }
        fn r10(&mut self) -> u32 {
            self.registers[7]
        }
        fn r11(&mut self) -> u32 {
            self.registers[8]
        }
        fn r0(&mut self) -> u32 {
            self.registers[9]
        }
        fn r1(&mut self) -> u32 {
            self.registers[10]
        }
        fn r2(&mut self) -> u32 {
            self.registers[11]
        }
        fn r3(&mut self) -> u32 {
            self.registers[12]
        }
        fn r12(&mut self) -> u32 {
            self.registers[13]
        }
        fn lr(&mut self) -> u32 {
            self.registers[14]
        }
        fn pc(&mut self) -> u32 {
            self.registers[15]
        }
        fn xpsr(&mut self) -> u32 {
            self.registers[16]
        }
    }

    pub struct ExtendedFrame {
        pub registers: [u32; EXTENDED_FRAME_SIZE],
    }

    impl Frame for ExtendedFrame {
        fn sp(&mut self) -> u32 {
            self.registers[16]
        }
        fn r4(&mut self) -> u32 {
            self.registers[17]
        }
        fn r5(&mut self) -> u32 {
            self.registers[18]
        }
        fn r6(&mut self) -> u32 {
            self.registers[19]
        }
        fn r7(&mut self) -> u32 {
            self.registers[20]
        }
        fn r8(&mut self) -> u32 {
            self.registers[21]
        }
        fn r9(&mut self) -> u32 {
            self.registers[22]
        }
        fn r10(&mut self) -> u32 {
            self.registers[23]
        }
        fn r11(&mut self) -> u32 {
            self.registers[24]
        }
        fn r0(&mut self) -> u32 {
            self.registers[25]
        }
        fn r1(&mut self) -> u32 {
            self.registers[26]
        }
        fn r2(&mut self) -> u32 {
            self.registers[27]
        }
        fn r3(&mut self) -> u32 {
            self.registers[28]
        }
        fn r12(&mut self) -> u32 {
            self.registers[29]
        }
        fn lr(&mut self) -> u32 {
            self.registers[30]
        }
        fn pc(&mut self) -> u32 {
            self.registers[31]
        }
        fn xpsr(&mut self) -> u32 {
            self.registers[32]
        }
    }

    impl ExtendedFrame {
        pub fn s16(&mut self) -> u32 {
            self.registers[0]
        }
        pub fn s17(&mut self) -> u32 {
            self.registers[1]
        }
        pub fn s18(&mut self) -> u32 {
            self.registers[2]
        }
        pub fn s19(&mut self) -> u32 {
            self.registers[3]
        }
        pub fn s20(&mut self) -> u32 {
            self.registers[4]
        }
        pub fn s21(&mut self) -> u32 {
            self.registers[5]
        }
        pub fn s22(&mut self) -> u32 {
            self.registers[6]
        }
        pub fn s23(&mut self) -> u32 {
            self.registers[7]
        }
        pub fn s24(&mut self) -> u32 {
            self.registers[8]
        }
        pub fn s25(&mut self) -> u32 {
            self.registers[9]
        }
        pub fn s26(&mut self) -> u32 {
            self.registers[10]
        }
        pub fn s27(&mut self) -> u32 {
            self.registers[11]
        }
        pub fn s28(&mut self) -> u32 {
            self.registers[12]
        }
        pub fn s29(&mut self) -> u32 {
            self.registers[13]
        }
        pub fn s30(&mut self) -> u32 {
            self.registers[14]
        }
        pub fn s31(&mut self) -> u32 {
            self.registers[15]
        }
        pub fn s0(&mut self) -> u32 {
            self.registers[33]
        }
        pub fn s1(&mut self) -> u32 {
            self.registers[34]
        }
        pub fn s2(&mut self) -> u32 {
            self.registers[35]
        }
        pub fn s3(&mut self) -> u32 {
            self.registers[36]
        }
        pub fn s4(&mut self) -> u32 {
            self.registers[37]
        }
        pub fn s5(&mut self) -> u32 {
            self.registers[38]
        }
        pub fn s6(&mut self) -> u32 {
            self.registers[39]
        }
        pub fn s7(&mut self) -> u32 {
            self.registers[40]
        }
        pub fn s8(&mut self) -> u32 {
            self.registers[41]
        }
        pub fn s9(&mut self) -> u32 {
            self.registers[42]
        }
        pub fn s10(&mut self) -> u32 {
            self.registers[43]
        }
        pub fn s11(&mut self) -> u32 {
            self.registers[44]
        }
        pub fn s12(&mut self) -> u32 {
            self.registers[45]
        }
        pub fn s13(&mut self) -> u32 {
            self.registers[46]
        }
        pub fn s14(&mut self) -> u32 {
            self.registers[47]
        }
        pub fn s15(&mut self) -> u32 {
            self.registers[48]
        }
        pub fn fpscr(&mut self) -> u32 {
            self.registers[49]
        }
    }

    pub struct BasicContext {
        is_basic_frame: u32,
        pip_flags: u32,
        frame: BasicFrame,
    }

    pub struct ExtendedContext {
        is_basic_frame: u32,
        pip_flags: u32,
        frame: ExtendedFrame,
    }

    pub struct StackedContect {
        is_basic_frame: u32,
    }
    /*
        impl BasicFrame {
            fn get(idx: u32) -> u32 {

            }
        }
    */
    pub struct Interface {
        /// The ID of the block containing the partition descriptor of the root partition
        part_desc_block_id: *const u8,

        /// The limit address of the stack of the root partition
        stack_limit: *const u8,

        /// The stack top address of the root partition
        stack_top: *const u8,

        /// The VIDT start address of the root partition
        vidt_start: *const u8,

        /// The VIDT end address of the root partition
        vidt_end: *const u8,

        /// The start address of the root partition binary
        root: *const u8,

        /// The start address of the unused ROM
        unused_rom_start: *const u8,

        /// The end address of the unused ROM
        rom_end: *const u8,

        /// The start address of the unused RAM
        unused_ram_start: *mut u8,

        /// The end address of the unused RAM
        ram_end: *const u8,
    }
}
