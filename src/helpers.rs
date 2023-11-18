pub fn normalize(x: u8) -> f32 {
    (x as f32) / 255.0
}

pub fn denormalize(v: f32) -> u8 {
    f32::round(v * 255.0) as u8
}
