// scene.rs
use raylib::prelude::*;
use crate::cube::Cube;
use crate::material::Material;

const DIFFUSE: Vector3 = Vector3::new(0.9, 0.9, 0.9);

pub fn create_scene_objects() -> Vec<Cube> {
    let stone = Material::with_texture(
        DIFFUSE,
        50.0,
        [0.8, 0.2, 0.0, 0.0],
        0.0,
        'S',
    );

    let diamond = Material::with_texture(
        DIFFUSE,
        75.0,
        [1.0, 0.5, 0.7, 0.8],
        1.0,
        'D',
    );

    let emerald = Material::with_texture(
        DIFFUSE,
        50.0,
        [1.0, 0.5, 0.3, 0.6],
        0.5,
        'E',
    );

    let rubi = Material::with_texture(
        DIFFUSE,
        60.0,
        [0.6, 0.5, 0.1, 0.6],
        0.8,
        'R',
    );

    let aquamarine = Material::with_texture(
        DIFFUSE,
        80.0,
        [0.9, 0.3, 0.1, 0.8],
        0.3,
        'A',
    );

    let mut objects: Vec<Cube> = Vec::new();
    let size = 1.0;
    
    // Nivel -3 (la base)
    objects.push(Cube::new(Vector3::new(0.0, -3.0, 4.0), size, stone));
    objects.push(Cube::new(Vector3::new(1.0, -3.0, 4.0), size, stone));   
    
    // Nivel -2
    objects.push(Cube::new(Vector3::new(-1.0, -2.0, 4.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(-1.0, -2.0, 3.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(0.0, -2.0, 3.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(1.0, -2.0, 3.0), size, stone));
    objects.push(Cube::new(Vector3::new(2.0, -2.0, 3.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(2.0, -2.0, 4.0), size, stone)); 

    // Nivel -1
    objects.push(Cube::new(Vector3::new(-2.0, -1.0, 4.0), size, stone));
    objects.push(Cube::new(Vector3::new(-2.0, -1.0, 3.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(-2.0, -1.0, 2.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(-1.0, -1.0, 2.0), size, stone));
    objects.push(Cube::new(Vector3::new(0.0, -1.0, 2.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(1.0, -1.0, 2.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(2.0, -1.0, 2.0), size, stone));
    objects.push(Cube::new(Vector3::new(3.0, -1.0, 2.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(3.0, -1.0, 3.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(3.0, -1.0, 4.0), size, stone)); 

    // Nivel 0
    for x in -3..=4 {
        objects.push(Cube::new(Vector3::new(x as f32, 0.0, 1.0), size, stone)); // Pared frontal
    }
    for z in 2..=4 {
        objects.push(Cube::new(Vector3::new(-3.0, 0.0, z as f32), size, stone)); // Pared izquierda
        objects.push(Cube::new(Vector3::new(4.0, 0.0, z as f32), size, stone)); // Pared derecha
    }

    // Nivel 1
    for x in -3..=4 {
        objects.push(Cube::new(Vector3::new(x as f32, 1.0, 1.0), size, stone)); 
    }
    for z in 2..=4 {
        objects.push(Cube::new(Vector3::new(-3.0, 1.0, z as f32), size, stone));
        objects.push(Cube::new(Vector3::new(4.0, 1.0, z as f32), size, stone));
    }

    // Nivel 2
    for x in -3..=4 {
        objects.push(Cube::new(Vector3::new(x as f32, 2.0, 1.0), size, stone)); 
    }
    for z in 2..=4 {
        objects.push(Cube::new(Vector3::new(-3.0, 2.0, z as f32), size, stone));
        objects.push(Cube::new(Vector3::new(4.0, 2.0, z as f32), size, stone));
    }

    // Nivel 3
    for x in -3..=4 {
        objects.push(Cube::new(Vector3::new(x as f32, 3.0, 1.0), size, stone)); 
    }
    for z in 2..=4 {
        objects.push(Cube::new(Vector3::new(-3.0, 3.0, z as f32), size, stone));
        objects.push(Cube::new(Vector3::new(4.0, 3.0, z as f32), size, stone));
    }

    // Nivel 4
    objects.push(Cube::new(Vector3::new(-2.0, 4.0, 4.0), size, stone));
    objects.push(Cube::new(Vector3::new(-2.0, 4.0, 3.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(-2.0, 4.0, 2.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(-1.0, 4.0, 2.0), size, stone));
    objects.push(Cube::new(Vector3::new(0.0, 4.0, 2.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(1.0, 4.0, 2.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(2.0, 4.0, 2.0), size, stone));
    objects.push(Cube::new(Vector3::new(3.0, 4.0, 2.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(3.0, 4.0, 3.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(3.0, 4.0, 4.0), size, stone)); 

    // Nivel 5
    objects.push(Cube::new(Vector3::new(-1.0, 5.0, 4.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(-1.0, 5.0, 3.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(0.0, 5.0, 3.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(1.0, 5.0, 3.0), size, stone));
    objects.push(Cube::new(Vector3::new(2.0, 5.0, 3.0), size, stone)); 
    objects.push(Cube::new(Vector3::new(2.0, 5.0, 4.0), size, stone));

    // Nivel 6
    objects.push(Cube::new(Vector3::new(0.0, 6.0, 4.0), size, stone));
    objects.push(Cube::new(Vector3::new(1.0, 6.0, 4.0), size, stone)); 
    
    let gem_size = size * 0.4;

    objects
}