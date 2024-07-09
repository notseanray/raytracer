#![feature(strict_provenance)]
mod color;
use rand::Rng;
use std::f32::consts::PI;
use std::rc::Rc;

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
mod material;
use material::*;

pub struct HitRecord {
    pub p: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub t: f32,
    pub front_face: bool,
    pub material: Option<Rc<Box<dyn Material>>>,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
            material: None,
        }
    }
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
    fn hit(
        &self,
        r: &Ray<f32>,
        ray_t: Interval,
        rec: &mut HitRecord,
        material: Rc<Box<dyn Material>>,
    ) -> bool;
    fn material(&self) -> Rc<Box<dyn Material>>;
}

struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,
    pub material: Rc<Box<dyn Material>>,
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

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &Ray<f32>,
        ray_t: Interval,
        rec: &mut HitRecord,
        material: Rc<Box<dyn Material>>,
    ) -> bool {
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
            material: Some(material),
        };
        rec.set_face_normal(r, rec.normal);
        true
    }

    fn material(&self) -> Rc<Box<dyn Material>> {
        self.material.clone()
    }
}

struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

#[inline(always)]
pub fn random_double() -> f32 {
    let mut rng = rand::thread_rng();
    let random_number: f32 = rng.gen();
    random_number
}

#[inline(always)]
pub fn random_double_lim(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let random_number: f32 = rng.gen_range(min..max);
    random_number
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
                object.material(),
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                hit_record.p = temp_rec.p;
                hit_record.normal = temp_rec.normal;
                hit_record.t = temp_rec.t;
                hit_record.front_face = temp_rec.front_face;
                hit_record.material = Some(object.material());
            }
        }
        hit_anything
    }
}

fn main() {
    let ground = Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    };
    let center = Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    };
    let left = Metal {
        attenuation: Vec3::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    };
    let right = Metal {
        attenuation: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };
    let mut world = HittableList {
        objects: vec![
            Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: Rc::new(Box::new(center)),
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: Rc::new(Box::new(ground)),
            }),
            Box::new(Sphere {
                center: Vec3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: Rc::new(Box::new(left)),
            }),
            Box::new(Sphere {
                center: Vec3::new(1.0, 0.0, -1.0),
                radius: 0.5,
                material: Rc::new(Box::new(right)),
            }),
        ],
    };

    let cam = Camera::initialize();
    cam.render(&mut world);

    println!("Hello, world!");
}
