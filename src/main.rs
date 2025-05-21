use macroquad::prelude::*;

#[macroquad::main("Curve Maker")]
async fn main() {
    let mut points = vec![];
    let mut dragging = None;
    let mut anim_steps = vec![];
    let mut step = 1;
    let mut timer = 0.0;
    let mut animating = false;

    loop {
        let dt = get_frame_time();
        let mpos = vec2(mouse_position().0, mouse_position().1);
        if !animating {
            if is_mouse_button_pressed(MouseButton::Left) {
                if let Some(i) = points.iter().position(|p: &Vec2| p.distance(mpos) < 10.0) {
                    dragging = Some(i);
                } else {
                    points.push(mpos);
                }                
            }
            if is_mouse_button_down(MouseButton::Left) {
                if let Some(i) = dragging { points[i] = mpos; }
            } else { dragging = None; }

            if is_key_pressed(KeyCode::Enter) && points.len() > 1 {
                anim_steps = vec![points.clone()];
                let mut cur = points.clone();
                for _ in 0..7 {
                    let mut next = vec![cur[0]];
                    for i in 0..cur.len() - 1 {
                        let a = cur[i];
                        let b = cur[i + 1];
                        next.push(a.lerp(b, 0.25));
                        next.push(a.lerp(b, 0.75));
                    }
                    next.push(*cur.last().unwrap());
                    anim_steps.push(next.clone());
                    cur = next;
                }
                step = 1;
                timer = 0.0;
                animating = true;
            }
            if is_key_pressed(KeyCode::C) { points.clear(); }
        } else {
            timer += dt;
            if timer >= 0.5 {
                step = (step + 1) % anim_steps.len();
                timer = 0.0;
            }
            if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::C) {
                animating = false;
                if is_key_pressed(KeyCode::C) { points.clear(); }
            }
        }

        clear_background(BLACK);
        for p in &points { draw_circle(p.x, p.y, 5.0, WHITE); }

        if !animating {
            for w in points.windows(2) {
                draw_line(w[0].x, w[0].y, w[1].x, w[1].y, 1.5, WHITE);
            }
            let txts = [
                "Left-click to add points", "Drag points to move them",
                "Press Enter to start animation", "Press C to clear points",
                "Press Escape to quit"
            ];
            for (i, t) in txts.iter().enumerate() {
                draw_text(t, 10.0, 20. + i as f32 * 25., 20., WHITE);
            }
            if points.is_empty() {
                let t = "Add points to begin";
                let s = measure_text(t, None, 30, 1.0);
                draw_text(t, screen_width()/2. - s.width/2., screen_height()/2., 30., GRAY);
            }
        } else {
            let curve = &anim_steps[step];
            for w in curve.windows(2) {
                draw_line(w[0].x, w[0].y, w[1].x, w[1].y, 2.0, GREEN);
            }
            for p in curve { draw_circle(p.x, p.y, 3.0, DARKGREEN); }
            draw_text(&format!("Step: {}/{} (Animation Looping)", step, anim_steps.len()-1), 10., 20., 20., WHITE);
            draw_text("Press Escape to go back or C to clear and go back", 10., 45., 20., WHITE);
        }
        if is_key_pressed(KeyCode::Escape) && !animating { break; }
        next_frame().await;
    }
}