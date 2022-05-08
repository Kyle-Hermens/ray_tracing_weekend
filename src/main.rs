use crate::hittable::*;
use ray::Ray;
use std::io::{stderr, Write};
use vec3::{Color, Point3, Vec3};

mod hittable;
mod ray;
mod vec3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 500;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let mut world = HittableList { objects: vec![] };
    world.objects.push(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    }));

    world.objects.push(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    }));

    let world: Box<dyn Hittable> = Box::new(world);
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
            let pixel_color = ray.ray_color_world(&world);
            println!("{pixel_color}");
        }
    }
    eprintln!("\nDone.");
}

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
