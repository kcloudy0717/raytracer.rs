use std::fs::File;
use std::io::prelude::*;

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
            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;
            image.pixels[((height - j - 1) * width + i) as usize] = Rgb {
                r: ir,
                g: ig,
                b: ib,
            };
        }
    }

    image.write(&String::from("images/image.ppm"));
}
