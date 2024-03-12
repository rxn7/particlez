mod color;
mod emitter;
mod particle;
mod renderer;
use emitter::{EmitShape, Emitter, EmitterOptions};
use particle::Particle;

use glam::Vec2;
use minifb::*;
use renderer::Renderer;

const WIDTH: usize = 2560;
const HEIGHT: usize = 1280;
const EXPLOSION_EMITTER_OPTS: EmitterOptions = EmitterOptions {
    particle_count: 50,
    spawn_interval: None,
    shape: EmitShape::Point,
    emit_angle_range: 0.0..360.0,
    emit_velocity_range: 50.0..400.0,
    start_size_range: 5..15,
    end_size_range: 0..0,
    lifetime_range_ms: 1000..5000,
    gravity: 98.0,
    start_color: 0xff32a11,
    end_color: 0,
};

fn main() {
    let mut window: Window = Window::new(
        "Particlez",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale_mode: ScaleMode::UpperLeft,
            transparency: false,
            borderless: false,
            resize: true,
            ..Default::default()
        },
    )
    .expect("Unable to create window");

    window.limit_update_rate(None);

    let mut particles: Vec<Particle> = Vec::with_capacity(100);
    let mut emitters: Vec<Emitter> = Vec::with_capacity(10);
    let mut renderer: Renderer = Renderer::new(WIDTH, HEIGHT);

    emitters.push(Emitter::new(
        Vec2::new(WIDTH as f32 / 2.0, HEIGHT as f32),
        EmitterOptions {
            particle_count: 0,
            spawn_interval: Some(std::time::Duration::from_millis(1)),
            shape: EmitShape::Point,
            emit_angle_range: -45.0..45.0,
            emit_velocity_range: 10.0..100.0,
            start_size_range: 10..20,
            end_size_range: 0..0,
            lifetime_range_ms: 1000..5000,
            gravity: 98.0,
            start_color: 0xFF0000,
            end_color: 0,
        },
    ));

    let mut last_frame_start_time: std::time::Instant = std::time::Instant::now();
    let mut mouse_down: bool = false;

    while window.is_open() {
        let frame_delta: std::time::Duration = last_frame_start_time.elapsed();
        last_frame_start_time = std::time::Instant::now();

        let win_size: (usize, usize) = window.get_size();
        if win_size != renderer.canvas_size() {
            renderer.resize(win_size.0, win_size.1);
        }

        renderer.clear();

        if !mouse_down && window.get_mouse_down(MouseButton::Left) {
            let pos = window.get_unscaled_mouse_pos(MouseMode::Clamp).unwrap();
            emitters.push(Emitter::new(
                Vec2::new(pos.0, pos.1),
                EXPLOSION_EMITTER_OPTS,
            ));
        }
        mouse_down = window.get_mouse_down(MouseButton::Left);

        emitters.retain_mut(|e: &mut Emitter| e.update(&mut particles));

        particles.retain_mut(|p: &mut Particle| {
            let alive: bool = p.update(frame_delta);
            p.render(&mut renderer);
            alive
        });

        renderer.display(&mut window);
    }
}
