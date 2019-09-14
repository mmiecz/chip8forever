use crate::audio::AudioSubsystem;
use crate::display::{DisplaySubsystem, Sprite};
use crate::input::{InputSubsystem, KeyboardMapper};
use crate::mem::Memory;
use sdl2::audio::AudioStatus;
use sdl2::hint::set_video_minimize_on_focus_loss;

const REGS: usize = 16;
const STACK_SIZE: usize = 16;

mod helper {
    pub fn nibbles(bytes: &[u8]) -> (u8, u8, u8, u8) {
        assert!(bytes.len() == 2);
        let o1 = (bytes[0] & 0xF0) >> 4;
        let o2 = (bytes[0] & 0x0F);

        let o3 = (bytes[1] & 0xF0) >> 4;
        let o4 = (bytes[1] & 0x0F);
        (o1, o2, o3, o4)
    }

    pub fn address(bytes: &[u8]) -> u16 {
        let address: u16 = (((bytes[0] & 0x0F) as u16) << 8) as u16 + bytes[1] as u16;
        address
    }

    #[cfg(test)]
    mod test {
        use crate::cpu::helper::{address, nibbles};

        #[test]
        fn to_nibbles_test() {
            let nibbles = nibbles(&[0xF0, 0xAC]);
            assert!(nibbles.0 == 0xF);
            assert!(nibbles.1 == 0x0);
            assert!(nibbles.2 == 0xA);
            assert!(nibbles.3 == 0xC);
        }

        #[test]
        fn address_test() {
            let address = address(&[0x0F, 0xFF]);
            assert_eq!(address, 0x0FFF);
        }
    }
}

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
    fn handle_beeper(&mut self, audio: &mut AudioSubsystem) {
        if self.st > 1 && audio.get_status() != AudioStatus::Playing {
            audio.resume();
        } else if self.st < 1 {
            audio.pause();
        }
    }
    pub fn step(
        &mut self,
        memory: &mut Memory,
        display: &mut DisplaySubsystem,
        input: &mut InputSubsystem,
        audio: &mut AudioSubsystem,
    ) {
        let instruction = memory.read_range(self.pc, 2);
        println!(
            "Doing: {:X?} @ pc: {:X?} dt: {}",
            instruction, self.pc, self.dt
        );
        self.pc_increment();
        let (o1, o2, o3, o4) = helper::nibbles(instruction);
        let reg1 = o2;
        let reg2 = o3;
        let address = helper::address(instruction);
        let value = (instruction[1] & 0xFF) as u8;

        match (o1, o2, o3, o4) {
            (0x0, 0x0, 0xE, 0x0) => self.clear_screen(display),
            (0x0, 0x0, 0xE, 0xE) => self.return_from_subroutine(),
            (0x1, _, _, _) => self.jump_to(address),
            (0x2, _, _, _) => self.call(address),
            (0x3, reg, _, _) => self.skip_equal(reg, value),
            (0x4, reg, _, _) => self.skip_not_equal(reg, value),
            (0x5, r1, r2, 0) => self.skip_regs_equal(r1, r2),
            (0x6, reg, _, _) => self.mov(reg, value),
            (0x7, reg, _, _) => self.add(reg, value),
            (0x8, r1, r2, 0) => self.mov_regs(r1, r2),
            (0x8, r1, r2, 1) => self.or(r1, r2),
            (0x8, r1, r2, 2) => self.and(r1, r2),
            (0x8, r1, r2, 3) => self.xor(r1, r2),
            (0x8, r1, r2, 4) => self.add_regs(r1, r2),
            (0x8, r1, r2, 5) => self.sub_regs(r1, r2),
            (0x8, r1, r2, 6) => self.shift_right(r1, r2),
            (0x8, r1, r2, 7) => self.sub_regs_2(r1, r2),
            (0x8, r1, r2, 0xE) => self.shift_left(r1, r2),
            (0x9, r1, r2, 0x0) => self.skip_not_equal(r1, r2),
            (0xA, _, _, _) => self.move_i(address),
            (0xB, _, _, _) => self.jump_with_add(address),
            (0xC, reg, _, _) => self.rnd(reg, value),
            (0xD, r1, r2, n) => self.draw(r1, r2, n, memory, display),
            (0xE, reg, 0x9, 0xE) => self.skip_key_pressed(reg, input),
            (0xE, reg, 0xA, 0x1) => self.skip_key_not_pressed(reg, input),
            (0xF, reg, 0x0, 0x7) => self.get_dt(reg),
            (0xF, reg, 0x0, 0xA) => self.wait_for_key(reg, input),
            (0xF, reg, 0x1, 0x5) => self.set_dt(reg),
            (0xF, reg, 0x1, 0x8) => self.set_st(reg),
            (0xF, reg, 0x1, 0xE) => self.add_to_i(reg),
            (0xF, reg, 0x2, 0x9) => self.font(reg), // TODO: maybe better function name
            (0xF, reg, 0x3, 0x3) => self.bcd(reg, memory),
            (0xF, reg, 0x5, 0x5) => self.store_range(reg, memory),
            (0xF, reg, 0x6, 0x5) => self.load_range(reg, memory),
            _ => panic!("WTF: wrong instruction"),
        }
        self.dt_decrement();
        self.st_decrement();

        self.handle_beeper(audio);
    }
    //PC DT and ST routines.
    fn pc_increment(&mut self) {
        self.pc += 2;
        println!("PC: increment: {:x?}", self.pc);
    }

    fn dt_decrement(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
    }

    fn st_decrement(&mut self) {
        if self.st > 0 {
            self.st -= 1;
        }
    }

    //Stack push and pop
    fn stack_push(&mut self, val: u16) {
        self.sp += 1;
        assert!(self.sp < STACK_SIZE, "Error! Stack over boundaries");
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
    fn clear_screen(&mut self, display: &mut DisplaySubsystem) {
        display.clear();
    }

    //Return from subroutine
    fn return_from_subroutine(&mut self) {
        self.pc = self.stack_pop();
    }

    //Jump to address
    fn jump_to(&mut self, addr: u16) {
        self.pc = addr;
    }

    //Call subroutine
    fn call(&mut self, addr: u16) {
        self.stack_push(self.pc);
        self.pc = addr;
    }

    //Skip if equal
    fn skip_equal(&mut self, reg: u8, val: u8) {
        if self.reg_get(reg) == val {
            // TODO: Figure out better pc handling.
            self.pc_increment();
        }
    }

    //Skip if not equal
    fn skip_not_equal(&mut self, reg: u8, val: u8) {
        if self.reg_get(reg) != val {
            self.pc_increment();
        }
    }

    //Skip if 2 regs are equal
    fn skip_regs_equal(&mut self, reg1: u8, reg2: u8) {
        if self.reg_get(reg1) == self.reg_get(reg2) {
            self.pc_increment();
        }
    }

    //Load val to reg
    fn mov(&mut self, reg: u8, val: u8) {
        self.reg_set(reg, val);
    }

    //Add non carry
    fn add(&mut self, reg: u8, val: u8) {
        let regval = self.reg_get(reg);
        let result = regval.wrapping_add(val);
        self.reg_set(reg, result);
    }

    //Move value from reg2 to reg1
    fn mov_regs(&mut self, reg1: u8, reg2: u8) {
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
    fn add_regs(&mut self, reg1: u8, reg2: u8) {
        let rv1 = self.reg_get(reg1);
        let rv2 = self.reg_get(reg2);
        let result = rv1.overflowing_add(rv2);
        self.reg_set(reg1, result.0);
        if result.1 == true {
            self.flag_set(1);
        } else {
            self.flag_set(0);
        }
    }

    //Vx = Vx - Vy, set VF = NOT borrow. Set VF if Vx > Vy
    fn sub_regs(&mut self, reg1: u8, reg2: u8) {
        let rv1 = self.reg_get(reg1);
        let rv2 = self.reg_get(reg2);
        let result = rv1.overflowing_sub(rv2);
        self.reg_set(reg1, result.0);
        if result.1 == false {
            self.flag_set(1);
        } else {
            self.flag_set(0);
        }
    }

    //Shift right. Store less significant bit in VF.
    // V[reg] = V[reg2] >> 1
    fn shift_right(&mut self, reg: u8, reg2: u8) {
        let val = self.reg_get(reg2);
        self.flag_set(val & 1);
        self.reg_set(reg, val >> 1);
    }

    //Set Vx = Vy - Vx, set VF = NOT borrow. Set VF if Vy > Vx
    fn sub_regs_2(&mut self, reg1: u8, reg2: u8) {
        let rv1 = self.reg_get(reg1);
        let rv2 = self.reg_get(reg2);
        let result = rv2.overflowing_sub(rv1);
        self.reg_set(reg1, result.0);
        if result.1 == false {
            self.flag_set(1);
        } else {
            self.flag_set(0);
        }
    }

    //Shift left. Most significant bit is stored in VF
    fn shift_left(&mut self, reg: u8, reg2: u8) {
        let val = self.reg_get(reg2);
        self.flag_set(val & 0x80); // Get MSB from value
        self.reg_set(reg, val << 1);
    }

    //Skip next instruction if Vx != Vy.
    fn skip_not_regs_equal(&mut self, reg1: u8, reg2: u8) {
        if self.reg_get(reg1) != self.reg_get(reg2) {
            self.pc_increment();
        }
    }

    //Set I = addr.
    fn move_i(&mut self, addr: u16) {
        self.i = addr;
    }

    //Jump to V0 + addr.
    fn jump_with_add(&mut self, addr: u16) {
        let regval = self.reg_get(0) as u16;
        self.jump_to(addr + regval);
    }

    //Load random from 0-255, AND with val and store to V[reg]
    fn rnd(&mut self, reg: u8, val: u8) {
        self.reg_set(reg, 36 % val); // TODO: Add proper random number.
    }

    //Draw [HEIGHT] bytes at (reg1, reg2) position. VF = 1 if there is a collision.
    fn draw(
        &mut self,
        reg1: u8,
        reg2: u8,
        height: u8,
        mem: &Memory,
        display: &mut DisplaySubsystem,
    ) {
        let mem = mem.read_range(self.i, height as u16);
        let sprite = Sprite::new(mem);
        let column = self.reg_get(reg1) as usize;
        let row = self.reg_get(reg2) as usize;
        println!("DRAW");
        let collision = display.draw_test(column, row, sprite);
        if collision == true {
            self.flag_set(1);
        } else {
            self.flag_set(0);
        }
    }

    //Skip if key from REG is pressed.
    fn skip_key_pressed(&mut self, reg: u8, input: &InputSubsystem) {
        let keycode = self.reg_get(reg);
        KeyboardMapper::map_to_scancode(keycode).and_then::<(), _>(|keycode| {
            if input.is_key_pressed(keycode) == true {
                self.pc_increment(); // Key pressed, advance.
            }
            Some(()) // Make typesystem happy;
        });
    }

    //Skip if key from reg is NOT pressed
    fn skip_key_not_pressed(&mut self, reg: u8, input: &InputSubsystem) {
        let keycode = self.reg_get(reg);
        KeyboardMapper::map_to_scancode(keycode).and_then::<(), _>(|keycode| {
            if input.is_key_pressed(keycode) == false {
                self.pc_increment(); // Key pressed, advance.
            }
            Some(()) // Make typesystem happy;
        });
    }

    //Place DT value into REG
    fn get_dt(&mut self, reg: u8) {
        println!("get_dt: {:X?} {:X?}", reg, self.dt);
        self.reg_set(reg, self.dt);
    }

    //Wait for key and load it to reg
    fn wait_for_key(&mut self, reg: u8, input: &mut InputSubsystem) {
        unimplemented!()
        //        let keycode = self.reg_get(reg);
        //        assert!(keycode < 16, "Keycode value somewhat wrong!");
        //        input.wait_for_keypress(KeyboardMapper::map_to_scancode(keycode).unwrap());
    }

    //Set DT value from REG
    fn set_dt(&mut self, reg: u8) {
        println!("set_dt: val: {:X?}", self.reg_get(reg));
        self.dt = self.reg_get(reg);
    }

    //Set ST from REG
    fn set_st(&mut self, reg: u8) {
        self.st = self.reg_get(reg);
    }

    //Add I to V[REG] andr store it in I.
    fn add_to_i(&mut self, reg: u8) {
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
    fn store_range(&mut self, reg: u8, memory: &mut Memory) {
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
    fn load_range(&mut self, reg: u8, memory: &Memory) {
        for i in 0..=reg {
            self.i += i as u16;
            let memval = memory.read_8(self.i);
            self.reg_set(i, memval);
        }
        self.i += 1;
    }
}
