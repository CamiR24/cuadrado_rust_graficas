// src/main.rs
mod framebuffer;
mod cube;
mod ray_intersect;

use framebuffer::Framebuffer;
use cube::Cube;
use ray_intersect::{Intersect, RayIntersect};

use raylib::prelude::*;
use raylib::color::Color;
use nalgebra_glm::{Vec3, normalize};

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Cube]) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            intersect = tmp;
        }
    }
    
    if !intersect.is_intersecting {
        return Color::new(4, 12, 36, 255); // Color de fondo
    }

    intersect.material.diffuse
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Cube]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Mapear coordenadas del pixel al espacio de pantalla [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Ajustar por aspect ratio
            let screen_x = screen_x * aspect_ratio;

            // Calcular la dirección del rayo para este pixel
            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

            // Lanzar el rayo y obtener el color del pixel
            let pixel_color = cast_ray(&Vec3::new(0.0, 0.0, 0.0), &ray_direction, objects);

            // Dibujar el pixel en pantalla con el color obtenido
            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    // Inicializar ventana
    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("3D Cube Raytracer")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Color::new(4, 12, 36, 255));

    // Crear un cubo centrado y visible
    let objects = vec![
        Cube::new(
            Vec3::new(-0.5, -0.5, -3.0),  // min
            Vec3::new(0.5, 0.5, -2.0),    // max
            Color::new(255, 100, 100, 255) // color rojo
        ),
    ];

    window.set_target_fps(60);

    while !window.window_should_close() {
        // Limpiar framebuffer
        framebuffer.clear();
        
        // Renderizar la escena
        render(&mut framebuffer, &objects);
        
        // Mostrar en pantalla
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}