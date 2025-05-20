mod geometrical_shapes;
mod chaikin_algo;
mod display;

use geometrical_shapes::{ Line, Point };
use crate::geometrical_shapes as gs;
use std::time::{ Duration, Instant };
use chaikin_algo::apply_chaikin;
use display::{ Mode, Window };
use gs::{ Drawable, Circle };
use minifb::{ MouseMode };
use raster::Color;

fn main() {
    let mut window = Window::new();

    let mut prev_mouse_down = false;
    let mut points: Vec<Point> = Vec::new();
    let mut lines: Vec<Line> = Vec::new();
    let mut ops_count: u8 = 0;
    let mut mode = Mode::Drawing;
    let mut last_chaikin_time = Instant::now();
    let chaikin_cooldown = Duration::from_secs_f32(0.5); // 0.5 seconds cooldown
    let point_color = Color::rgb(200, 200, 200);

    while window.is_open() {
        let mouse_down = window.get_mouse_down();

        if let Mode::Drawing = mode {
            if mouse_down && !prev_mouse_down {
                if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
                    points.push(Point(x as i32, y as i32, point_color.clone()));
                    Circle::new(x as i32, y as i32, 3, point_color.clone()).draw(&mut window.image);
                }
            }

            if window.is_enter_pressed() {


                mode = Mode::Animating;
            }
        }

        if let Mode::Animating = mode {
            if last_chaikin_time.elapsed() >= chaikin_cooldown {
                apply_chaikin(&mut lines, &points, &mut ops_count);
                last_chaikin_time = Instant::now();
            }

            let color = Line::color();

            for line in &mut lines {
                line.2 = color.clone();
                line.draw(&mut window.image);
            }
        }
        prev_mouse_down = mouse_down;

        window.update().expect("Failed to update window");
    }
}
