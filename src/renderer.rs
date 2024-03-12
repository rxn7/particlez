use crate::color::Color;

use fontdue::*;
use glam::Vec2;
use minifb::Window;

pub const SCALE_DIVISOR: usize = 8;

pub struct Renderer {
    pixels: Vec<u32>,
    width: usize,
    height: usize,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let width: usize = width / SCALE_DIVISOR;
        let height: usize = height / SCALE_DIVISOR;

        Self {
            pixels: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn display(&self, window: &mut Window) {
        window
            .update_with_buffer(&self.pixels, self.width, self.height)
            .expect("Failed to display framebuffer");
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        let width: usize = width / SCALE_DIVISOR;
        let height: usize = height / SCALE_DIVISOR;

        self.pixels = vec![0; width * height];
        self.width = width;
        self.height = height;
    }

    pub fn negate_pixel(&mut self, x: f32, y: f32) {
        if x < 0.0 || x >= self.width as f32 || y < 0.0 || y >= self.height as f32 {
            return;
        }

        unsafe {
            let pixel: &mut u32 = self
                .pixels
                .get_unchecked_mut(y as usize * self.width + x as usize);
            *pixel = Color::negative(*pixel)
        }
    }

    pub fn set_pixel(&mut self, x: f32, y: f32, color: u32) {
        if x < 0.0 || x >= self.width as f32 || y < 0.0 || y >= self.height as f32 {
            return;
        }

        unsafe {
            *self
                .pixels
                .get_unchecked_mut(y as usize * self.width + x as usize) = color;
        }
    }

    pub fn draw_square(&mut self, pos: Vec2, size: u32, color: u32) {
        self.set_pixel(pos.x, pos.y, color);
        for x in 0..size {
            for y in 0..size {
                self.set_pixel(pos.x + x as f32, pos.y + y as f32, color);
            }
        }
    }

    pub fn draw_text(
        &mut self,
        font: &Font,
        text: &str,
        pos: Vec2,
        size: f32,
        color: u32,
        smooth: bool,
    ) {
        let mut x: f32 = pos.x;
        let mut y: f32 = pos.y;

        for c in text.chars() {
            match c {
                '\n' => {
                    x = pos.x;
                    y += size;
                }
                _ => {
                    let (metrics, bitmap): (Metrics, Vec<u8>) = font.rasterize(c, size as f32);

                    for yo in 0..metrics.height {
                        for xo in 0..metrics.width {
                            let value: u8 = bitmap[xo + yo * metrics.width];
                            if (smooth && value != 0) || (!smooth && value == 255) {
                                let x: f32 = x + xo as f32;
                                let y: f32 = y + yo as f32;

                                self.set_pixel(
                                    x + metrics.xmin as f32,
                                    y + size - metrics.height as f32 - metrics.ymin as f32,
                                    if smooth {
                                        (color as f32 * (value as f32) / 255.0) as u32
                                    } else {
                                        color
                                    },
                                );
                            }
                        }
                    }

                    x += metrics.advance_width;
                }
            }
        }
    }
}
