const ROUND_THRESHOLD: i8 = 3;

pub fn round_to_threshold(x: f32) -> f32 {
    let factor = 10_f32.powi(ROUND_THRESHOLD as i32);
    (x * factor).round() / factor
}
