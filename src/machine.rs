use snafu::{ResultExt, Snafu};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::cpu::Cpu;
use crate::display::{DisplaySubsystem, Sprite};
use crate::input::InputSubsystem;
use crate::mem::Memory;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use crate::audio::AudioSubsystem;

#[derive(Debug, Snafu)]
pub enum RomError {
    #[snafu(display("Could not load ROM from file {}: {}", filename.display(), source))]
    FileError {
        filename: PathBuf,
        source: std::io::Error,
    },
}
#[derive(Debug)]
pub struct Rom {
    content: Vec<u8>,
}

impl Rom {
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, RomError> {
        let filename = path.as_ref();
        let mut file = File::open(filename).context(FileError {
            filename: filename.to_path_buf(),
        })?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).context(FileError {
            filename: filename.to_path_buf(),
        })?;
        println!("ROM size: {}", buffer.len());
        Ok(Rom { content: buffer })
    }

    //I don't know how to return iter, so I will just return a whole vec...
    //I know, weak.
    pub fn get_bytes(&self) -> Vec<u8> {
        self.content.clone()
    }
}

pub struct Machine {
    memory: Memory,
    cpu: Cpu,
    input: InputSubsystem,
    display: DisplaySubsystem,
    audio: AudioSubsystem
}

impl Machine {
    pub fn new(input: InputSubsystem, display: DisplaySubsystem, audio: AudioSubsystem) -> Machine {
        let memory = Memory::new();
        let cpu = Cpu::new();
        Machine {
            memory,
            cpu,
            input,
            display,
            audio,
        }
    }
    fn load_rom(&mut self, rom: Rom, offset: u16) {
        let rom = rom.get_bytes();
        for (i, byte) in rom.iter().enumerate() {
            self.memory.write_8(*byte, offset + i as u16);
        }
    }
    fn load_fonts(&mut self, offset: u16) {
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
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        for (i, byte) in font_set.iter().enumerate() {
            self.memory.write_8(*byte, offset + i as u16); // TODO: pass this offset somewhere to the MEMORY?
        }
    }
    pub fn init(&mut self, rom: Rom) {
        self.load_rom(rom, 0x200);
        self.load_fonts(0x0);
        self.cpu.reset();
    }

    fn should_quit(
        &self,
        event: &Option<sdl2::event::Event>,
        pressed_keys: &Vec<sdl2::keyboard::Scancode>,
    ) -> bool {
        if let Some(sdl2::event::Event::Quit { .. }) = event {
            return true;
        }
        let esc_pressed = pressed_keys
            .iter()
            .find(|key| **key == sdl2::keyboard::Scancode::Escape)
            .is_some();
        return esc_pressed;
    }
    pub fn run(&mut self) {
        self.display.set_color(Color::RGB(0, 0, 0));
        'main: loop {
            let event = self.input.poll();
            let keys = self.input.keys_pressed();

            self.cpu.step(&mut self.memory, &mut self.display, &mut self.input, &mut self.audio);
            //self.input.wait_for_keypress(Scancode::Space);
            println!("Step");
            if self.should_quit(&event, &keys) {
                break 'main;
            }
        }
    }
}
