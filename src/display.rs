#![feature(fixed_size_array)]

use crate::utils::BitVec;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::fmt::{Debug, Error, Formatter};

const PIXEL_SIZE: u32 = 10;

pub struct DisplaySubsystem {
    canvas: WindowCanvas,
    color: Color,
    pixel_buffer: PixelBuffer,
}

impl DisplaySubsystem {
    pub fn new(context: &Sdl, title: &str, width: u32, height: u32) -> DisplaySubsystem {
        let video_subsystem = context.video().unwrap();
        let mut window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();

        DisplaySubsystem {
            canvas,
            color: Color::RGB(0, 0, 0),
            pixel_buffer: PixelBuffer::new(
                (width / PIXEL_SIZE) as usize,
                (height / PIXEL_SIZE) as usize,
            ),
        }
    }

    //TODO: BUG PixelBuffer is not cleared here!
    pub fn clear(&mut self) {
        self.canvas.clear();
    }
    // Draw current pixel buffer to the screen!
    pub fn update(&mut self) {
        self.pixel_buffer.draw_on_canvas(&mut self.canvas);
        self.canvas.present();
    }

    pub fn draw(&mut self, column: usize, row: usize, sprite: Sprite) -> bool {
        let collision = self.pixel_buffer.add_sprite(column, row, sprite);
        collision
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
        self.canvas.set_draw_color(self.color);
    }
}

//pixelbuffer is arrray representing what is shown on the screen.
struct PixelBuffer {
    pixels: Vec<Vec<bool>>,
    columns: usize,
    rows: usize,
}

impl PixelBuffer {
    fn new(columns: usize, rows: usize) -> PixelBuffer {
        PixelBuffer {
            pixels: vec![vec![false; columns]; rows],
            columns,
            rows,
        }
    }
    fn add_sprite(&mut self, column: usize, row: usize, sprite: Sprite) -> bool {
        let mut collision = false;
        for pixel in sprite.into_iter() {
            let (pixel_x, pixel_y) = pixel;
            let (pos_x, pos_y) = (
                (pixel_x + column) % self.columns,
                ((pixel_y + row) % self.rows),
            );
            self.pixels[pos_y][pos_x] ^= true;
            collision |= !self.pixels[pos_y][pos_x];
        }
        println!("add_sprite");
        println!("{:?}", &self);
        println!("add_sprite end");
        collision
    }

    fn draw_on_canvas(&mut self, canvas: &mut WindowCanvas) {
        //println!("{:?}", &self);
        let mut rects: Vec<Rect> = Vec::new();
        for (j, row) in self.pixels.iter().enumerate() {
            for (i, column) in row.into_iter().enumerate() {
                if *column == true {
                    let rect = Rect::new(
                        (i * PIXEL_SIZE as usize) as i32,
                        (j * PIXEL_SIZE as usize) as i32,
                        PIXEL_SIZE,
                        PIXEL_SIZE,
                    );
                    rects.push(rect);
                }
            }
        }
        canvas.fill_rects(&rects);
    }

    fn clear(&mut self) {
        self.pixels = vec![vec![false; self.columns]; self.rows];
    }
}

impl Debug for PixelBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        println!("----PIXELBUFFER DUMP START----");
        for (j, row) in self.pixels.iter().enumerate() {
            for (i, column) in row.into_iter().enumerate() {
                match *column {
                    true => print!("*"),
                    false => print!("-"),
                }
            }
            println!();
        }
        println!("----PIXELBUFFER DUMP END----");
        Ok(())
    }
}

#[derive(Debug)]
pub struct Sprite {
    pixels_on: Vec<(usize, usize)>,
}

impl Sprite {
    pub fn new(bytes: &[u8]) -> Sprite {
        let mut pixels_on: Vec<(usize, usize)> = Vec::new();
        let bits = BitVec::from_bytes(&bytes);
        let mut row = 0;
        for line in bits.as_slice().chunks(8) {
            for (col, bit) in line.into_iter().enumerate() {
                if *bit == true {
                    pixels_on.push((col, row));
                }
            }
            row += 1;
        }
        Sprite { pixels_on }
    }
    pub fn pixels(&self) -> Vec<(usize, usize)> {
        self.pixels_on.clone()
    }
}

//Sprite is a collection of visible pixels positions relative to the beginning of the sprite (0, 0)
impl IntoIterator for Sprite {
    type Item = (usize, usize);
    type IntoIter = std::vec::IntoIter<(usize, usize)>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels_on.into_iter()
    }
}

#[cfg(test)]
mod test {
    use crate::display::Sprite;

    #[test]
    fn test_sprite() {
        let bytes = &[0xF0, 0x90, 0x90, 0x90, 0xF0];
        let sprite = Sprite::new(bytes);
    }
}
