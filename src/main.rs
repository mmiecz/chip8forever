use std::path::{PathBuf, Path};
use snafu::{ensure, Backtrace, ErrorCompat, ResultExt, Snafu};
use structopt::StructOpt;
use std::fs::File;

#[derive(Debug, StructOpt)]
#[structopt(name = "Chip8Forever", about = "Yet another chip8 emulator in infinite sea of those.")]
struct Options {
    /// Input file
    #[structopt( name="path-to-rom", short="r", long="rom", parse(from_os_str))]
    rom_path: PathBuf,
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Could not open config from {}: {}", filename.display(), source))]
    FileError {
        filename: PathBuf,
        source: std::io::Error
    }
}
type Result<T, E = Error> = std::result::Result<T, E>;

fn open_file<T: AsRef<Path>>(path: T) -> Result<std::fs::File, Error> {
    let filename = path.as_ref();
    let file = File::open(filename).context(FileError { filename: filename.to_path_buf() })?;
    Ok(file)
}

fn main() -> Result<(), Error> {
    let opt = Options::from_args();
    match open_file(opt.rom_path) {
        Ok(file) => {
            println!("{:?}", file)
        }
        Err(error) => {
            return Err(error)
        }
    }
    Ok(())
}
