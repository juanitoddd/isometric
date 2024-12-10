mod entities;

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use macroquad::math::Affine2;

use entities::shape::Shape;

#[macroquad::main("Isometric")]
async fn main() {
    
    const MOVEMENT_SPEED: f32 = 200.0;    

    rand::srand(miniquad::date::now() as u64);
    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    let mut gameover = false;
        
    loop {
        clear_background(WHITE);
        
        draw_line(0.0, 0.0, 100.0, 200.0, 1.0, BLACK);

        if let Some(key) = get_last_key_pressed() {
            println!("{:?}", key);
        }             

        if !gameover {
            let delta_time = get_frame_time();
            if is_key_down(KeyCode::Right) {
                circle.x += MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Left) {
                circle.x -= MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Down) {
                circle.y += MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Up) {
                circle.y -= MOVEMENT_SPEED * delta_time;
            }

            circle.x = clamp(circle.x, 0.0, screen_width());
            circle.y = clamp(circle.y, 0.0, screen_height());

            // Generate a new square
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                });
            }

            // Move squares
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            // Remove squares below bottom of screen
            squares.retain(|square| square.y < screen_height() + square.size);
        }        

        // Check for collisions
        if squares.iter().any(|square| circle.collides_with(square)) {
            gameover = true;
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
        }

        // Draw everything
        draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);

        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }

        if gameover {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }

        let affine: Affine2 = Affine2::from_angle(90.0);

        next_frame().await
    }
}