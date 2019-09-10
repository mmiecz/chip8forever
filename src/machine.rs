use std::path::{Path, PathBuf};
use snafu::{ensure, Backtrace, ErrorCompat, ResultExt, Snafu};
use std::fs::File;
use std::io::Read;

use crate::mem::Memory;

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
        file.read_to_end(&mut buffer);
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
    pub fn new(rom: Rom) -> Machine {
        let mut mem = Memory::new();
        Machine::load_rom(&mut mem, rom, 0x200 as usize );
        //Machine::load_fonts(mem, offset); ?
        //let cpu = Cpu::new()
        //Machine::cpu_reset(cpu);
        //let input_events = Input::new();
        //let display = Display::new();
        //Machine::display_reset(display); ?
        Machine{ memory: mem }
    }
    pub fn run() {

    }
}