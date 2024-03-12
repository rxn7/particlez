use glam::Vec3;

pub struct Color;

impl Color {
    pub fn to_rgb(c: u32) -> (u8, u8, u8) {
        let r: u8 = ((c >> 16) & 0xFF) as u8;
        let g: u8 = ((c >> 8) & 0xFF) as u8;
        let b: u8 = (c & 0xFF) as u8; 

        (r, g, b)
    }

    pub fn to_rgbf(c: u32) -> Vec3 {
        let (r, g, b) = Color::to_rgb(c);
        Vec3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> u32 {
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    pub fn from_rgbf(rgbf: Vec3) -> u32 {
        let (r, g, b) = (rgbf.x * 255.0, rgbf.y * 255.0, rgbf.z * 255.0);
        Color::from_rgb(r as u8, g as u8, b as u8)
    }

    pub fn lerp(a: u32, b: u32, t: f32) -> u32 {
        let rgbf1 = Color::to_rgbf(a);
        let rgbf2 = Color::to_rgbf(b);
        let result = rgbf1.lerp(rgbf2, t);

        Color::from_rgbf(result)
    }

    pub fn hue_shift(c: u32, delta_degrees: f32) -> u32 {
        const K: Vec3 = Vec3::new(0.57735, 0.57735, 0.57735);

        let (r, g, b) = Color::to_rgb(c);
        let rgbf: Vec3 = Vec3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);

        let delta_radians: f32 = delta_degrees.to_radians();

        let cos_angle: f32 = delta_radians.cos();
        let result: Vec3 = rgbf * cos_angle + K.cross(rgbf) * delta_radians.sin() + K * K.dot(rgbf) * (1.0 - cos_angle);

        Color::from_rgbf(result)
    }

    pub fn negative(c: u32) -> u32 {
        let (r, g, b) = Color::to_rgb(c);
        Color::from_rgb(255 - r, 255 - g, 255 - b)
    }
}
