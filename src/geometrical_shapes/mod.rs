use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color() -> Color {
        let r = rand::rng().random_range(1..255);
        let g = rand::rng().random_range(1..255);
        let b = rand::rng().random_range(1..255);
        Color::rgb(r, g, b)
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// point struct
#[derive(Debug, Clone)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
    pub fn random(width: i32, height: i32) -> Self {
        let x = rand::rng().random_range(0..width);
        let y = rand::rng().random_range(0..height);
        Self(x, y)
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.0, self.1, Self::color());
    }
}

// line struct
#[derive(Debug, Clone)]
pub struct Line(Point, Point, Color);

impl Line {
    pub fn new(p1: &Point, p2: &Point, color: Color) -> Self {
        Self(p1.clone(), p2.clone(), color.clone())
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        // get a random color
        let color: Color = self.2.clone();

        let start_x: i32 = self.0.0;
        let start_y: i32 = self.0.1;

        let end_x: i32 = self.1.0;
        let end_y: i32 = self.1.1;

        let dis_x: i32 = end_x - start_x; // distance between the x of start & end points
        let dis_y: i32 = end_y - start_y; // distance between the y of start & end points

        let steps = i32::max(dis_x.abs(), dis_y.abs());

        let mut new_x: f32 = start_x as f32;
        let mut new_y: f32 = start_y as f32;

        let x_inc = dis_x as f32 / steps as f32;
        let y_inc = dis_y as f32 / steps as f32;

        for _ in 0..=steps {
            image.display(new_x.round() as i32, new_y.round() as i32, color.clone());
            new_x += x_inc;
            new_y += y_inc;
        }
    }
}
