use pixelrealms_color::Color;

pub struct Texture {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Texture {
    pub fn blank(width: usize, height: usize) -> Texture {
        let pixels = vec![Color::BLACK; width*height];
        Texture {
            width, height, pixels
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn as_u32_buffer(&self) -> Vec<u32> {
        let mut buffer = Vec::with_capacity(self.get_width()*self.get_height());
        for pixel in &self.pixels {
            buffer.push(pixel.r as u32 * 256 * 256 + pixel.g as u32 * 256 + pixel.b as u32);
        }
        buffer
    }
}

