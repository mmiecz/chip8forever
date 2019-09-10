const REGS: usize = 16;
const STACK_SIZE: usize = 16;
pub struct Cpu {
    regs: [u8;  REGS],
    i: u16,
    pc: u16,
    dt: u16,
    st: u16,
    stack: [u16; STACK_SIZE],
    sp: usize,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu{ regs: [0; REGS], i: 0, pc: 0, dt: 0, st: 0, stack: [0; STACK_SIZE], sp: 0}
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu::default()
    }

    pub fn reset(&mut self) {
        *self = Cpu{ pc: 0x200, .. Default::default() }
    }
}