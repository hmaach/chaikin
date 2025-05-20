mod geometrical_shapes;
mod display;

use crate::geometrical_shapes as gs;
use geometrical_shapes::{ Line, Point };
use gs::{ Drawable, Circle };
use display::{Mode, Window};
use minifb::{ MouseMode };

fn main() {
    let mut window = Window::new();

    let mut prev_mouse_down = false;
    let mut points: Vec<Point> = Vec::new();
    let mut mode = Mode::Drawing;

    while window.is_open() {
        let mouse_down = window.get_mouse_down();

        if let Mode::Drawing = mode {
            if mouse_down && !prev_mouse_down {
                if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                    points.push(Point(x as i32, y as i32));
                    Circle::new(x as i32, y as i32, 4).draw(&mut window.image);
                }
            }

            if window.is_enter_pressed() {
                mode = Mode::Animating;
            }
        }

        if let Mode::Animating = mode {
            for i in 0..points.len().saturating_sub(1) {
                let p1 = &points[i];
                let p2 = &points[i + 1];
                Line::new(p1, p2).draw(&mut window.image);
            }
        }

        prev_mouse_down = mouse_down;

        window.update().expect("Failed to update window");
    }
}
