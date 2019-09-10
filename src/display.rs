use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct DisplaySubsystem {
    canvas: WindowCanvas,
    color: Color,
}

impl DisplaySubsystem {
    pub fn new(context: &Sdl, title: &str, width: u32, height: u32 ) ->  DisplaySubsystem {
        let video_subsystem = context.video().unwrap();
        let mut window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas()
            .present_vsync()
            .build()
            .unwrap();

        DisplaySubsystem { canvas, color: Color::RGB(0, 0, 0,) }
    }
    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn update(&mut self) {
        self.canvas.present();
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
        self.canvas.set_draw_color(self.color);
    }
}