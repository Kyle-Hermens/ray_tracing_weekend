use crate::{Color, Point3, Vec3};

#[derive(Copy, Clone)]
pub struct Ray {
    pub(crate) origin: Point3,
    pub(crate) direction: Vec3,
}

impl Ray {
    pub(crate) fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub(crate) fn ray_color(self) -> Color {
        let mut t = crate::hit_sphere(
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
