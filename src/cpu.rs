use crate::display::DisplaySubsystem;
use crate::input::InputSubsystem;
use crate::mem::Memory;

const REGS: usize = 16;
const STACK_SIZE: usize = 16;

pub struct Cpu {
    regs: [u8; REGS],
    i: u16,
    pc: u16,
    dt: u16,
    st: u16,
    stack: [u16; STACK_SIZE],
    sp: usize,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            regs: [0; REGS],
            i: 0,
            pc: 0,
            dt: 0,
            st: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu::default()
    }

    pub fn reset(&mut self) {
        *self = Cpu {
            pc: 0x200,
            ..Default::default()
        }
    }

    pub fn step(
        &mut self,
        memory: &mut Memory,
        display: &mut DisplaySubsystem,
        input: &InputSubsystem,
    ) {
        let instruction = memory.read_range(self.pc, 2);
    }

    //Clear screen
    fn cls(&mut self, &mut display: DisplaySubsystem) {
        unimplemented!()
    }

    //Return from subroutine
    fn rts(&mut self) {
        unimplemented!()
    }

    //Jump to address
    fn jmp(&mut self, addr: u16) {
        unimplemented!()
    }

    //Call subroutine
    fn call(&mut self, addr: u16) {
        unimplemented!()
    }

    //Skip if equal
    fn skeq(&mut self, reg: u8, val: u8) {
        unimplemented!()
    }

    //Skip if not equal
    fn skne(&mut self, reg: u8, val: u8) {
        unimplemented!()
    }

    //Skip if 2 regs are equal
    fn skeqr(&mut self, reg1: u8, reg2: u8) {
        unimplemented!()
    }

    //Load val to reg
    fn mov(&mut self, reg: u8, val: u8) {
        unimplemented!()
    }

    //Add non carry
    fn addnc(&mut self, reg: u8, val: u8) {
        unimplemented!()
    }

    //Move value from reg2 to reg1
    fn movr(&mut self, reg1: u8, reg2: u8) {
        unimplemented!()
    }

    //OR values, store result in reg1
    fn or(&mut self, reg1: u8, reg2: u8) {
        unimplemented!()
    }

    //AND values, store result in reg1
    fn and(&mut self, reg1: u8, reg2: u8) {
        unimplemented!()
    }

    //XOR values, store resuilt in reg1
    fn xor(&mut self, reg1: u8, reg2: u8) {
        unimplemented!()
    }

    //Add reg2 to reg1; if overflows then VF flag is set
    fn add(&mut self, reg1: u8, reg2: u8) {
        unimplemented!()
    }

    //Vx = Vx - Vy, set VF = NOT borrow. Set VF if Vx > Vy
    fn sub(&mut self, reg1: u8, reg2: u8) {
        unimplemented!()
    }

    //Shift right. Store less significant bit in VF.
    fn shr(&mut self, reg: u8) {
        unimplemented!()
    }

    //Set Vx = Vy - Vx, set VF = NOT borrow. Set VF if Vy > Vx
    fn rsb(&mut self, reg1: u8, reg2: u8) {
        unimplemented!()
    }

    //Shift left. Most significant bit is stored in VF
    fn shl(&mut self, reg: u8) {
        unimplemented!()
    }

    //Skip next instruction if Vx != Vy.
    fn skner(&mut self, reg1: u8, reg2: u8) {
        unimplemented!()
    }

    //Set I = addr.
    fn mvi(&mut self, addr: u16) {
        unimplemented!()
    }

    //Jump to V0 + addr.
    fn jmi(&mut self, addr: u16) {
        unimplemented!()
    }

    //Load random from 0-255, AND with val and store to V[reg]
    fn rnd(&mut self, reg: u8, val: u8) {
        unimplemented!()
    }

    //Draw [HEIGHT] bytes at (reg1, reg2) position. VF = 1 if there is a collision.
    fn drw(&mut self, reg1: u8, reg2: u8, height: u8, display: &mut DisplaySubsystem) {
        unimplemented!()
    }

    //Skip if key from REG is pressed.
    fn skkp(&mut self, reg: u8, input: &InputSubsystem) {
        unimplemented!()
    }

    //Skip if key from reg is NOT pressed
    fn skkr(&mut self, reg: u8, input: &InputSubsystem) {
        unimplemented!()
    }

    //Place DT value into REG
    fn gdel(&mut self, reg: u8) {
        unimplemented!()
    }

    //Wait for key from REG
    fn wkey(&mut self, reg: u8) {
        unimplemented!()
    }

    //Set DT value from REG
    fn sdel(&mut self, reg: u8) {
        unimplemented!()
    }

    //Set ST from REG
    fn ssnd(&mut self, reg: u8) {
        unimplemented!()
    }

    //Add I to V[REG] andr store it in I.
    fn adi(&mut self, reg: u8) {
        unimplemented!()
    }

    //Set I to location of sprite digit from V[REG]
    fn font(&mut self, reg: u8) {
        unimplemented!()
    }

    //Store three digits in I I+1 I+2
    //TODO: Should this instruction modify I directly?
    fn bcd(&mut self, reg: u8) {
        unimplemented!()
    }

    //Store all registers from V[0] to V[REG] starting from I.
    fn str(&mut self, reg: u8) {
        unimplemented!()
    }

    //Load values to registers from V[0] to V[REG] starting from I.
    fn ldr(&mut self, reg: u8) {
        unimplemented!()
    }

}