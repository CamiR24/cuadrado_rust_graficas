//texture.rs

use raylib::prelude::*;
use std::collections::HashMap;
use std::slice;

pub struct TextureManager {
    images: HashMap<char, Image>,       
    textures: HashMap<char, Texture2D>,
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut images = HashMap::new();
        let mut textures = HashMap::new();

        // Map characters to texture file paths
        let texture_files = vec![
            ('S', "assets/rock.png"),
            ('D', "assets/diamond.png"),
            ('E', "assets/emerald.png"),
            ('R', "assets/rubi.png"),
            ('A', "assets/aquamarine.png"),
        ];

        println!("Cargando texturas...");
        
        for (ch, path) in texture_files {
            println!("  Intentando cargar: {} -> {}", ch, path);
            
            // Intentar cargar imagen
            match Image::load_image(path) {
                Ok(image) => {
                    println!("Imagen cargada: {}x{}", image.width, image.height);
                    
                    // Intentar crear textura GPU
                    match rl.load_texture(thread, path) {
                        Ok(texture) => {
                            println!("Textura GPU creada");
                            images.insert(ch, image);
                            textures.insert(ch, texture);
                        },
                        Err(e) => {
                            println!("Error creando textura GPU: {:?}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("Error cargando imagen: {:?}", e);
                    println!("Verifica que el archivo existe");
                }
            }
        }

        TextureManager { images, textures }
    }

    pub fn get_pixel_color(&self, ch: char, tx: u32, ty: u32) -> Color {
        if let Some(image) = self.images.get(&ch) {
            // Normalizar las coordenadas de 128 al tamaño real de la imagen
            let texture_size = 128.0;
            let normalized_x = (tx as f32 / texture_size * image.width as f32) as u32;
            let normalized_y = (ty as f32 / texture_size * image.height as f32) as u32;
            
            let x = normalized_x.min(image.width as u32 - 1) as i32;
            let y = normalized_y.min(image.height as u32 - 1) as i32;
            get_pixel_color(image, x, y)
        } else {
            // ⚠️ ESTE ES EL CÓDIGO QUE ESTÁ EJECUTÁNDOSE AHORA
            println!("⚠️ No hay textura para '{}', usando color sólido de respaldo", ch);
            match ch {
                'A' => Color::new(255, 215, 0, 255),   // Gold (esto es lo que ves como "ladrillo")
                'R' => Color::new(220, 20, 60, 255),   // Crimson
                'V' => Color::new(50, 205, 50, 255),   // Lime Green
                'M' => Color::new(138, 43, 226, 255),  // Blue Violet
                'B' => Color::new(30, 144, 255, 255),  // Dodger Blue
                'T' => Color::new(64, 224, 208, 255),  // Turquoise
                'P' => Color::new(255, 105, 180, 255), // Hot Pink
                'N' => Color::new(255, 140, 0, 255),   // Dark Orange
                'g' => Color::GRAY,
                _ => Color::WHITE,
            }
        }
    }

    pub fn get_texture(&self, ch: char) -> Option<&Texture2D> {
        self.textures.get(&ch)
    }
}

fn get_pixel_color(image: &Image, x: i32, y: i32) -> Color {
    let width = image.width as usize;
    let height = image.height as usize;

    if x < 0 || y < 0 || x as usize >= width || y as usize >= height {
        return Color::WHITE;
    }

    let x = x as usize;
    let y = y as usize;

    let data_len = width * height * 4;

    unsafe {
        let data = slice::from_raw_parts(image.data as *const u8, data_len);

        let idx = (y * width + x) * 4;

        if idx + 3 >= data_len {
            return Color::WHITE;
        }

        Color::new(data[idx], data[idx + 1], data[idx + 2], data[idx + 3])
    }
}