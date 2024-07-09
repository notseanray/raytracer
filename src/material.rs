use crate::{
    vec3::{random_unit_vec, reflect, Vec3},
    HitRecord, Ray,
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray<f32>,
        rec: &HitRecord,
        attenuation: &mut Vec3<f32>,
        scattered: &mut Ray<f32>,
    ) -> bool;
}

#[derive(Default)]
pub struct Lambertian {
    pub albedo: Vec3<f32>,
}

macro_rules! near_zero_vec {
    ($v:expr) => {{
        let s = 1e-8;
        $v.x().abs() < s && $v.y().abs() < s && $v.z().abs() < s
    }};
}

impl Lambertian {}

impl Material for Lambertian {
    fn scatter(
            &self,
            _r_in: &Ray<f32>,
            rec: &HitRecord,
            attenuation: &mut Vec3<f32>,
            scattered: &mut Ray<f32>,
        ) -> bool {
        let mut direction = rec.normal + random_unit_vec();
        if near_zero_vec!(direction) {
            direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, direction);
        *attenuation = self.albedo;
        true
    }
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

#[derive(Default)]
pub struct Metal {
    pub attenuation: Vec3<f32>,
    pub fuzz: f32,
}

impl Metal {
}

impl Material for Metal {
    fn scatter(
            &self,
            r_in: &Ray<f32>,
            rec: &HitRecord,
            attenuation: &mut Vec3<f32>,
            scattered: &mut Ray<f32>,
        ) -> bool {
        let reflected = unit_v!(reflect(r_in.direction(), rec.normal)) + random_unit_vec() * self.fuzz;
        let scattered_r = Ray::new(rec.p, reflected);
        *scattered = scattered_r;
        *attenuation = self.attenuation;
        scattered_r.direction().dot(rec.normal) > 0.0
    }
}
