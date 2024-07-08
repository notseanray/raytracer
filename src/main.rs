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
use camera::*;
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

fn main() {
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

    let cam = Camera::initialize();
    cam.render(&mut world);

    println!("Hello, world!");
}
