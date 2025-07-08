pub struct Window {
    pub running: bool,
    minifb_window: minifb::Window,
}

impl Window {
    pub fn new(width: usize, height: usize, title: &str) -> Result<Window, minifb::Error> {
        let minifb_window = minifb::Window::new(title, width, height,
            minifb::WindowOptions::default())?;

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
}

