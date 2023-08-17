use macroquad::prelude::*;

use std::collections::LinkedList;

mod lib;

use lib::GameManager::GameManager;

// fn check_collision(projectile: &Projectile, balloon: &Balloon) -> bool {
//     let distance_x = projectile.position.x - balloon.position.x;
//     let distance_y = projectile.position.y - balloon.position.y;

//     return distance_x.abs() < BALLOON_COLLIDER_SIZE && distance_y.abs() < BALLOON_COLLIDER_SIZE;
// }

#[macroquad::main("Balloons")]
async fn main() {
    let mut score = 0;
    const BALLOON_SPEED: f32 = 150.;
    let mut last_update = get_time();
    let mut game_over = false;

    let mut is_placing_tower = false;
    let mut new_tower_angle: f32 = 0.;

    let mut game_manager = GameManager::new().await;

    loop {
        if !game_over {
            game_manager.update(get_frame_time());

            // if get_time() - last_update > 1. / 60. {
            //     last_update = get_time();
            //     score += 1;
            // }

            // for tower in &mut towers {
            //     tower.shot_cooldown -= get_frame_time();

            //     if (tower.shot_cooldown <= 0.) {
            //         let new_projectile: Projectile = Projectile {
            //             position: Vec2::new(tower.position.x, tower.position.y),
            //             direction: Vec2::new(tower.angle.cos(), tower.angle.sin()),
            //             is_alive: true,
            //         };

            //         projectiles.push(new_projectile);
            //         tower.shot_cooldown = 1.;
            //     }
            // }

            // for projectile in &mut projectiles {
            //     let balloons_before = balloons.len();

            //     balloons = balloons
            //         .into_iter()
            //         .filter(|balloon| !check_collision(projectile, balloon))
            //         .collect();

            //     if balloons.len() < balloons_before {
            //         projectile.is_alive = false;
            //     }

            // }

            // projectiles = projectiles
            //     .into_iter()
            //     .filter(|projectile| projectile.is_alive)
            //     .collect();

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

            // if is_key_down(KeyCode::Enter) {
            //     balloons.clear();
            //     score = 0;
            //     last_update = get_time();
            //     game_over = false;
            // }
        }

        next_frame().await;
    }
}
