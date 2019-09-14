use sdl2::Sdl;
use sdl2::audio::{AudioSpecDesired, AudioCallback, AudioStatus};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 { self.volume } else { -self.volume };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}


pub struct AudioSubsystem {
    device: sdl2::audio::AudioDevice<SquareWave>,
}

impl AudioSubsystem {
    pub fn new(sdl2_context: &Sdl) -> AudioSubsystem {
        let sdl2_audio = sdl2_context.audio().unwrap();

        //Default audio spec.
        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };

        let device = sdl2_audio.open_playback(None, &desired_spec, |spec|{
            println!("{:?}", spec);
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap(); // No error handling :(

        AudioSubsystem{ device }
    }

    pub fn resume(&mut self) {
        self.device.resume();
    }

    pub fn pause(&mut self) {
        self.device.pause();
    }

    pub fn get_status(&self) -> AudioStatus {
        self.device.status()
    }
}