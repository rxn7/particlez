use glam::Vec3;

pub struct Color;

impl Color {
    #[inline]
    pub fn to_rgb(c: u32) -> (u8, u8, u8) {
        let r: u8 = ((c >> 16) & 0xFF) as u8;
        let g: u8 = ((c >> 8) & 0xFF) as u8;
        let b: u8 = (c & 0xFF) as u8; 

        (r, g, b)
    }

    #[inline]
    pub fn to_rgbf(c: u32) -> Vec3 {
        let (r, g, b) = Color::to_rgb(c);
        Vec3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> u32 {
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    #[inline]
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

    #[inline]
    pub fn negative(c: u32) -> u32 {
        let (r, g, b) = Color::to_rgb(c);
        Color::from_rgb(255 - r, 255 - g, 255 - b)
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn to_rgb() {
        let c = 0xFFAAEEu32;
        let (r, g, b) = Color::to_rgb(c);
        assert_eq!(r, 255);
        assert_eq!(g, 170);
        assert_eq!(b, 238);
    }

    #[test]
    fn from_rgb() {
        let (r, g, b) = (120, 50, 250);
        let c = Color::from_rgb(r, g, b);
        assert_eq!(c, 0x7832FAu32);
    }

    #[test]
    fn to_rgbf() {
        let c = 0xAAFF52u32;
        let rgbf = Color::to_rgbf(c);
        let c2 = Color::from_rgbf(rgbf);
        assert_eq!(c, c2);
    }

    #[test]
    fn from_rgbf() {
        let c = 0x4D80B3u32;
        let rgbf = Color::to_rgbf(c);
        let c2 = Color::from_rgbf(rgbf);
        assert_eq!(c, c2);
    }

    #[test]
    fn negative() {
        let c = 0x52a5a9u32;
        let n = Color::negative(c);
        let (r, g, b) = Color::to_rgb(n);
        assert_eq!(r, 255 - 82);
        assert_eq!(g, 255 - 165);
        assert_eq!(b, 255 - 169);
    }
}
