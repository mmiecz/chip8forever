
const MEM_SIZE: usize = 4096;
pub struct Memory {
    data: [u8; MEM_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory{data: [0; MEM_SIZE]}
    }
    pub fn write_8(&mut self, b: u8, addr: usize) {
        self.data[addr] = b;
    }
}