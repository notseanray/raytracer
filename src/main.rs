#![feature(strict_provenance)]
mod color;
use color::*;
mod ppm;
use ppm::*;
mod ray;
use ray::*;
use vec3::Vec3;
mod vec3;

fn hit_sphere(center: Vec3<f32>, radius: f32, r: &Ray<f32>) -> bool {
    let oc = center - r.origin();
    let a = r.direction().dot(r.direction());
    let b = r.direction().dot(oc) * -2.0;
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(r: Ray<f32>) -> Vec3<f32> {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let direction = r.direction();
    let unit_direction = direction / f32_len!(direction.length_squared());
    let a = (unit_direction.y() + 1.0) * 0.5;
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}

fn main() {
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
    let camera_center = Vec3::<f32>::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::<f32>::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::<f32>::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
        - Vec3::<f32>::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let pixels: Vec<(f32, f32, f32)> = (0..image_height as usize * image_width as usize)
        .map(|i| {
            let height = ((i as f32) / image_width).round();
            let width = ((i as f32) % image_width).round();
            let pixel_center = pixel00_loc + (pixel_delta_u * width) + (pixel_delta_v * height);
            let ray_direction = pixel_center - camera_center;
            let pixel_color = ray_color(Ray::new(camera_center, ray_direction));
            (pixel_color.x(), pixel_color.y(), pixel_color.z())
        })
        .collect();
    let ppm_writer = Ppm::new(
        image_width as usize,
        image_height as usize,
        255,
        &pixels
            .into_iter()
            .flat_map(write_color)
            .collect::<Vec<u8>>(),
    );
    ppm_writer.write("out.ppm").unwrap();
    println!("Hello, world!");
}
