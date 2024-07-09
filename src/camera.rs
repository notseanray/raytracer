use crate::{
    random_double,
    vec3::{random_on_hemisphere, random_unit_vec, Vec3},
    write_color, HitRecord, Hittable, HittableList, Interval, Ppm, Ray,
};

pub struct Camera {
    image_width: f32,
    image_height: f32,
    center: Vec3<f32>,
    pixel00_loc: Vec3<f32>,
    pixel_delta_u: Vec3<f32>,
    pixel_delta_v: Vec3<f32>,
    samples_per_pixel: f32,
    pixel_samples_scale: f32,
    max_depth: usize,
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
        let samples_per_pixel = 100.0;
        let pixel_samples_scale = 1.0 / samples_per_pixel;
        let max_depth = 50;
        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
        }
    }

    fn sample_square() -> Vec3<f32> {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn get_ray(&self, width: f32, height: f32) -> Ray<f32> {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (width + offset.x()))
            + (self.pixel_delta_v * (height + offset.y()));
        let ray_direction = pixel_sample - self.center;
        Ray {
            origin: self.center,
            direction: ray_direction,
        }
    }

    pub fn render(&self, world: &mut HittableList) {
        let pixels: Vec<(f32, f32, f32)> = (0..self.image_height as usize
            * self.image_width as usize)
            .map(|i| {
                let height = ((i as f32) / self.image_width) as u32 as f32;
                let width = ((i as f32) % self.image_width) as u32 as f32;
                //let r = Ray::new(self.center, ray_direction);
                let mut color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel as usize {
                    let r = self.get_ray(width, height);
                    color += Self::ray_color(&r, self.max_depth, world);
                }
                color = color * self.pixel_samples_scale;
                (color.x(), color.y(), color.z())
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

    fn ray_color(r: &Ray<f32>, depth: usize, world: &mut HittableList) -> Vec3<f32> {
        if depth == 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::default();
        if world.hit(
            r,
            Interval {
                min: 0.025,
                max: f32::INFINITY,
            },
            &mut rec,
        ) {
            let mut scattered = Ray::default();
            let mut attenuation = Vec3::<f32>::default();
            if let Some(mat) = &rec.material {
                if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * Self::ray_color(&scattered, depth - 1, world);
                }
            }
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let unit_direction = unit_v!(r.direction());
        let a = (unit_direction.y() + 1.0) * 0.5;

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }
}
