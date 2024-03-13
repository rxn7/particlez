use glam::Vec2;
use rand::{distributions::uniform::SampleUniform, rngs::ThreadRng, Rng};

use crate::color::Color;
use crate::particle::{Particle, ParticleOptions};

#[derive(Clone, Default)]
pub enum EmitShape {
    #[default]
    Point,
    Circle(f32), // radius
    Rect(Vec2),  // size,
}

#[derive(Clone, Default)]
pub enum RangedOption<T>
where
    T: SampleUniform,
{
    #[default]
    None,
    Range(std::ops::RangeInclusive<T>),
    Value(T),
}

impl<T> RangedOption<T>
where
    T: SampleUniform + std::cmp::PartialOrd + Copy,
{
    pub fn sample(&self, default: T) -> T {
        let mut rng = rand::thread_rng();
        match self {
            RangedOption::None => default,
            RangedOption::Range(range) => rng.gen_range(range.clone()),
            RangedOption::Value(value) => *value,
        }
    }
}

#[derive(Default, Clone)]
pub struct EmitterOptions {
    pub particle_count: usize, // 0 -> inf
    pub spawn_interval_ms: RangedOption<u32>,
    pub shape: EmitShape,
    pub emit_angle: RangedOption<f32>,
    pub emit_velocity: RangedOption<f32>,
    pub start_size: RangedOption<u32>,
    pub end_size: RangedOption<u32>,
    pub lifetime_ms: RangedOption<u32>,
    pub drag_coefficient: RangedOption<f32>,
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
    next_emit_time: std::time::Instant,
}

impl Emitter {
    pub fn new(position: Vec2, opts: &EmitterOptions) -> Self {
        let remaining_particle_count: usize = opts.particle_count;
        Self {
            position,
            options: opts.clone(),
            remaining_particle_count,
            next_emit_time: std::time::Instant::now(),
        }
    }

    // Returns true if alive
    pub fn update(&mut self, particles: &mut Vec<Particle>) -> bool {
        match &self.options.spawn_interval_ms {
            RangedOption::None => {
                while self.remaining_particle_count > 0 {
                    self.emit(particles);
                }
                return false;
            }

            _ => {
                while !self.next_emit_time.elapsed().is_zero() {
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
        let mut rng: ThreadRng = rand::thread_rng();
        self.next_emit_time = std::time::Instant::now()
            + std::time::Duration::from_millis(self.options.spawn_interval_ms.sample(0) as u64);

        if self.options.particle_count != 0 {
            self.remaining_particle_count -= 1;
        }

        let start_size: u32 = self.options.start_size.sample(1);
        let end_size: u32 = self.options.end_size.sample(0);
        let angle: f32 = (self.options.emit_angle.sample(0.0) - 90.0).to_radians();
        let direction: Vec2 = Vec2::new(angle.cos(), angle.sin());
        let velocity: Vec2 = direction * self.options.emit_velocity.sample(1.0);
        let drag_coefficient: f32 = self.options.drag_coefficient.sample(0.0);

        let start_color = if self.options.start_color_variation == 0.0 {
            self.options.start_color
        } else {
            let hue_shift: f32 = rng
                .gen_range(-self.options.start_color_variation..self.options.start_color_variation);
            Color::hue_shift(self.options.start_color, hue_shift)
        };

        let end_color = if self.options.end_color_variation == 0.0 {
            self.options.end_color
        } else {
            let hue_shift: f32 =
                rng.gen_range(-self.options.end_color_variation..self.options.end_color_variation);
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
                    self.options.lifetime_ms.sample(1000) as u64
                ),
                gravity: self.options.gravity,
                drag_coefficient,
                velocity,
            },
        ))
    }
}
