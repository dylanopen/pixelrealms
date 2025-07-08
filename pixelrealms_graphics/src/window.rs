use crate::texture::Texture;

pub struct Window {
    pub running: bool,
    minifb_window: minifb::Window,
}

impl Window {
    pub fn new(width: usize, height: usize, title: &str) -> Result<Window, minifb::Error> {
        let mut window_options = minifb::WindowOptions::default();
        window_options.resize = false;
        let minifb_window = minifb::Window::new(title, width, height, window_options)?;

        Ok(Window {
            minifb_window,
            running: true,
        })
    }

    pub fn is_running(&self) -> bool {
        self.running && self.minifb_window.is_open()
    }

    pub fn set_target_fps(&mut self, target_fps: usize) {
        self.minifb_window.set_target_fps(target_fps);
    }

    pub fn draw(&mut self, texture: &Texture) {
        self.minifb_window.update_with_buffer(&texture.as_u32_buffer(), texture.get_width(), texture.get_height())
            .expect("Failed to draw framebuffer texture to screen");
    }
}

