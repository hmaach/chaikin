use crate::geometrical_shapes::{ Line, Point };
use crate::geometrical_shapes::Drawable;

pub fn apply_chaikin(lines: &mut Vec<Line>, points: &Vec<Point>, ops_count: &mut u8) {
    let color = Line::color();
    match ops_count {
        7 | 0 => {
            lines.clear();

            for i in 0..points.len().saturating_sub(1) {
                let p1 = &points[i];
                let p2 = &points[i + 1];
                lines.push(Line::new(p1, p2, &color));
            }
            match ops_count {
                7 => *ops_count = 0,
                _ => *ops_count += 1,
            }

           
        }
        _ => {
            let mut i = 0;

            while i < lines.len() - 1 {
                process_lines(i, lines);

                i += 2;
            }

            *ops_count += 1;
        }
    }
}

fn calculate_delta(point_1: &Point, point_2: &Point) -> (i32, i32) {
    let delta_x = point_2.0 - point_1.0;
    let delta_y = point_2.1 - point_1.1;
    (delta_x, delta_y)
}

// Function to scale the second point of the first line and the first point of the second line
fn scale_points(
    point_1_2: &mut Point,
    point_2_1: &mut Point,
    delta_1_x: i32,
    delta_1_y: i32,
    delta_2_x: i32,
    delta_2_y: i32
) -> Line {
    point_1_2.0 = ((point_1_2.0 as f32) - (delta_1_x as f32) * 0.25) as i32;
    point_1_2.1 = ((point_1_2.1 as f32) - (delta_1_y as f32) * 0.25) as i32;

    point_2_1.0 = ((point_2_1.0 as f32) + (delta_2_x as f32) * 0.25) as i32;
    point_2_1.1 = ((point_2_1.1 as f32) + (delta_2_y as f32) * 0.25) as i32;

    let color = Line::color();

    Line::new(&point_1_2.clone(), &point_2_1.clone(), &color)
}

fn process_lines(i: usize, lines: &mut Vec<Line>) {
    let (line_1, line_2) = lines.split_at_mut(i + 1);

    // Borrow the first and second line
    let line_1 = &mut line_1[i]; // First line at index `i`
    let line_2 = &mut line_2[0]; // Second line at index `i + 1` (after the split)

    // Borrow the points
    let point_1_1 = &mut line_1.0;
    let point_1_2 = &mut line_1.1;
    let point_2_1 = &mut line_2.0;
    let point_2_2 = &mut line_2.1;

    // Calculate deltas
    let (delta_1_x, delta_1_y) = calculate_delta(&point_1_1, &point_1_2);
    let (delta_2_x, delta_2_y) = calculate_delta(&point_2_1, &point_2_2);

    // Scale the points
    let new_line = scale_points(point_1_2, point_2_1, delta_1_x, delta_1_y, delta_2_x, delta_2_y);
    lines.insert(i + 1, new_line);
}
