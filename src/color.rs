use crate::{f32_len, Interval};

#[inline(always)]
fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        return f32_len!(linear_component);
    }
    0.0
}

#[inline(always)]
pub fn write_color(pixel_color: (f32, f32, f32)) -> Vec<u8> {
    let r = linear_to_gamma(pixel_color.0);
    let g = linear_to_gamma(pixel_color.1);
    let b = linear_to_gamma(pixel_color.2);

    let intensity = Interval {
        min: 0.000,
        max: 0.999,
    };
    let r_b = (256.0 * intensity.clamp(r)) as u8;
    let g_b = (256.0 * intensity.clamp(g)) as u8;
    let b_b = (256.0 * intensity.clamp(b)) as u8;

    vec![r_b, g_b, b_b]
}
