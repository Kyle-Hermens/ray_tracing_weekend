use std::fmt::{Display, Formatter};
use std::io::{stderr, Write};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 500;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::default();
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };

    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    //Render
    print!("P3\n{image_width} {image_height}\n255\n");
    //
    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {j} ");
        stderr().flush().expect("Failed to flush somehow");
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let ray = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let pixel_color = ray.ray_color();
            // let vec = Vec3 {
            //     x: (i as f64 / (image_width - 1) as f64),
            //     y: (j as f64 / (image_height - 1) as f64),
            //     z: 0.25,
            // };
            println!("{pixel_color}");
        }
    }
    eprintln!("\nDone.");
}

#[derive(Copy, Clone, Default)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ir = (255.999 * self.x) as i32;
        let ig = (255.999 * self.y) as i32;
        let ib = (255.999 * self.z) as i32;
        write!(f, "{} {} {}", ir, ig, ib)
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1_f64 / rhs) * self
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}

impl Vec3 {
    fn dot(u: Vec3, v: Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}
type Color = Vec3;
type Point3 = Vec3;

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = Vec3::dot(oc, r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}
#[derive(Copy, Clone)]
struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    fn ray_color(self) -> Color {
        let mut t = hit_sphere(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
            self,
        );
        if t > 0.0 {
            let n = (self.at(t)
                - Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                })
            .unit_vector();

            return 0.5
                * Vec3 {
                    x: n.x + 1.0,
                    y: n.y + 1.0,
                    z: n.z + 1.0,
                };
        }

        let unit_direction = self.direction.unit_vector();
        t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t)
            * Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
            + t * Vec3 {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            }
    }
}

struct HitRecord {
    p: Point3,
    normal: Vec3,
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

trait Hittable {
    fn hit(&self, _r: Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord) -> bool {
        false
    }
}

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        };

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&r, outward_normal);

        true
    }
}
