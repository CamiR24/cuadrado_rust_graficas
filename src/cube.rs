use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use raylib::prelude::Vector3;

pub struct Cube {
    pub center: Vector3,
    pub size: f32, 
    pub material: Material,
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect {
        //calcular min y max en cada eje
        let half_size = self.size * 0.5;
        let min = Vector3::new(
            self.center.x - half_size,
            self.center.y - half_size,
            self.center.z - half_size,
        );
        let max = Vector3::new(
            self.center.x + half_size,
            self.center.y + half_size,
            self.center.z + half_size,
        );
        
        //evitar división por cero
        let inv_dir = Vector3::new(
            if ray_direction.x.abs() < 1e-8 { 1e8 } else { 1.0 / ray_direction.x },
            if ray_direction.y.abs() < 1e-8 { 1e8 } else { 1.0 / ray_direction.y },
            if ray_direction.z.abs() < 1e-8 { 1e8 } else { 1.0 / ray_direction.z },
        );

        //calcular t para entrada y salida en cada eje, como son 6 caras tengo 6 t's
        let t1 = (min.x - ray_origin.x) * inv_dir.x;
        let t2 = (max.x - ray_origin.x) * inv_dir.x;
        let t3 = (min.y - ray_origin.y) * inv_dir.y;
        let t4 = (max.y - ray_origin.y) * inv_dir.y;
        let t5 = (min.z - ray_origin.z) * inv_dir.z;
        let t6 = (max.z - ray_origin.z) * inv_dir.z;

        //obtener min y max para cada eje
        let tmin_x = t1.min(t2);
        let tmax_x = t1.max(t2);
        let tmin_y = t3.min(t4);
        let tmax_y = t3.max(t4);
        let tmin_z = t5.min(t6);
        let tmax_z = t5.max(t6);

        //el punto de entrada es el máximo de los mínimos
        let tmin = tmin_x.max(tmin_y).max(tmin_z);
        //el punto de salida es el mínimo de los máximos
        let tmax = tmax_x.min(tmax_y).min(tmax_z);

        if tmax < 0.0 || tmin > tmax {
            return Intersect::empty();
        }

        //elegir t más cercano y positivo
        let t = if tmin > 0.0 { tmin } else { tmax };
        
        if t <= 0.0 {
            return Intersect::empty();
        }

        //calcular intersección
        let point = *ray_origin + *ray_direction * t;

        //calcular normal
        let normal = self.calculate_normal(&point);

        Intersect::new(point, normal, t, self.material)
    }
}

impl Cube {
    pub fn new(center: Vector3, size: f32, material: Material) -> Self {
        Cube {
            center,
            size,
            material,
        }
    }

    //calcular la normal de la cara intersectada
    fn calculate_normal(&self, point: &Vector3) -> Vector3 {
        let half_size = self.size * 0.5;
        let relative = *point - self.center;
        
        let abs_x = (relative.x.abs() - half_size).abs();
        let abs_y = (relative.y.abs() - half_size).abs();
        let abs_z = (relative.z.abs() - half_size).abs();

        //la normal apunta hacia afuera en la dirección del componente más grande
        if abs_x <= abs_y && abs_x <= abs_z {
            Vector3::new(if relative.x > 0.0 { 1.0 } else { -1.0 }, 0.0, 0.0) //x
        } else if abs_y <= abs_z {
            Vector3::new(0.0, if relative.y > 0.0 { 1.0 } else { -1.0 }, 0.0) //y
        } else {
            Vector3::new(0.0, 0.0, if relative.z > 0.0 { 1.0 } else { -1.0 }) //z
        }
    }
}