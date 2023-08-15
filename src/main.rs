use macroquad::prelude::*;

use std::collections::LinkedList;

const SQUARES: i16 = 16;

type Point = (i16, i16);

enum Direction {
    Right,
    Left,
}

struct Balloon {
    position: Vec2,
    direction: Direction,
}

#[macroquad::main("Snake")]
async fn main() {
    let mut score = 0;
    const BALLOON_SIZE: f32 = 50.;
    const BALLOON_SPEED: f32 = 150.;
    let mut last_update = get_time();
    let mut game_over = false;

    let mut balloon = Balloon {
        position: Vec2::new(-screen_width() / 2., 0.),
        direction: Direction::Right,
    };

    loop {
        if !game_over {
            if is_key_down(KeyCode::T) {
                game_over = true;
            }

            if is_key_down(KeyCode::Right) {
                balloon.position.x += BALLOON_SPEED * get_frame_time();
            } else if is_key_down(KeyCode::Left) {
                balloon.position.x -= BALLOON_SPEED * get_frame_time();
            }
        }
        if !game_over {
            clear_background(LIGHTGRAY);

            let game_size = screen_width().min(screen_height());

            draw_circle(balloon.position.x, balloon.position.y, BALLOON_SIZE, RED);
            draw_text(format!("SCORE: {score}").as_str(), 10., 20., 20., DARKGRAY);
        } else {
            clear_background(WHITE);
            let text = "Game Over. Press [enter] to play again.";
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2.,
                font_size,
                DARKGRAY,
            );

            if is_key_down(KeyCode::Enter) {
                balloon = Balloon {
                    position: Vec2::new(-screen_width() / 2., 0.),
                    direction: Direction::Right,
                };
                score = 0;
                last_update = get_time();
                game_over = false;
            }
        }
        next_frame().await;
    }
}
