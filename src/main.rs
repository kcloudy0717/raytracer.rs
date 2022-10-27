use std::fs::File;
use std::io::prelude::*;

pub mod vec3;
use vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Rgb>,
}

impl Image {
    fn new(width: u32, height: u32) -> Image {
        Image {
            width: width,
            height: height,
            pixels: vec![Rgb { r: 0, g: 0, b: 0 }; (width * height) as usize],
        }
    }

    fn write(self, path: &String) {
        let mut file = File::create(path).expect("Failed to create file");
        file.write_all(b"P3\n").expect("Failed to write to file");
        file.write_all(format!("{} {}\n", self.width, self.height).as_bytes())
            .expect("Failed to write to file");
        file.write_all(b"255\n").expect("Failed to write to file");
        for pixel in self.pixels {
            file.write_all(format!("{} {} {}\n", pixel.r, pixel.g, pixel.b).as_bytes())
                .expect("Failed to write pixel");
        }
    }

    fn write_color(&mut self, x: u32, y: u32, color: &Vec3) {
        let ir = (255.99 * color.x) as u8;
        let ig = (255.99 * color.y) as u8;
        let ib = (255.99 * color.z) as u8;

        self.pixels[((self.height - y - 1) * self.width + x) as usize] = Rgb {
            r: ir,
            g: ig,
            b: ib,
        };
    }
}

fn main() {
    let width = 256;
    let height = 256;
    let mut image: Image = Image::new(width, height);

    for j in 0..height {
        for i in 0..width {
            let r = i as f32 / (width - 1) as f32;
            let g = j as f32 / (height - 1) as f32;
            let b = 0.25;
            image.write_color(i, j, &Vec3::new(r, g, b));
        }
    }

    image.write(&String::from("images/image.ppm"));
}
