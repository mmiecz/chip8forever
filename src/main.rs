mod machine;
mod mem;
mod cpu;

use std::path::PathBuf;
use snafu::Snafu;
use structopt::StructOpt;

use machine::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "Chip8Forever", about = "Yet another chip8 emulator in infinite sea of those.")]
struct Options {
    /// Input file
    #[structopt( name="path-to-rom", short="r", long="rom", parse(from_os_str))]
    rom_path: PathBuf,
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Error while attempting to load ROM"))]
    RomError {
        source: machine::RomError
    }
}
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<(), Error> {
    let opt = Options::from_args();
    let rom = Rom::from_file(opt.rom_path);
    let rom = rom.expect("Rom Error");
    let mut _machine = Machine::new(rom);

    Ok(())
}
