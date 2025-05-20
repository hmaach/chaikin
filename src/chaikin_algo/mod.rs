use crate::geometrical_shapes::{ Line, Point };
use crate::geometrical_shapes::Drawable;

pub fn apply_chaikin(lines: &mut Vec<Line>, points: &Vec<Point>, ops_count: &mut u8) {
    match ops_count {
        0 | 7 => {
            let color = Line::color();

            for i in 0..points.len().saturating_sub(1) {
                let p1 = &points[i];
                let p2 = &points[i + 1];
                lines.push(Line::new(p1, p2, &color));
            }
            match ops_count {
                7 => {
                    *ops_count = 0;
                }
                _ => {
                    *ops_count += 1;
                }
            }
        }
        _ => {
            // let mut new_lines: Vec<Line> = Vec::new();
            // let color = Line::color();

            // for line in lines.iter() {
            //     let Point(x0, y0, _) = line.0;
            //     let Point(x1, y1, _) = line.1;

            //     let qx = (3 * x0 + x1) / 4;
            //     let qy = (3 * y0 + y1) / 4;
            //     let rx = (x0 + 3 * x1) / 4;
            //     let ry = (y0 + 3 * y1) / 4;

            //     let q = Point(qx, qy, color.clone());
            //     let r = Point(rx, ry, color.clone());

            //     new_lines.push(Line::new(&Point(x0, y0, color.clone()), &q, &color));
            //     new_lines.push(Line::new(&r, &Point(x1, y1, color.clone()), &color));
            // }

            // *lines = new_lines;
            *ops_count += 1;
        }
    }
}
