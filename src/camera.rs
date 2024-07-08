use crate::{vec3::Vec3, write_color, HitRecord, Hittable, HittableList, Interval, Ppm, Ray};

pub struct Camera {
    aspect_ratio: f32,
    image_width: f32,
    image_height: f32,
    center: Vec3<f32>,
    pixel00_loc: Vec3<f32>,
    pixel_delta_u: Vec3<f32>,
    pixel_delta_v: Vec3<f32>,
}

macro_rules! f32_len {
    ($v:expr) => {{
        let mut i: i32 = $v.to_bits() as i32;
        i = 0x1fbd3f7d_i32.wrapping_add(i >> 1);
        let y = f32::from_bits(i as u32);
        (((y * y) + $v) / (y)) * 0.5
    }};
}

macro_rules! unit_v {
    ($v:expr) => {
        $v / f32_len!($v.length_squared())
    };
}

impl Camera {
    pub fn initialize() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400.0;
        let mut image_height = image_width / aspect_ratio;
        if image_height < 1.0 {
            image_height = 1.0;
        }
        // Camera
        // viewport height and width may not match the aspect ratio
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * image_width / image_height;

        let center = Vec3::<f32>::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::<f32>::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::<f32>::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vec3::<f32>::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &mut HittableList) {
        let pixels: Vec<(f32, f32, f32)> = (0..self.image_height as usize
            * self.image_width as usize)
            .map(|i| {
                let height = ((i as f32) / self.image_width).round();
                let width = ((i as f32) % self.image_width).round();
                let pixel_center =
                    self.pixel00_loc + (self.pixel_delta_u * width) + (self.pixel_delta_v * height);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);
                let pixel_color = Self::ray_color(&r, world);
                (pixel_color.x(), pixel_color.y(), pixel_color.z())
            })
            .collect();
        let ppm_writer = Ppm::new(
            self.image_width as usize,
            self.image_height as usize,
            255,
            &pixels
                .into_iter()
                .flat_map(write_color)
                .collect::<Vec<u8>>(),
        );
        ppm_writer.write("out.ppm").unwrap();
    }

    fn ray_color(r: &Ray<f32>, world: &mut HittableList) -> Vec3<f32> {
        let mut rec = HitRecord::default();
        if world.hit(
            r,
            Interval {
                min: 0.0,
                max: f32::INFINITY,
            },
            &mut rec,
        ) {
            return (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_direction = unit_v!(r.direction());
        let a = (unit_direction.y() + 1.0) * 0.5;

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }
}
