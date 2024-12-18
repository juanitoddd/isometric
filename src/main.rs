mod entities;

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use macroquad::math::Affine2;

use entities::shape::Shape;

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

#[macroquad::main("Isometric")]
async fn main() {
    
    const MOVEMENT_SPEED: f32 = 200.0;    

    rand::srand(miniquad::date::now() as u64);
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };
    
    let mut game_state = GameState::MainMenu;
        
    loop {
        clear_background(DARKPURPLE);                

        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::Space) {                    
                    game_state = GameState::Playing;
                }
                let text = "Press space";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            }
            GameState::Playing => {
                let delta_time = get_frame_time();

                draw_line(0.0, 0.0, 100.0, 200.0, 1.0, BLACK);
                
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }                                                
                
                if is_key_pressed(KeyCode::Space) {                    
                    game_state = GameState::GameOver;
                }                                
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
                let text = "Paused";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
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
        }                    

        let affine: Affine2 = Affine2::from_angle(90.0);

        next_frame().await
    }
}