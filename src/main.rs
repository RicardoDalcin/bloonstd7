use macroquad::prelude::*;

use std::collections::LinkedList;

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

    let mut balloons: LinkedList<Balloon> = LinkedList::new();

    let mut spawn_timer = 0.;

    loop {
        if !game_over {
            clear_background(LIGHTGRAY);

            if is_key_down(KeyCode::T) {
                game_over = true;
            }

            if is_key_down(KeyCode::Right) {
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
            }

            if get_time() - last_update > 1. / 60. {
                last_update = get_time();
                score += 1;
            }

            spawn_timer += get_frame_time();

            if (spawn_timer > 1.) {
                spawn_timer -= 1.;
                let new_balloon: Balloon = Balloon {
                    position: Vec2::new(BALLOON_SIZE, screen_height() / 2.),
                    direction: Direction::Right,
                };
                balloons.push_back(new_balloon);
            }

            for balloon in &mut balloons {
                match balloon.direction {
                    Direction::Right => balloon.position.x += BALLOON_SPEED * get_frame_time(),
                    Direction::Left => balloon.position.x -= BALLOON_SPEED * get_frame_time(),
                }

                if balloon.position.x > screen_width() + BALLOON_SIZE {
                    balloon.position.x = -BALLOON_SIZE;
                } else if balloon.position.x < -BALLOON_SIZE {
                    balloon.position.x = screen_width() + BALLOON_SIZE;
                }

                draw_circle(balloon.position.x, screen_height() / 2., BALLOON_SIZE, RED);
            }
            let game_size = screen_width().min(screen_height());

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
                balloons.clear();
                score = 0;
                last_update = get_time();
                game_over = false;
            }
        }
        next_frame().await;
    }
}
