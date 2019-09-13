use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Scancode;
use std::iter::Scan;

pub struct InputSubsystem {
    event_pump: EventPump,
}

impl InputSubsystem {
    pub fn new(sdl_context: &sdl2::Sdl) -> InputSubsystem {
        InputSubsystem {
            event_pump: sdl_context.event_pump().unwrap(),
        }
    }
    pub fn poll(&mut self) -> Option<Event> {
        self.event_pump.poll_event()
    }
    pub fn is_key_pressed(&self, key: sdl2::keyboard::Scancode) -> bool {
        let keyboard_state = self.event_pump.keyboard_state();
        keyboard_state.is_scancode_pressed(key)
    }
    pub fn keys_pressed(&self) -> Vec<sdl2::keyboard::Scancode> {
        let keyboard_state = self.event_pump.keyboard_state();
        keyboard_state.pressed_scancodes().collect()
    }

    pub fn wait_for_keypress(&mut self, scancode: sdl2::keyboard::Scancode) {
        'wait: loop {
            match self.event_pump.wait_event() {
                Event::KeyDown {
                    scancode: Some(code),
                    ..
                } if code == scancode => {
                    println!("{:?} keydown scancode", code);
                    break 'wait;
                }
                _ => {}
            }
        }
    }
}

pub struct KeyboardMapper;
/*

InEmulator Keyobard maps to:
1	2	3	C       1 2 3 4
4	5	6	D   =>  q w e r
7	8	9	E   =>  a s d f
A	0	B	F       z x c v


 */
impl KeyboardMapper {
    pub fn map_to_scancode( keycode: u8) -> Option<sdl2::keyboard::Scancode> {
        match keycode {
            0x0 => Some(Scancode::X),
            0x1 => Some(Scancode::Num1),
            0x2 => Some(Scancode::Num2),
            0x3 => Some(Scancode::Num3),
            0x4 => Some(Scancode::Q),
            0x5 => Some(Scancode::W),
            0x6 => Some(Scancode::E),
            0x7 => Some(Scancode::A),
            0x8 => Some(Scancode::S),
            0x9 => Some(Scancode::D),
            0xA => Some(Scancode::Z),
            0xB => Some(Scancode::C),
            0xC => Some(Scancode::Num4),
            0xD => Some(Scancode::R),
            0xE => Some(Scancode::F),
            0xF => Some(Scancode::V),
            _ => None
        }
    }
}
