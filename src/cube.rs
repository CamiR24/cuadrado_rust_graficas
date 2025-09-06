use nalgebra_glm::Vec3;
use crate::ray_intersect::{RayIntersect, Intersect, Material};
use raylib::color::Color;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, color: Color) -> Self {
        Cube {
            min,
            max,
            material: Material { diffuse: color },
        }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ro: &Vec3, rd: &Vec3) -> Intersect {
        let mut tmin = f32::MIN; // evitar autointersección
        let mut tmax = f32::MAX;

        for i in 0..3 {
            if rd[i].abs() < 1e-8 {
                // Rayo paralelo al eje
                if ro[i] < self.min[i] || ro[i] > self.max[i] {
                    return Intersect::empty(); // fuera del cubo
                }
            } else {
                let inv_d = 1.0 / rd[i];
                let mut t0 = (self.min[i] - ro[i]) * inv_d;
                let mut t1 = (self.max[i] - ro[i]) * inv_d;

                if inv_d < 0.0 {
                    std::mem::swap(&mut t0, &mut t1);
                }

                tmin = tmin.max(t0);
                tmax = tmax.min(t1);

                if tmax < tmin {
                    return Intersect::empty(); // no hay solapamiento
                }
            }
        }

        if tmin > 0.0 {
            Intersect::new(tmin, self.material)
        } else {
            Intersect::empty()
        }
    }
}
