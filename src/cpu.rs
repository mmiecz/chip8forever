use crate::display::{DisplaySubsystem, Sprite};
use crate::input::{InputSubsystem, KeyboardMapper};
use crate::mem::Memory;

const REGS: usize = 16;
const STACK_SIZE: usize = 16;

pub struct Cpu {
    regs: [u8; REGS],
    i: u16,
    pc: u16,
    dt: u8,
    st: u8,
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

    //Stack push and pop
    fn stack_push(&mut self, val: u16) {
        self.sp += 1;
        assert!(self.sp >= STACK_SIZE, "Error! Stack over boundaries");
        self.stack[self.sp] = val;
    }

    fn stack_pop(&mut self) -> u16 {
        let val = self.stack[self.sp];
        assert!(self.sp >= 1, "Stack error!");
        self.sp -= 1;
        val
    }

    //Regs routines
    fn reg_set(&mut self, reg: u8, val: u8) {
        self.regs[reg as usize] = val;
    }

    fn reg_get(&self, reg: u8) -> u8 {
        self.regs[reg as usize]
    }

    fn flag_set(&mut self, val: u8) {
        self.regs[15] = val;
    }

    fn flag_get(&self) -> u8 {
        self.regs[15]
    }

    //ROUTINES FUNCTIONS

    //Clear screen
    fn cls(&mut self, display: &mut DisplaySubsystem) {
        display.clear();
    }

    //Return from subroutine
    fn rts(&mut self) {
        self.pc = self.stack_pop();
    }

    //Jump to address
    fn jmp(&mut self, addr: u16) {
        self.pc = addr;
    }

    //Call subroutine
    fn call(&mut self, addr: u16) {
        self.stack_push(self.pc);
        self.pc = addr;
    }

    //Skip if equal
    fn skeq(&mut self, reg: u8, val: u8) {
        if self.reg_get(reg) == val {
            // TODO: Figure out better pc handling.
            self.pc += 2;
        }
    }

    //Skip if not equal
    fn skne(&mut self, reg: u8, val: u8) {
        if self.reg_get(reg) != val {
            self.pc += 2;
        }
    }

    //Skip if 2 regs are equal
    fn skeqr(&mut self, reg1: u8, reg2: u8) {
        if self.reg_get(reg1) == self.reg_get(reg2) {
            self.pc += 2;
        }
    }

    //Load val to reg
    fn mov(&mut self, reg: u8, val: u8) {
        self.reg_set(reg, val);
    }

    //Add non carry
    fn addnc(&mut self, reg: u8, val: u8) {
        let regval = self.reg_get(reg);
        let result = regval.wrapping_add(val);
        self.reg_set(reg, result);
    }

    //Move value from reg2 to reg1
    fn movr(&mut self, reg1: u8, reg2: u8) {
        self.reg_set(reg1, self.reg_get(reg2));
    }

    //OR values, store result in reg1
    fn or(&mut self, reg1: u8, reg2: u8) {
        let regval = self.reg_get(reg1);
        self.reg_set(reg1, regval | self.reg_get(reg2));
    }

    //AND values, store result in reg1
    fn and(&mut self, reg1: u8, reg2: u8) {
        let regval = self.reg_get(reg1);
        self.reg_set(reg1, regval & self.reg_get(reg2));
    }

    //XOR values, store resuilt in reg1
    fn xor(&mut self, reg1: u8, reg2: u8) {
        let regval = self.reg_get(reg1);
        self.reg_set(reg1, regval ^ self.reg_get(reg2));
    }

    //Add reg2 to reg1; if overflows then VF flag is set
    fn add(&mut self, reg1: u8, reg2: u8) {
        let rv1 = self.reg_get(reg1);
        let rv2 = self.reg_get(reg2);
        let result = rv1.overflowing_add(rv2);
        self.reg_set(reg1, result.0);
        if result.1 == true {
            self.flag_set(1);
        }
    }

    //Vx = Vx - Vy, set VF = NOT borrow. Set VF if Vx > Vy
    fn sub(&mut self, reg1: u8, reg2: u8) {
        let rv1 = self.reg_get(reg1);
        let rv2 = self.reg_get(reg2);
        let result = rv1.overflowing_sub(rv2);
        self.reg_set(reg1, result.0);
        if result.1 == false {
            self.flag_set(1);
        }
    }

    //Shift right. Store less significant bit in VF.
    fn shr(&mut self, reg: u8) {
        let val = self.reg_get(reg);
        self.flag_set(val & 1);
        self.reg_set(reg, val >> 1);
    }

    //Set Vx = Vy - Vx, set VF = NOT borrow. Set VF if Vy > Vx
    fn rsb(&mut self, reg1: u8, reg2: u8) {
        let rv1 = self.reg_get(reg1);
        let rv2 = self.reg_get(reg2);
        let result = rv2.overflowing_sub(rv1);
        self.reg_set(reg1, result.0);
        if result.1 == false {
            self.flag_set(1);
        }

    }

    //Shift left. Most significant bit is stored in VF
    fn shl(&mut self, reg: u8) {
        let val = self.reg_get(reg);
        self.flag_set(val & 0x80); // Get MSB from value
        self.reg_set(reg, val << 1);
    }

    //Skip next instruction if Vx != Vy.
    fn skner(&mut self, reg1: u8, reg2: u8) {
        if self.reg_get(reg1) != self.reg_get(reg2) {
            self.pc += 2;
        }
    }

    //Set I = addr.
    fn mvi(&mut self, addr: u16) {
        self.i = addr;
    }

    //Jump to V0 + addr.
    fn jmi(&mut self, addr: u16) {
        let regval = self.reg_get(0) as u16;
        self.jmp(addr + regval);
    }

    //Load random from 0-255, AND with val and store to V[reg]
    fn rnd(&mut self, reg: u8, val: u8) {
        self.reg_set(reg, 36 % val); // TODO: Add proper random number.
    }

    //Draw [HEIGHT] bytes at (reg1, reg2) position. VF = 1 if there is a collision.
    fn drw(&mut self, reg1: u8, reg2: u8, height: u8, mem: &Memory, display: &mut DisplaySubsystem) {
        let mem = mem.read_range(self.i, height as u16);
        let sprite = Sprite::new(mem);
        let column = self.reg_get(reg1) as usize;
        let row = self.reg_get(reg2) as usize;
        let collision = display.draw(column, row, sprite);
        if collision == true {
            self.flag_set(1);
        }
    }

    //Skip if key from REG is pressed.
    fn skkp(&mut self, reg: u8, input: &InputSubsystem) {
        let keycode = self.reg_get(reg);
        KeyboardMapper::map_to_scancode(keycode).and_then::<(), _>( | keycode| {
            if input.is_key_pressed(keycode) == true {
                self.pc += 2; // Key pressed, advance.
            }
            Some(()) // Make typesystem happy;
        });
    }

    //Skip if key from reg is NOT pressed
    fn skkr(&mut self, reg: u8, input: &InputSubsystem) {
        let keycode = self.reg_get(reg);
        KeyboardMapper::map_to_scancode(keycode).and_then::<(), _>( | keycode| {
            if input.is_key_pressed(keycode) == false {
                self.pc += 2; // Key pressed, advance.
            }
            Some(()) // Make typesystem happy;
        });
    }

    //Place DT value into REG
    fn gdel(&mut self, reg: u8) {
        self.reg_set(reg, self.dt);
    }

    //Wait for key from REG
    fn wkey(&mut self, reg: u8, input: &mut InputSubsystem) {
        let keycode = self.reg_get(reg);
        assert!(keycode < 16, "Keycode value somewhat wrong!");
        input.wait_for_keypress(KeyboardMapper::map_to_scancode(keycode).unwrap());
    }

    //Set DT value from REG
    fn sdel(&mut self, reg: u8) {
        self.dt = self.reg_get(reg);
    }

    //Set ST from REG
    fn ssnd(&mut self, reg: u8) {
        self.st = self.reg_get(reg);
    }

    //Add I to V[REG] andr store it in I.
    fn adi(&mut self, reg: u8) {
        self.i += self.reg_get(reg) as u16;
    }

    //Set I to location of sprite digit from V[REG]
    fn font(&mut self, reg: u8) {
        //Font is 5 bytes; address of the font sprite is font number * 5
        //TODO: CPU should not calculate this address, fix it.
        self.i = self.reg_get(reg) as u16 * 5;
    }

    //Store three digits in I I+1 I+2
    //TODO: Should this instruction modify I directly?
    fn bcd(&mut self, reg: u8, memory: &mut Memory) {
        let mut value = self.reg_get(reg);
        let mut digits = Vec::new(); // max value is 3 digits;
        while value > 0 {
            let digit = value % 10;
            value /= 10;
            digits.push(digit);
        }
        let mut i: u16 = 0;
        while let Some(digit) = digits.pop() {
            let address = self.i + i;
            memory.write_8(digit, address);
        }
    }

    //Store all registers from V[0] to V[REG] starting from I.
    fn str(&mut self, reg: u8, memory: &mut Memory) {
        //For 0 to reg - read all regs and store in memory starting from I.
        for i in 0..=reg {
            let regval = self.reg_get(i);
            self.i += i as u16;
            let addr = self.i;
            memory.write_8(reg, addr);
        }
        self.i += 1;
    }

    //Load values to registers from V[0] to V[REG] starting from I.
    fn ldr(&mut self, reg: u8, memory: &Memory) {
        for i in 0..=reg {
            self.i += self.i + i as u16;
            let memval = memory.read_8(self.i);
            self.reg_set(i, memval);
        }
        self.i += 1;
    }

}