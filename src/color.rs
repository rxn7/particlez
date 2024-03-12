use glam::Vec3;

pub fn to_rgb(c: u32) -> (u8, u8, u8) {
    let r: u8 = ((c >> 16) & 0xFF) as u8;
    let g: u8 = ((c >> 8) & 0xFF) as u8;
    let b: u8 = (c & 0xFF) as u8; 

    (r, g, b)
}

pub fn from_rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn lerp(a: u32, b: u32, t: f32) -> u32 {
    let (r1, g1, b1) = to_rgb(a);
    let r1 = r1 as f32 / 255.0;
    let g1 = g1 as f32 / 255.0;
    let b1 = b1 as f32 / 255.0;

    let (r2, g2, b2) = to_rgb(b);
    let r2 = r2 as f32 / 255.0;
    let g2 = g2 as f32 / 255.0;
    let b2 = b2 as f32 / 255.0;

    let r = r1 + t * (r2 - r1);
    let g = g1 + t * (g2 - g1);
    let b = b1 + t * (b2 - b1);

    ((r * 255.0) as u32) << 16 | ((g * 255.0) as u32) << 8 | (b * 255.0) as u32
}

pub fn hue_shift(c: u32, delta_degrees: f32) -> u32 {
    const K: Vec3 = Vec3::new(0.57735, 0.57735, 0.57735);

    let (r, g, b) = to_rgb(c);
    let rgbf: Vec3 = Vec3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);

    let delta_radians: f32 = delta_degrees.to_radians();

    let cos_angle: f32 = delta_radians.cos();
    let result: Vec3 = rgbf * cos_angle + K.cross(rgbf) * delta_radians.sin() + K * K.dot(rgbf) * (1.0 - cos_angle);
    let (r, g, b) = (result.x * 255.0, result.y * 255.0, result.z * 255.0);

    from_rgb(r as u8, g as u8, b as u8)
}
