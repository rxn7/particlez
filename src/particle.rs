use glam::Vec2;

use crate::renderer::Renderer;

#[derive(Default, Clone, Copy)]
pub struct ParticleOptions {
    pub lifetime: std::time::Duration,
    pub velocity: Vec2,
    pub start_size: u32,
    pub end_size: u32,
    pub start_color: u32,
    pub end_color: u32,
    pub gravity: f32,
}

pub struct Particle {
    spawn_options: ParticleOptions,
    position: Vec2,
    size: u32,
    velocity: Vec2,
    color: u32,
    spawn_time: std::time::Instant,
}

impl Particle {
    pub fn new(position: Vec2, options: ParticleOptions) -> Self {
        return Self {
            position,
            size: options.start_size,
            velocity: options.velocity,
            color: options.start_color,
            spawn_time: std::time::Instant::now(),
            spawn_options: options,
        };
    }

    pub fn age(&self) -> std::time::Duration {
        std::time::Instant::now() - self.spawn_time
    }

    // Returns true if alive
    pub fn update(&mut self, frame_delta: std::time::Duration) -> bool {
        if self.age() > self.spawn_options.lifetime {
            return false;
        }

        let age_ratio: f32 = self.age().as_secs_f32() / self.spawn_options.lifetime.as_secs_f32();

        self.velocity.y += self.spawn_options.gravity * frame_delta.as_secs_f32();
        self.position += self.velocity * frame_delta.as_secs_f32();
        self.color = crate::color::lerp(
            self.spawn_options.start_color,
            self.spawn_options.end_color,
            age_ratio,
        );

        true
    }

    pub fn render(&self, renderer: &mut Renderer) {
        renderer.draw_square(self.position, self.size, self.color);
    }
}
