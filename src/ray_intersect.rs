// src/ray_intersect.rs - Actualizado con normales correctas
use nalgebra_glm::Vec3;
use raylib::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub is_intersecting: bool,
    pub material: Material,
}

impl Intersect {
    pub fn new(distance: f32, point: Vec3, normal: Vec3, material: Material) -> Self {
        Intersect {
            distance,
            point,
            normal,
            is_intersecting: true,
            material,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            distance: 0.0,
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            is_intersecting: false,
            material: Material {
                diffuse: Color::new(0, 0, 0, 255),
            },
        }
    }
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
}