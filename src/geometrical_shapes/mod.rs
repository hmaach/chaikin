use rand::Rng;
use raster::{ Color, Image };

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
pub struct Point(pub i32, pub i32, pub Color);

impl Point {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        Self(x, y, color)
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        let cx = self.0;
        let cy = self.1;
        let radius = 2;

        for x in cx - radius..=cx + radius {
            for y in cy - radius..=cy + radius {
                let dx = x - cx;
                let dy = y - cy;
                if dx * dx + dy * dy <= radius * radius {
                    image.display(x, y, self.2.clone());
                }
            }
        }
    }
}

// line struct
#[derive(Debug, Clone)]
pub struct Line(pub Point, pub Point, pub Color);

impl Line {
    pub fn new(p1: &Point, p2: &Point, color: &Color) -> Self {
        Self(p1.clone(), p2.clone(), color.clone())
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        // get a random color
        let color: Color = self.2.clone();

        let start_x: i32 = self.0.0;
        let start_y: i32 = self.0.1;
        Point::new(start_x, start_y, color.clone()).draw(image);

        let end_x: i32 = self.1.0;
        let end_y: i32 = self.1.1;
        Point::new(end_x, end_y, color.clone()).draw(image);

        let dis_x: i32 = end_x - start_x; // distance between the x of start & end points
        let dis_y: i32 = end_y - start_y; // distance between the y of start & end points

        let steps = i32::max(dis_x.abs(), dis_y.abs());

        let mut new_x: f32 = start_x as f32;
        let mut new_y: f32 = start_y as f32;

        let x_inc = (dis_x as f32) / (steps as f32);
        let y_inc = (dis_y as f32) / (steps as f32);

        for _ in 0..=steps {
            image.display(new_x.round() as i32, new_y.round() as i32, color.clone());
            new_x += x_inc;
            new_y += y_inc;
        }
    }
}

// circle struct
#[derive(Debug, Clone)]
pub struct Circle(Point, i32);

impl Circle {
    pub fn new(x: i32, y: i32, r: i32, color: Color) -> Self {
        Self(Point(x - r / 2, y - r / 2, color), r)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        // get a random color

        let center_x: i32 = self.0.0;
        let center_y: i32 = self.0.1;
        let raduis: i32 = self.1;

        let mut x: i32 = 0;
        let mut y: i32 = -raduis;

        while x < -y {
            let y_mid = (y as f64) + 0.5;
            if (x.pow(2) as f64) + y_mid.powf(2.0) > ((raduis * raduis) as f64) {
                y += 1;
            }

            image.display(center_x + x, center_y + y, self.0.2.clone());
            image.display(center_x - x, center_y + y, self.0.2.clone());
            image.display(center_x + x, center_y - y, self.0.2.clone());
            image.display(center_x - x, center_y - y, self.0.2.clone());
            image.display(center_x + y, center_y + x, self.0.2.clone());
            image.display(center_x - y, center_y + x, self.0.2.clone());
            image.display(center_x + y, center_y - x, self.0.2.clone());
            image.display(center_x - y, center_y - x, self.0.2.clone());

            x += 1;
        }
    }
}
