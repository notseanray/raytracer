#![feature(strict_provenance)]
mod color;
use std::f32::consts::PI;

use color::*;
mod ppm;
use ppm::*;
mod ray;
use ray::*;
use vec3::Vec3;
mod camera;
mod interval;
mod vec3;
use interval::*;

#[derive(Default)]
struct HitRecord {
    pub p: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray<f32>, outward_normal: Vec3<f32>) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

trait Hittable {
    fn hit(&self, r: &Ray<f32>, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,
}

#[macro_export]
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

impl Hittable for Sphere {
    fn hit(&self, r: &Ray<f32>, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f32_len!(discriminant);
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        let p = r.at(root);
        *rec = HitRecord {
            p,
            normal: (p - self.center) / self.radius,
            t: root,
            front_face: false,
        };
        rec.set_face_normal(r, rec.normal);
        true
    }
}

struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, r: &Ray<f32>, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(
                r,
                Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                hit_record.p = temp_rec.p;
                hit_record.normal = temp_rec.normal;
                hit_record.t = temp_rec.t;
                hit_record.front_face = temp_rec.front_face;
            }
        }
        hit_anything
    }
}

fn hit_sphere(center: Vec3<f32>, radius: f32, r: &Ray<f32>) -> f32 {
    let oc = center - r.origin();
    let a = r.direction().length_squared();
    let h = r.direction().dot(oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - f32_len!(discriminant)) / a
    }
}

/*
fn ray_color(r: &Ray<f32>) -> Vec3<f32> {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let normal_vec = unit_v!(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return Vec3::new(
            normal_vec.x() + 1.0,
            normal_vec.y() + 1.0,
            normal_vec.z() + 1.0,
        ) * 0.5;
    }
    let direction = r.direction();
    let unit_direction = unit_v!(direction);
    let a = (unit_direction.y() + 1.0) * 0.5;
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}
*/

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

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;

    let mut image_height = image_width / aspect_ratio;
    if image_height < 1.0 {
        image_height = 1.0;
    }

    let mut world = HittableList {
        objects: vec![
            Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
            }),
        ],
    };

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
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r, &mut world);
            //let pixel_color = ray_color(&r);
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
