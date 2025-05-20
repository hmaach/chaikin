use minifb::{ Key, KeyRepeat, MouseButton, MouseMode, Window as MiniWindow, WindowOptions };
use raster::{ Color, Image };

use crate::geometrical_shapes as gs;
use gs::{ Displayable };

pub struct Window {
    pub window: MiniWindow,
    pub image: Image,
    width: usize,
    height: usize,
}

pub enum Mode {
    Drawing,
    Animating,
}

impl Window {
    pub fn new() -> Self {
        const WIDTH: usize = 840;
        const HEIGHT: usize = 600;

        let image = Image::blank(WIDTH as i32, HEIGHT as i32);

        let window = MiniWindow::new("CHAIKIN", WIDTH, HEIGHT, WindowOptions {
            borderless: false,
            resize: false,
            ..WindowOptions::default()
        }).expect("Unable to open window");

        // window.set_position(1000, 100);

        Self {
            window,
            image,
            width: WIDTH,
            height: HEIGHT,
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn get_mouse_down(&self) -> bool {
        self.window.get_mouse_down(MouseButton::Left)
    }

    pub fn is_enter_pressed(&self) -> bool {
        self.window.is_key_pressed(Key::Enter, KeyRepeat::No)
    }

    pub fn get_mouse_pos(&self, mode: MouseMode) -> Option<(f32, f32)> {
        self.window.get_mouse_pos(mode)
    }

    pub fn update(&mut self) -> Result<(), minifb::Error> {
        let buffer: Vec<u32> = self.image.bytes
            .chunks(4)
            .map(|px| {
                let r = px[0] as u32;
                let g = px[1] as u32;
                let b = px[2] as u32;
                0xff000000 | (r << 16) | (g << 8) | b
            })
            .collect();

        self.window.update_with_buffer(&buffer, self.width, self.height)
    }
}

// Implement drawing for raster::Image
impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
