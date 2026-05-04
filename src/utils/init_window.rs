use crate::size;
use crate::utils::pixels_buffer::Buffer;
use display_info::DisplayInfo;
use minifb::{Window, WindowOptions};

fn make_window(name: &str, width: usize, height: usize) -> Window {
    let mut window =
        Window::new(name, width, height, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.set_target_fps(60);

    window
}

fn get_screen_size() -> (usize, usize) {
    let display_infos = DisplayInfo::all().unwrap();
    let display = &display_infos[0];

    (display.width as usize, display.height as usize)
}

pub fn init_window(name: &str) -> (Buffer, Window) {
    let (width, height) = get_screen_size();
    let window = make_window(name, width, height);
    let buffer = Buffer::new(size!(width as u32, height as u32));

    (buffer, window)
}
