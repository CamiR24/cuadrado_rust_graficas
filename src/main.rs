mod framebuffer;
mod cube;
mod ray_intersect;

use framebuffer::Framebuffer;
use cube::Cube;
use ray_intersect::{Intersect, RayIntersect};

use raylib::prelude::*;
use raylib::color::Color;
use nalgebra_glm::{Vec3, normalize, dot};

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
        return Color::new(135, 206, 235, 255);
    }

    // Sistema de iluminación simple pero efectivo
    let light_dir = normalize(&Vec3::new(0.7, 0.7, 0.3)); // Luz diagonal
    let normal = intersect.normal;
    
    // Calcular intensidad de luz (0.4 a 1.0)
    let light_intensity = dot(&normal, &light_dir).max(0.4);
    
    // Color base del material
    let base_color = intersect.material.diffuse;
    
    // Aplicar iluminación
    let r = ((base_color.r as f32) * light_intensity) as u8;
    let g = ((base_color.g as f32) * light_intensity) as u8;
    let b = ((base_color.b as f32) * light_intensity) as u8;
    
    Color::new(r, g, b, 255)
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Cube]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Coordenadas de pantalla normalizadas
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;
            
            // Ajustar por aspect ratio
            let screen_x = screen_x * aspect_ratio;

            // Vista isométrica - posición de cámara que muestra 3 caras
            let ray_origin = Vec3::new(2.0, 1.5, 2.0);  // Arriba, derecha, adelante
            let target = Vec3::new(0.0, 0.0, 0.0);      // Mirando al centro
            
            // Calcular dirección del rayo hacia el pixel
            let forward = normalize(&(target - ray_origin));
            let right = normalize(&Vec3::new(-forward.z, 0.0, forward.x));
            let up = normalize(&Vec3::new(
                -forward.x * forward.y,
                forward.x * forward.x + forward.z * forward.z,
                -forward.z * forward.y
            ));
            
            // FOV pequeño para vista más "ortográfica"
            let fov_scale = 0.6;
            let ray_direction = normalize(&(
                forward + right * (screen_x * fov_scale) + up * (screen_y * fov_scale)
            ));

            let pixel_color = cast_ray(&ray_origin, &ray_direction, objects);
            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Cubo 3D - Vista Isométrica")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    // ✨ CONFIGURACIÓN PERFECTA DEL CUBO ✨
    let objects = vec![
        Cube::new(
            Vec3::new(-0.8, -0.8, -0.8),  // Esquina mínima
            Vec3::new(0.8, 0.8, 0.8),     // Esquina máxima  
            Color::new(200, 80, 80, 255)   // Rojo suave
        ),
    ];

    window.set_target_fps(60);

    println!("🎲 CUBO 3D RENDERIZADO");
    println!("Deberías ver un cubo rojo con:");
    println!("- Cara frontal: Más clara");
    println!("- Cara superior: Tonalidad media"); 
    println!("- Cara lateral: Más oscura");

    while !window.window_should_close() {
        framebuffer.clear();
        render(&mut framebuffer, &objects);
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}