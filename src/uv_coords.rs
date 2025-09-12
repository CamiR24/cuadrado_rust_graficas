// uv_coords.rs - Coordenadas UV simplificadas para TextureManager (toda esta clase me ayudó Claude)
use raylib::prelude::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct UVCoordinates {
    pub u: f32,
    pub v: f32,
}

impl UVCoordinates {
    pub fn new(u: f32, v: f32) -> Self {
        UVCoordinates { u, v }
    }

    // Convertir coordenadas UV (0.0-1.0) a coordenadas de texel para tu TextureManager
    // Tu TextureManager usa 128x128, así que escalamos a ese tamaño
    pub fn to_texture_coords(&self, texture_size: u32) -> (u32, u32) {
        let tx = (self.u * (texture_size - 1) as f32) as u32;
        let ty = (self.v * (texture_size - 1) as f32) as u32;
        (tx.min(texture_size - 1), ty.min(texture_size - 1))
    }
}

// Función para calcular coordenadas UV basadas en la cara del cubo y el punto de intersección
pub fn calculate_cube_uv(point: &Vector3, center: &Vector3, size: f32, normal: &Vector3) -> UVCoordinates {
    let half_size = size * 0.5;
    let relative = *point - *center;
    
    // Determinar qué cara y calcular UV correspondientes
    if normal.x.abs() > 0.5 {
        // Caras X (izquierda/derecha)
        let u = (relative.z + half_size) / size;
        let v = (-relative.y + half_size) / size; // Invertir Y para orientación correcta
        UVCoordinates::new(u, v)
    } else if normal.y.abs() > 0.5 {
        // Caras Y (arriba/abajo)  
        let u = (relative.x + half_size) / size;
        let v = (relative.z + half_size) / size;
        UVCoordinates::new(u, v)
    } else {
        // Caras Z (frente/atrás)
        let u = if normal.z > 0.0 {
            (relative.x + half_size) / size
        } else {
            (-relative.x + half_size) / size
        };
        let v = (-relative.y + half_size) / size; // Invertir Y
        UVCoordinates::new(u, v)
    }
}