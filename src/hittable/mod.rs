use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

mod list;
mod sphere;

#[derive(Copy, Clone, Default)]
pub struct HitRecord {
    p: Point3,
    pub(crate) normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, _r: Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        false
    }
}

pub use self::list::*;
pub use self::sphere::*;
