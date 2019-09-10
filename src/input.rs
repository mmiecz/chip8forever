use sdl2::EventPump;

pub struct InputSubsystem {
    event_pump: EventPump,
}

impl InputSubsystem {
    pub fn new(sdl_context: &sdl2::Sdl) -> InputSubsystem {
        InputSubsystem { event_pump: sdl_context.event_pump().unwrap() }
    }
    pub fn update(&mut self) {
        self.event_pump.poll_event();
    }
    pub fn is_key_pressed(&self, key: sdl2::keyboard::Scancode ) -> bool {
        let keyboard_state = self.event_pump.keyboard_state();
        keyboard_state.is_scancode_pressed(key)
    }
    pub fn keys_pressed(&self) -> Vec<sdl2::keyboard::Scancode> {
        let keyboard_state = self.event_pump.keyboard_state();
        keyboard_state.pressed_scancodes().collect()
    }
}

