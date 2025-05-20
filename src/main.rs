mod geometrical_shapes;
mod display;

use display::Window;
use minifb::{ MouseButton, MouseMode };
use crate::geometrical_shapes as gs;
use gs::{ Drawable };

fn main() {
    let mut window = Window::new();

    let mut prev_mouse_down = false;

    while window.is_open() {
        let mouse_down = window.get_mouse_down(MouseButton::Left);

        if mouse_down && !prev_mouse_down {
            if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                gs::Point::new(x as i32, y as i32).draw(&mut window.image);
            }
        }

        prev_mouse_down = mouse_down;

        window.update().expect("Failed to update window");
    }
}
