pub struct Window {
    minifb_window: minifb::Window,
}

impl Window {
    pub fn new(width: usize, height: usize, title: &str) -> Result<Window, minifb::Error> {
        let minifb_window = minifb::Window::new(title, width, height,
            minifb::WindowOptions::default())?;

        Ok(Window {
            minifb_window
        })
    }
}

