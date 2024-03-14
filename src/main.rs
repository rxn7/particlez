mod color;
mod emitter;
mod fps_counter;
mod particle;
mod renderer;
use emitter::{EmitShape, Emitter, EmitterOptions, RangedOption};
use fps_counter::FpsCounter;
use particle::Particle;

use fontdue::{Font, FontSettings};
use glam::Vec2;
use minifb::*;
use renderer::Renderer;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const EXPLOSION_EMITTER_OPTS: EmitterOptions = EmitterOptions {
    particle_count: 100,
    spawn_interval_ms: RangedOption::None,
    shape: EmitShape::Point,
    emit_angle: RangedOption::Range(0.0..=360.0),
    emit_velocity: RangedOption::Range(50.0..=200.0),
    start_size: RangedOption::Range(1..=2),
    end_size: RangedOption::Value(0),
    lifetime_ms: RangedOption::Range(500..=1000),
    drag_coefficient: RangedOption::Range(0.1..=1.0),
    gravity: 250.0,
    start_color: 0xFF8700,
    start_color_variation: 25.0,
    end_color: 0,
    end_color_variation: 0.0,
};
const SPARKS_EMITTER_OPTS: EmitterOptions = EmitterOptions {
    particle_count: 0,
    spawn_interval_ms: RangedOption::Range(1..=50),
    shape: EmitShape::Point,
    emit_angle: RangedOption::Range(0.0..=360.0),
    emit_velocity: RangedOption::Range(5.0..=20.0),
    start_size: RangedOption::Value(1),
    end_size: RangedOption::Value(0),
    lifetime_ms: RangedOption::Range(200..=500),
    drag_coefficient: RangedOption::Range(0.1..=0.5),
    gravity: 250.0,
    start_color: 0xFF8700,
    start_color_variation: 20.0,
    end_color: 0,
    end_color_variation: 0.0,
};

fn main() {
    let mut window: Window = Window::new(
        "Particlez",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale_mode: ScaleMode::AspectRatioStretch,
            transparency: false,
            borderless: false,
            resize: true,
            ..Default::default()
        },
    )
    .expect("Unable to create window");
    window.limit_update_rate(None);

    let font: &[u8] = include_bytes!("../resources/bitach-4x6.ttf") as &[u8];
    let font: Font = Font::from_bytes(
        font,
        FontSettings {
            scale: 8.0,
            ..Default::default()
        },
    )
    .expect("Failed to load font");

    let mut particles: Vec<Particle> = Vec::with_capacity(1000);
    let mut emitters: Vec<Emitter> = Vec::with_capacity(10);
    let mut renderer: Renderer = Renderer::new(WIDTH, HEIGHT);
    let mut fps_counter: FpsCounter = FpsCounter::new();

    let mut last_win_size: (usize, usize) = window.get_size();
    let mut last_start_time: std::time::Instant = std::time::Instant::now();
    let mut last_mouse_down: bool = false;

    emitters.push(Emitter::new(
        Vec2::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
        &SPARKS_EMITTER_OPTS,
    ));

    while window.is_open() {
        let frame_delta: std::time::Duration = last_start_time.elapsed();
        last_start_time = std::time::Instant::now();

        fps_counter.tick();

        let win_size: (usize, usize) = window.get_size();
        if win_size != last_win_size {
            last_win_size = win_size;
            renderer.resize(win_size.0, win_size.1);
        }

        renderer.clear();

        let mut mouse_pos = window.get_unscaled_mouse_pos(MouseMode::Clamp).unwrap();
        mouse_pos.0 /= renderer::SCALE_DIVISOR as f32;
        mouse_pos.1 /= renderer::SCALE_DIVISOR as f32;

        // GOOFY AHH HACK BECAUSE I HATE BORROW CHECKER
        emitters.first_mut().unwrap().position = Vec2::new(mouse_pos.0, mouse_pos.1);

        let mouse_down = window.get_mouse_down(MouseButton::Left);
        if !last_mouse_down && mouse_down {
            emitters.push(Emitter::new(
                Vec2::new(mouse_pos.0, mouse_pos.1),
                &EXPLOSION_EMITTER_OPTS,
            ));
        }
        last_mouse_down = mouse_down;

        emitters.retain_mut(|e: &mut Emitter| e.update(&mut particles));

        particles.retain_mut(|p: &mut Particle| p.update(frame_delta, &mut renderer));

        renderer.draw_text(
            &font,
            format!("fps {}\np {}", fps_counter.fps(), particles.len()).as_str(),
            Vec2::ZERO,
            8.0,
            0xffffff,
            false,
        );

        renderer.display(&mut window);
    }
}
