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

    pub fn get(&self, x: usize, y: usize) -> Option<&Color> {
        self.pixels.get(y * self.width + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        self.pixels.get_mut(y * self.width + x)
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) -> Option<()> {
        *self.get_mut(x, y)? = color;
        Some(())
    }
}

