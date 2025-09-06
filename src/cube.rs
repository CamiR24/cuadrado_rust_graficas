// src/cube.rs - Con cálculo correcto de normales
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
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let mut t_near = f32::NEG_INFINITY;
        let mut t_far = f32::INFINITY;
        let mut normal = Vec3::new(0.0, 0.0, 0.0);

        // Intersección con cada par de planos
        for i in 0..3 {
            if ray_direction[i].abs() < 1e-8 {
                // Rayo paralelo a los planos
                if ray_origin[i] < self.min[i] || ray_origin[i] > self.max[i] {
                    return Intersect::empty();
                }
            } else {
                // Calcular intersecciones con los planos
                let t1 = (self.min[i] - ray_origin[i]) / ray_direction[i];
                let t2 = (self.max[i] - ray_origin[i]) / ray_direction[i];

                let (t_min, t_max) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

                if t_min > t_near {
                    t_near = t_min;
                    // Determinar la normal basada en qué plano golpeamos
                    normal = Vec3::new(0.0, 0.0, 0.0);
                    normal[i] = if ray_direction[i] > 0.0 { -1.0 } else { 1.0 };
                }

                if t_max < t_far {
                    t_far = t_max;
                }

                if t_near > t_far || t_far < 0.0 {
                    return Intersect::empty();
                }
            }
        }

        // Usar la intersección más cercana que sea positiva
        let t = if t_near > 0.001 { t_near } else { t_far };
        
        if t > 0.001 {
            let hit_point = *ray_origin + *ray_direction * t;
            Intersect::new(t, hit_point, normal, self.material)
        } else {
            Intersect::empty()
        }
    }
}