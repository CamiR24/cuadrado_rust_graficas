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
        [1.0, 0.5, 0.4, 0.1],
        1.5,
        'D',
    );

    let emerald = Material::with_texture(
        DIFFUSE,
        50.0,
        [0.8, 0.2, 0.4, 0.0],
        0.5,
        'E',
    );

    let rubi = Material::with_texture(
        DIFFUSE,
        60.0,
        [0.8, 0.2, 0.3, 0.0],
        0.8,
        'R',
    );

    let aquamarine = Material::with_texture(
        DIFFUSE,
        80.0,
        [0.8, 0.2, 0.5, 0.0],
        0.3,
        'A',
    );

    let glass = Material::new(
        Vector3::new(0.9, 0.9, 0.9), 
        200.0,
        [0.0, 0.0, 0.05, 0.95],      
        1.5,                          
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

    //pantalla de vidrio
    objects.push(Cube::new(Vector3::new(-0.5, 5.0, 4.5), 0.5, glass));
    objects.push(Cube::new(Vector3::new(0.0, 5.0, 4.5), 0.5, glass));
    objects.push(Cube::new(Vector3::new(0.5, 5.0, 4.5), 0.5, glass));
    objects.push(Cube::new(Vector3::new(1.0, 5.0, 4.5), 0.5, glass));
    objects.push(Cube::new(Vector3::new(-0.5, 4.5, 4.5), 0.5, glass));
    objects.push(Cube::new(Vector3::new(0.0, 4.5, 4.5), 0.5, glass));
    objects.push(Cube::new(Vector3::new(0.5, 4.5, 4.5), 0.5, glass));
    objects.push(Cube::new(Vector3::new(1.0, 4.5, 4.5), 0.5, glass));

    /*objects.push(Cube::new(Vector3::new(0.0, 4.0, 4.0), size, glass));
    objects.push(Cube::new(Vector3::new(1.0, 4.0, 4.0), size, glass));
    objects.push(Cube::new(Vector3::new(-1.0, 4.0, 4.0), size, glass));
    objects.push(Cube::new(Vector3::new(2.0, 4.0, 4.0), size, glass));

    objects.push(Cube::new(Vector3::new(0.0, 3.0, 4.0), size, glass));
    objects.push(Cube::new(Vector3::new(1.0, 3.0, 4.0), size, glass));
    objects.push(Cube::new(Vector3::new(-1.0, 3.0, 4.0), size, glass));
    objects.push(Cube::new(Vector3::new(2.0, 3.0, 4.0), size, glass));
    objects.push(Cube::new(Vector3::new(-2.0, 3.0, 4.0), size, glass));
    objects.push(Cube::new(Vector3::new(3.0, 3.0, 4.0), size, glass));

    objects.push(Cube::new(Vector3::new(0.0, 2.0, 4.0), size, glass));
    objects.push(Cube::new(Vector3::new(1.0, 2.0, 4.0), size, glass));
    objects.push(Cube::new(Vector3::new(-1.0, 2.0, 4.0), size, glass));
    objects.push(Cube::new(Vector3::new(2.0, 2.0, 4.0), size, glass));
    objects.push(Cube::new(Vector3::new(-2.0, 2.0, 4.0), size, glass));
    objects.push(Cube::new(Vector3::new(3.0, 2.0, 4.0), size, glass));*/

    //DIAMANTES - ESTRICTAMENTE dentro del espacio hueco
    objects.push(Cube::new(Vector3::new(-1.8, -0.8, 2.5), 0.4, diamond)); // Pared izquierda
    objects.push(Cube::new(Vector3::new(2.5, -0.7, 3.0), 0.5, diamond));  // Pared derecha
    objects.push(Cube::new(Vector3::new(0.2, -0.9, 3.1), 0.3, diamond));  // Centro fondo
    objects.push(Cube::new(Vector3::new(-1.9, 0.5, 3.0), 0.6, diamond));  // Pared izquierda medio
    objects.push(Cube::new(Vector3::new(2.7, 0.8, 2.8), 0.4, diamond));   // Pared derecha medio
    objects.push(Cube::new(Vector3::new(-0.3, -0.6, 2.0), 0.5, diamond)); // Centro frente
    objects.push(Cube::new(Vector3::new(-1.6, 1.2, 2.1), 0.3, diamond));  // Pared izquierda alto
    objects.push(Cube::new(Vector3::new(2.4, 1.5, 3.1), 0.4, diamond));   // Pared derecha alto
    objects.push(Cube::new(Vector3::new(1.2, -0.8, 2.4), 0.6, diamond));  // Centro derecha bajo
    objects.push(Cube::new(Vector3::new(-1.3, -0.7, 2.9), 0.5, diamond)); // Centro izquierda bajo
    objects.push(Cube::new(Vector3::new(-1.9, 2.3, 3.0), 0.4, diamond));  // Pared izquierda muy alto
    objects.push(Cube::new(Vector3::new(2.6, 2.1, 2.2), 0.3, diamond));   // Pared derecha muy alto
    objects.push(Cube::new(Vector3::new(0.8, 3.7, 2.8), 0.5, diamond));   // Cerca del techo centro
    objects.push(Cube::new(Vector3::new(-1.1, 3.9, 2.5), 0.4, diamond));  // Cerca del techo izq
    objects.push(Cube::new(Vector3::new(1.9, 3.8, 2.9), 0.6, diamond));   // Cerca del techo der
    objects.push(Cube::new(Vector3::new(-1.7, 0.1, 1.9), 0.3, diamond));  // Pared izquierda frente
    objects.push(Cube::new(Vector3::new(2.2, 0.3, 1.9), 0.5, diamond));   // Pared derecha frente
    objects.push(Cube::new(Vector3::new(-0.6, 1.3, 3.1), 0.4, diamond));  // Fondo centro alto
    objects.push(Cube::new(Vector3::new(1.7, 2.8, 3.0), 0.3, diamond));   // Fondo alto derecha
    objects.push(Cube::new(Vector3::new(-0.9, 2.5, 2.9), 0.5, diamond));  // Fondo alto izquierda

    //ESMERALDAS - ESTRICTAMENTE dentro del espacio hueco
    objects.push(Cube::new(Vector3::new(-1.7, -0.6, 2.7), 0.5, emerald)); // Pared izquierda bajo
    objects.push(Cube::new(Vector3::new(2.5, -0.4, 2.4), 0.4, emerald));  // Pared derecha bajo
    objects.push(Cube::new(Vector3::new(-0.8, -0.8, 2.3), 0.6, emerald)); // Centro izquierda bajo
    objects.push(Cube::new(Vector3::new(1.4, -0.7, 2.9), 0.3, emerald));  // Centro derecha bajo
    objects.push(Cube::new(Vector3::new(-1.8, 0.8, 3.1), 0.5, emerald));  // Pared izquierda medio
    objects.push(Cube::new(Vector3::new(2.6, 0.4, 2.0), 0.4, emerald));   // Pared derecha medio
    objects.push(Cube::new(Vector3::new(0.4, 1.2, 3.0), 0.3, emerald));   // Centro fondo medio
    objects.push(Cube::new(Vector3::new(-1.9, 2.4, 2.6), 0.5, emerald));  // Pared izquierda alto
    objects.push(Cube::new(Vector3::new(2.4, 2.0, 3.0), 0.6, emerald));   // Pared derecha alto
    objects.push(Cube::new(Vector3::new(-1.2, 3.8, 2.9), 0.4, emerald));  // Cerca techo izquierda
    objects.push(Cube::new(Vector3::new(1.6, 3.9, 2.3), 0.3, emerald));   // Cerca techo derecha
    objects.push(Cube::new(Vector3::new(0.1, 3.7, 2.1), 0.5, emerald));   // Cerca techo frente
    objects.push(Cube::new(Vector3::new(-1.5, 1.6, 1.9), 0.4, emerald));  // Pared izquierda frente
    objects.push(Cube::new(Vector3::new(2.1, 1.8, 3.1), 0.6, emerald));   // Pared derecha fondo
    objects.push(Cube::new(Vector3::new(-0.2, -0.2, 3.1), 0.3, emerald)); // Centro fondo bajo
    objects.push(Cube::new(Vector3::new(1.8, -0.1, 2.8), 0.5, emerald));  // Derecha fondo bajo
    objects.push(Cube::new(Vector3::new(-1.6, -0.5, 2.1), 0.4, emerald)); // Izquierda frente bajo
    objects.push(Cube::new(Vector3::new(1.3, 0.9, 1.9), 0.3, emerald));   // Derecha frente medio
    objects.push(Cube::new(Vector3::new(-1.4, 3.1, 2.4), 0.5, emerald));  // Alto izquierda
    objects.push(Cube::new(Vector3::new(2.1, 3.4, 2.2), 0.4, emerald));   // Alto derecha

    //RUBÍES - ESTRICTAMENTE dentro, concentrados en niveles bajos
    objects.push(Cube::new(Vector3::new(-1.8, -0.8, 2.8), 0.4, rubi));    // Pared izquierda bajo
    objects.push(Cube::new(Vector3::new(2.7, -0.9, 2.5), 0.5, rubi));     // Pared derecha bajo
    objects.push(Cube::new(Vector3::new(-1.0, -0.7, 2.0), 0.3, rubi));    // Centro izq frente bajo
    objects.push(Cube::new(Vector3::new(0.9, -0.6, 3.0), 0.6, rubi));     // Centro der fondo bajo
    objects.push(Cube::new(Vector3::new(1.8, -0.8, 1.9), 0.4, rubi));     // Derecha frente bajo
    objects.push(Cube::new(Vector3::new(-1.9, -0.3, 2.4), 0.5, rubi));    // Pared izquierda medio-bajo
    objects.push(Cube::new(Vector3::new(2.5, -0.5, 3.1), 0.3, rubi));     // Pared derecha medio-bajo
    objects.push(Cube::new(Vector3::new(-0.4, -0.4, 2.9), 0.4, rubi));    // Centro fondo bajo
    objects.push(Cube::new(Vector3::new(-1.7, 0.2, 2.8), 0.6, rubi));     // Pared izquierda medio
    objects.push(Cube::new(Vector3::new(2.4, 0.1, 2.1), 0.5, rubi));      // Pared derecha medio
    objects.push(Cube::new(Vector3::new(1.5, -0.1, 2.0), 0.3, rubi));     // Derecha frente medio
    objects.push(Cube::new(Vector3::new(-1.2, -0.8, 3.1), 0.4, rubi));    // Izquierda fondo bajo
    objects.push(Cube::new(Vector3::new(-1.6, 1.1, 2.6), 0.5, rubi));     // Pared izquierda medio-alto
    objects.push(Cube::new(Vector3::new(2.6, 1.0, 2.3), 0.3, rubi));      // Pared derecha medio-alto
    objects.push(Cube::new(Vector3::new(0.6, 0.5, 3.0), 0.4, rubi));      // Centro fondo medio
    objects.push(Cube::new(Vector3::new(-1.4, 2.1, 2.9), 0.6, rubi));     // Pared izquierda alto
    objects.push(Cube::new(Vector3::new(2.2, 2.2, 2.7), 0.5, rubi));      // Pared derecha alto
    objects.push(Cube::new(Vector3::new(-0.8, 3.3, 3.0), 0.3, rubi));     // Alto izquierda fondo
    objects.push(Cube::new(Vector3::new(1.5, 3.5, 2.8), 0.4, rubi));      // Alto derecha fondo
    objects.push(Cube::new(Vector3::new(0.3, 2.4, 2.1), 0.5, rubi));      // Centro frente alto

    //AGUAMARINAS - ESTRICTAMENTE dentro, distribuidas uniformemente
    objects.push(Cube::new(Vector3::new(-1.9, -0.7, 2.3), 0.5, aquamarine)); // Pared izquierda bajo
    objects.push(Cube::new(Vector3::new(2.8, -0.5, 2.9), 0.4, aquamarine));  // Pared derecha bajo
    objects.push(Cube::new(Vector3::new(-0.6, -0.8, 2.6), 0.6, aquamarine)); // Centro bajo
    objects.push(Cube::new(Vector3::new(1.7, -0.6, 2.2), 0.3, aquamarine));  // Derecha bajo
    objects.push(Cube::new(Vector3::new(-1.3, -0.9, 3.0), 0.5, aquamarine)); // Izquierda fondo bajo
    objects.push(Cube::new(Vector3::new(-1.8, 0.3, 2.7), 0.4, aquamarine));  // Pared izquierda medio
    objects.push(Cube::new(Vector3::new(2.7, 0.7, 2.4), 0.3, aquamarine));   // Pared derecha medio
    objects.push(Cube::new(Vector3::new(1.1, 0.2, 3.1), 0.6, aquamarine));   // Derecha fondo medio
    objects.push(Cube::new(Vector3::new(-1.1, 0.6, 2.0), 0.5, aquamarine));  // Izquierda frente medio
    objects.push(Cube::new(Vector3::new(-1.7, 1.4, 3.0), 0.4, aquamarine));  // Pared izquierda alto
    objects.push(Cube::new(Vector3::new(2.5, 1.7, 1.9), 0.3, aquamarine));   // Pared derecha alto frente
    objects.push(Cube::new(Vector3::new(-0.1, 1.8, 3.1), 0.5, aquamarine));  // Centro fondo alto
    objects.push(Cube::new(Vector3::new(1.9, 1.3, 2.9), 0.4, aquamarine));   // Derecha fondo alto
    objects.push(Cube::new(Vector3::new(-1.5, 2.6, 2.0), 0.6, aquamarine));  // Pared izquierda muy alto
    objects.push(Cube::new(Vector3::new(2.3, 2.4, 2.8), 0.3, aquamarine));   // Pared derecha muy alto
    objects.push(Cube::new(Vector3::new(-1.0, 3.7, 2.6), 0.5, aquamarine));  // Cerca techo izquierda
    objects.push(Cube::new(Vector3::new(1.4, 3.8, 3.0), 0.4, aquamarine));   // Cerca techo derecha
    objects.push(Cube::new(Vector3::new(0.7, 3.9, 2.0), 0.3, aquamarine));   // Cerca techo frente
    objects.push(Cube::new(Vector3::new(-0.5, 3.2, 2.5), 0.6, aquamarine));  // Alto centro
    objects.push(Cube::new(Vector3::new(1.8, 1.1, 2.1), 0.5, aquamarine));   // Derecha frente medio

    objects
}