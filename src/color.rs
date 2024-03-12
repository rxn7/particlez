pub fn lerp(a: u32, b: u32, t: f32) -> u32 {
    let r1 = ((a >> 16) & 0xFF) as f32 / 255.0;
    let g1 = ((a >> 8) & 0xFF) as f32 / 255.0;
    let b1 = (a & 0xFF) as f32 / 255.0;

    let r2 = ((b >> 16) & 0xFF) as f32 / 255.0;
    let g2 = ((b >> 8) & 0xFF) as f32 / 255.0;
    let b2 = (b & 0xFF) as f32 / 255.0;

    let r = r1 + t * (r2 - r1);
    let g = g1 + t * (g2 - g1);
    let b = b1 + t * (b2 - b1);

    ((r * 255.0) as u32) << 16 | ((g * 255.0) as u32) << 8 | (b * 255.0) as u32
}
