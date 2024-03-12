use glam::Vec2;
use minifb::Window;

pub struct Renderer {
    pixels: Vec<u32>,
    width: usize,
    height: usize,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn display(&self, window: &mut Window) {
        window
            .update_with_buffer(&self.pixels, self.width, self.height)
            .expect("Failed to display framebuffer");
    }

    pub fn canvas_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.pixels = vec![0; width * height];
        self.width = width;
        self.height = height;
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
}
