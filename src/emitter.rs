use glam::Vec2;
use rand::{distributions::uniform::SampleUniform, rngs::ThreadRng, Rng};

use crate::particle::{Particle, ParticleOptions};
use crate::color::Color;

#[derive(Clone)]
pub enum EmitShape {
    Point,
    Circle(f32), // radius
    Rect(Vec2),  // size,
}

impl Default for EmitShape {
    fn default() -> Self {
        EmitShape::Point
    }
}

#[derive(Default, Clone)]
pub struct EmitterOptions {
    pub particle_count: usize, // 0 -> inf
    pub spawn_interval: Option<std::time::Duration>,
    pub shape: EmitShape,
    pub emit_angle_range: std::ops::Range<f32>,
    pub emit_velocity_range: std::ops::Range<f32>,
    pub start_size_range: std::ops::Range<u32>,
    pub end_size_range: std::ops::Range<u32>,
    pub lifetime_range_ms: std::ops::Range<u32>,
    pub drag_coefficient_range: std::ops::Range<f32>,
    pub gravity: f32,
    pub start_color: u32,
    pub start_color_variation: f32,
    pub end_color: u32,
    pub end_color_variation: f32,
}

pub struct Emitter {
    pub position: Vec2,
    options: EmitterOptions,
    remaining_particle_count: usize,
    last_emit_time: std::time::Instant,
}

impl Emitter {
    pub fn new(position: Vec2, opts: &EmitterOptions) -> Self {
        let remaining_particle_count: usize = opts.particle_count;
        Self {
            position,
            options: opts.clone(),
            remaining_particle_count,
            last_emit_time: std::time::Instant::now(),
        }
    }

    // Returns true if alive
    pub fn update(&mut self, particles: &mut Vec<Particle>) -> bool {
        match self.options.spawn_interval {
            None => {
                while self.remaining_particle_count > 0 {
                    self.emit(particles);
                }
                return false;
            }

            Some(interval) => {
                while self.last_emit_time.elapsed() > interval {
                    self.emit(particles);
                    if self.options.particle_count != 0 && self.remaining_particle_count == 0 {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn emit(&mut self, particles: &mut Vec<Particle>) {
        self.last_emit_time = std::time::Instant::now();
        if self.options.particle_count != 0 {
            self.remaining_particle_count -= 1;
        }

        let mut rng: ThreadRng = rand::thread_rng();
fn range_or<T>(range: std::ops::Range<T>, or: T, rng: &mut ThreadRng) -> T
        where
            T: PartialOrd,
            T: SampleUniform,
        {
            if range.is_empty() {
                return or;
            }

            rng.gen_range(range)
        }

        let start_size: u32 = range_or(self.options.start_size_range.clone(), 1, &mut rng);
        let end_size: u32 = range_or(self.options.end_size_range.clone(), 0, &mut rng);

        let angle: f32 =
            range_or(self.options.emit_angle_range.clone(), -90.0, &mut rng).to_radians();

        let direction: Vec2 = Vec2::new(angle.cos(), angle.sin());
        let velocity: Vec2 = direction * rng.gen_range(self.options.emit_velocity_range.clone());
        let drag_coefficient: f32 =
            range_or(self.options.drag_coefficient_range.clone(), 0.0, &mut rng);

        let start_color = if self.options.start_color_variation == 0.0 {
            self.options.start_color
        } else {
            let hue_shift: f32 = rng.gen_range(-self.options.start_color_variation..self.options.start_color_variation);
            Color::hue_shift(self.options.start_color, hue_shift)
        };

        let end_color = if self.options.end_color_variation == 0.0 {
            self.options.end_color
        } else {
            let hue_shift: f32 = rng.gen_range(-self.options.end_color_variation..self.options.end_color_variation);
            Color::hue_shift(self.options.end_color, hue_shift)
        };

        // TODO: Apply EmitShape
        //
        particles.push(Particle::new(
            self.position,
            &ParticleOptions {
                start_color,
                end_color,
                start_size,
                end_size,
                lifetime: std::time::Duration::from_millis(
                    rand::thread_rng()
                        .gen_range(self.options.lifetime_range_ms.clone())
                        .into(),
                ),
                gravity: self.options.gravity,
                drag_coefficient,
                velocity,
            },
        ))
    }
}
