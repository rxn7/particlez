use glam::Vec2;

use crate::renderer::Renderer;
use crate::color::Color;

#[derive(Default, Clone, Copy)]
pub struct ParticleOptions {
    pub lifetime: std::time::Duration,
    pub drag_coefficient: f32,
    pub velocity: Vec2,
    pub start_size: u32,
    pub end_size: u32,
    pub start_color: u32,
    pub end_color: u32,
    pub gravity: f32,
}

pub struct Particle {
    options: ParticleOptions,
    position: Vec2,
    size: u32,
    velocity: Vec2,
    gravity_force: f32,
    color: u32,
    spawn_time: std::time::Instant,
}

impl Particle {
    pub fn new(position: Vec2, options: &ParticleOptions) -> Self {
        return Self {
            position,
            size: options.start_size,
            velocity: options.velocity,
            color: options.start_color,
            gravity_force: 0.0,
            spawn_time: std::time::Instant::now(),
            options: options.clone(),
        };
    }

    pub fn age(&self) -> std::time::Duration {
        std::time::Instant::now() - self.spawn_time
    }

    // Returns true if alive
    pub fn update(&mut self, frame_delta: std::time::Duration, renderer: &mut Renderer) -> bool {
        if self.position.y < 0.0 || self.position.y > renderer.height() as f32 || self.position.x < 0.0 || self.position.x > renderer.width() as f32 {
            return false;
        }

        if self.age() > self.options.lifetime {
            return false;
        }

        let age_ratio: f32 = self.age().as_secs_f32() / self.options.lifetime.as_secs_f32();

        self.velocity = self.velocity.lerp(
            Vec2::ZERO,
            self.options.drag_coefficient * frame_delta.as_secs_f32(),
        );

        self.gravity_force += self.options.gravity * frame_delta.as_secs_f32();
        self.position += self.velocity * frame_delta.as_secs_f32()
            + Vec2::Y * self.gravity_force * frame_delta.as_secs_f32();
        self.color =
            Color::lerp(self.options.start_color, self.options.end_color, age_ratio);

        renderer.draw_square(self.position, self.size, self.color);
        true
    }
}
