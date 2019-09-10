use std::path::{Path, PathBuf};
use snafu::{ResultExt, Snafu};
use std::fs::File;
use std::io::Read;

use crate::mem::Memory;
use crate::cpu::Cpu;

#[derive(Debug, Snafu)]
pub enum RomError {
    #[snafu(display("Could not load ROM from file {}: {}", filename.display(), source))]
    FileError {
        filename: PathBuf,
        source: std::io::Error
    },
}
#[derive(Debug)]
pub struct Rom {
    content: Vec<u8>,
}

impl Rom {
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, RomError> {
        let filename = path.as_ref();
        let mut file = File::open(filename).context(FileError { filename: filename.to_path_buf() })?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).context(FileError { filename: filename.to_path_buf()})?;
        println!("ROM size: {}", buffer.len());
        Ok(Rom{ content: buffer })
    }

    //I don't know how to return iter, so I will just return a whole vec...
    //I know, weak.
    pub fn get_bytes(&self) -> Vec<u8> {
        self.content.clone()
    }
}


pub struct Machine {
    memory: Memory,
    //memory: Memory
    //cpu: Cpu
    //input_events: Events
    //display: Display
}

impl Machine{
    fn load_rom(mem: &mut Memory, rom: Rom, offset: usize) {
        let rom = rom.get_bytes();
        for (i, byte) in rom.iter().enumerate() {
            mem.write_8(*byte, offset + i);
        }
    }
    fn load_fonts(mem: &mut Memory, offset: usize) {
        let font_set: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        for (i, byte) in font_set.iter().enumerate() {
            mem.write_8(*byte, offset + i);
        }
    }
    pub fn new(rom: Rom) -> Machine {
        let mut mem = Memory::new();
        Machine::load_rom(&mut mem, rom, 0x200 as usize );
        Machine::load_fonts(&mut mem, 0);
        let mut cpu = Cpu::new();
        cpu.reset();
        //let input_events = Input::new();
        //let display = Display::new();
        //Machine::display_reset(display); ?
        Machine{ memory: mem }
    }
    pub fn run() {

    }
}