use std::ops::Range;

const MEM_SIZE: usize = 4096;
pub struct Memory {
    data: [u8; MEM_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: [0; MEM_SIZE],
        }
    }
    pub fn write_8(&mut self, b: u8, addr: u16) {
        self.data[addr as usize] = b;
    }

    pub fn read_8(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn read_range(&self, addr: u16, num: u16) -> &[u8] {
        &self.data[addr as usize..(addr + num) as usize]
    }
}
