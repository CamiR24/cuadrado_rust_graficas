mod framebuffer;
mod cube;
mod material;

use framebuffer::Framebuffer;
use cube::Cube;
use crate::Material;

use raylib::prelude::*;
use raylib::color::Color;
use std::thread;
use std::time::{Duration};
use std::f32::consts::PI;
use nalgebra_glm::Vec3;

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Cube]) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting &&
        tmp.distance < zbuffer {
            zbuffer = intersect.distance;
            intersect = tmp;
        }
    }
    
    if !intersect.is_intersecting {
        return Color::new(4, 12, 36);
    }

    let diffuse = intersect.material.diffuse;

    diffuse
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Object]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Adjust for aspect ratio
            let screen_x = screen_x * aspect_ratio;

            // Calculate the direction of the ray for this pixel
            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

            // Cast the ray and get the pixel color
            let pixel_color = cast_ray(&Vec3::new(0.0, 0.0, 0.0), &ray_direction, objects);

            // Draw the pixel on screen with the returned color
            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
}

fn main() {
  let window_width = 1300;
  let window_height = 900;
  let block_size = 100;

  // Solo inicializar UNA ventana
  let (mut window, raylib_thread) = raylib::init()
    .size(window_width, window_height)
    .title("Cube")
    .log_level(TraceLogLevel::LOG_WARNING)
    .build();

  let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
  framebuffer.set_background_color(Color::new(153, 102, 204, 255));

  let objects = [
    Cube {
        min: Vec3::new(1.0, 1.0, 0.0),
        max: Vec3::new(1.0, 2.0, 0.0),
        color: Color::new(255, 255, 255, 255),
    }
  ];

  while !window.window_should_close() {

    thread::sleep(Duration::from_millis(16));
  }
}