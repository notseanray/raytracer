use crate::Interval;

#[inline(always)]
pub fn write_color(pixel_color: (f32, f32, f32)) -> Vec<u8> {
    let r = pixel_color.0;
    let g = pixel_color.1;
    let b = pixel_color.2;

    let intensity = Interval {
        min: 0.000,
        max: 0.999,
    };
    let r_b = (256.0 * intensity.clamp(r)) as u8;
    let g_b = (256.0 * intensity.clamp(g)) as u8;
    let b_b = (256.0 * intensity.clamp(b)) as u8;

    vec![r_b, g_b, b_b]
}
