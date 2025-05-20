use macroquad::prelude::*;

const POINT_RADIUS: f32 = 5.0;
const BACKGROUND_COLOR: Color = WHITE;
const POINT_COLOR: Color = RED;
const LINE_COLOR: Color = BLUE;
const CURVE_COLOR: Color = GREEN;
const CURVE_POINT_COLOR: Color = DARKGREEN;
const INSTRUCTION_TEXT_COLOR: Color = BLACK;
const MESSAGE_TEXT_COLOR: Color = GRAY;
const ANIMATION_SPEED: f32 = 0.5; // seconds per step

#[derive(Clone, Debug)]
struct Point {
    position: Vec2,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
        }
    }
    
    fn distance_to(&self, point: Vec2) -> f32 {
        self.position.distance(point)
    }
    
    fn draw(&self, color: Color, radius: f32) {
        draw_circle(self.position.x, self.position.y, radius, color);
    }
}

enum AppState {
    Drawing,
    Animating,
}

struct App {
    points: Vec<Point>,
    state: AppState,
    is_dragging: Option<usize>,
    animation_timer: f32,
    current_step: usize,
    animation_steps: Vec<Vec<Point>>,
}

impl App {
    // fn new() -> Self {
    //     Self {
    //         points: Vec::new(),
    //         state: AppState::Drawing,
    //         is_dragging: None,
    //         animation_timer: 0.0,
    //         current_step: 0,
    //         animation_steps: Vec::new(),
    //     }
    // }
    
    fn update(&mut self, dt: f32) {
        match self.state {
            AppState::Drawing => {
                self.handle_drawing_input();
                
                if is_key_pressed(KeyCode::Enter) && self.points.len() > 1 {
                    self.start_animation();
                }
                
                if is_key_pressed(KeyCode::C) {
                    self.points.clear();
                }
            },
            AppState::Animating => {
                self.animation_timer += dt;
                if self.animation_timer >= ANIMATION_SPEED {
                    self.animation_timer = 0.0;
                    self.current_step = (self.current_step + 1) % self.animation_steps.len();
                    if self.current_step == 0 {
                        self.state = AppState::Drawing;
                    }
                }
                if is_key_pressed(KeyCode::Escape) {
                    self.state = AppState::Drawing;
                }
            }
        }
    }
    
    fn handle_drawing_input(&mut self) {
        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
        
        if is_mouse_button_pressed(MouseButton::Left) {
            let mut clicked_on_point = false;
            for (i, point) in self.points.iter().enumerate() {
                if point.distance_to(mouse_pos) < POINT_RADIUS * 2.0 {
                    self.is_dragging = Some(i);
                    clicked_on_point = true;
                    break;
                }
            }
            
            if !clicked_on_point {
                self.points.push(Point::new(mouse_pos.x, mouse_pos.y));
            }
        }
        
        if is_mouse_button_down(MouseButton::Left) {
            if let Some(index) = self.is_dragging {
                self.points[index].position = mouse_pos;
            }
        } else {
            self.is_dragging = None;
        }
    }
    
    fn start_animation(&mut self) {
        if self.points.len() <= 1 {
            return; // Need at least 2 points to animate
        }
        
        self.animation_steps = vec![self.points.clone()];
        
        let mut current_points = self.points.clone();
        for _ in 0..7 {
            current_points = self.chaikin_step(&current_points);
            self.animation_steps.push(current_points.clone());
        }
        
        self.current_step = 0;
        self.animation_timer = 0.0;
        self.state = AppState::Animating;
    }
    
    fn chaikin_step(&self, points: &[Point]) -> Vec<Point> {
        if points.len() <= 2 { return points.to_vec(); // No subdivision for 1 or 2 points
        }
        
        let mut result = Vec::new();
        
        for i in 0..points.len() - 1 {
            let p0 = &points[i];
            let p1 = &points[i + 1];
            
            // Q point: 3/4 of p0, 1/4 of p1
            let q = Point::new(
                0.75 * p0.position.x + 0.25 * p1.position.x,
                0.75 * p0.position.y + 0.25 * p1.position.y,
            );
            
            // R point: 1/4 of p0, 3/4 of p1
            let r = Point::new(
                0.25 * p0.position.x + 0.75 * p1.position.x,
                0.25 * p0.position.y + 0.75 * p1.position.y,
            );
            
            result.push(q);
            result.push(r);
        }
        
        result
    }
    
    fn draw(&self) {
        clear_background(BACKGROUND_COLOR);
        
        match self.state {
            AppState::Drawing => {
                // Draw all points
                for point in &self.points {
                    point.draw(POINT_COLOR, POINT_RADIUS);
                }
                
                // Draw lines between points
                if self.points.len() > 1 {
                    for i in 0..self.points.len() - 1 {
                        draw_line(
                            self.points[i].position.x,
                            self.points[i].position.y,
                            self.points[i + 1].position.x,
                            self.points[i + 1].position.y,
                            1.5,
                            LINE_COLOR
                        );
                    }
                }
                
                // Display instructions
                draw_text("Left-click to add points", 10.0, 20.0, 20.0, INSTRUCTION_TEXT_COLOR);
                draw_text("Drag points to move them", 10.0, 45.0, 20.0, INSTRUCTION_TEXT_COLOR);
                draw_text("Press Enter to start animation", 10.0, 70.0, 20.0, INSTRUCTION_TEXT_COLOR);
                draw_text("Press C to clear points", 10.0, 95.0, 20.0, INSTRUCTION_TEXT_COLOR);
                draw_text("Press Escape to quit", 10.0, 120.0, 20.0, INSTRUCTION_TEXT_COLOR);
                
                if self.points.is_empty() {
                    let text = "Add points to begin";
                    let text_size = measure_text(text, None, 30, 1.0);
                    draw_text(
                        text,
                        screen_width() / 2.0 - text_size.width / 2.0,
                        screen_height() / 2.0,
                        30.0,
                        MESSAGE_TEXT_COLOR
                    );
                }
            },
            AppState::Animating => {
                let current_curve = &self.animation_steps[self.current_step];
                
                // Draw the current step of the curve
                if current_curve.len() > 1 {
                    for i in 0..current_curve.len() - 1 {
                        draw_line(
                            current_curve[i].position.x,
                            current_curve[i].position.y,
                            current_curve[i + 1].position.x,
                            current_curve[i + 1].position.y,
                            2.0,
                            CURVE_COLOR
                        );
                    }
                    
                    // Draw points at curve vertices
                    for point in current_curve {
                        point.draw(CURVE_POINT_COLOR, 3.0);
                    }
                } else if current_curve.len() == 1 {
                    // Draw a single point if that's all we have
                    current_curve[0].draw(CURVE_POINT_COLOR, POINT_RADIUS);
                }
                
                // Display animation information
                draw_text(
                    &format!("Animation Step: {}/{}", self.current_step, self.animation_steps.len() - 1), 
                    10.0, 
                    20.0, 
                    20.0, 
                    INSTRUCTION_TEXT_COLOR
                );
                draw_text(
                    "Press Escape to return to drawing mode", 
                    10.0, 
                    45.0, 
                    20.0, 
                    INSTRUCTION_TEXT_COLOR
                );
            }
        }
    }
}

#[macroquad::main("Chaikin's Algorithm Animation")]
async fn main() {
    let mut app = App::new();
    
    loop {
        let dt = get_frame_time();
        app.update(dt);
        app.draw();
        
        // Quit on Escape key only in drawing mode
        if is_key_pressed(KeyCode::Escape) && matches!(app.state, AppState::Drawing) {
            break;
        }
        
        next_frame().await
    }
}