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

struct Tower {
    position: Vec2,
    angle: f32,
    shot_cooldown: f32,
}

struct Projectile {
    position: Vec2,
    direction: Vec2,
    is_alive: bool,
}

const BALLOON_SPRITE_SIZE: f32 = 32.;
const BALLOON_SIZE: f32 = BALLOON_SPRITE_SIZE * 3.;
const BALLOON_COLLIDER_SIZE: f32 = BALLOON_SIZE / 2.;

fn check_collision(projectile: &Projectile, balloon: &Balloon) -> bool {
    let distance_x = projectile.position.x - balloon.position.x;
    let distance_y = projectile.position.y - balloon.position.y;

    return distance_x.abs() < BALLOON_COLLIDER_SIZE && distance_y.abs() < BALLOON_COLLIDER_SIZE;
}

#[macroquad::main("Balloons")]
async fn main() {
    let mut score = 0;
    const BALLOON_SPEED: f32 = 150.;
    let mut last_update = get_time();
    let mut game_over = false;

    let mut balloons: Vec<Balloon> = Vec::new();
    let mut towers: LinkedList<Tower> = LinkedList::new();

    let mut spawn_timer = 0.;

    let mut is_placing_tower = false;
    let mut new_tower_angle: f32 = 0.;

    let mut projectiles: Vec<Projectile> = Vec::new();

    let background_sprite = load_texture("resources/sprites/background.png")
        .await
        .unwrap();

    let balloon_sprite = load_texture("resources/sprites/balloon.png").await.unwrap();

    background_sprite.set_filter(FilterMode::Nearest);
    balloon_sprite.set_filter(FilterMode::Nearest);

    loop {
        if !game_over {
            clear_background(LIGHTGRAY);

            let scale_factor = screen_width() / background_sprite.width();
            let adjusted_width = background_sprite.width() * scale_factor;
            let adjusted_height = background_sprite.height() * scale_factor;

            let x = (screen_width() - adjusted_width) / 2.0;
            let y = (screen_height() - adjusted_height) / 2.0;

            draw_texture_ex(
                &background_sprite,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(adjusted_width, adjusted_height)),
                    ..Default::default()
                },
            );

            if is_key_down(KeyCode::T) {
                is_placing_tower = true;
            }

            if get_time() - last_update > 1. / 60. {
                last_update = get_time();
                score += 1;
            }

            spawn_timer += get_frame_time();

            if spawn_timer > 1. {
                spawn_timer -= 1.;
                let new_balloon: Balloon = Balloon {
                    position: Vec2::new(BALLOON_SIZE, screen_height() / 2.),
                    direction: Direction::Right,
                };
                balloons.push(new_balloon);
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

                draw_texture_ex(
                    &balloon_sprite,
                    balloon.position.x - BALLOON_SIZE / 2.,
                    screen_height() / 2. - (BALLOON_SIZE / 2.),
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(BALLOON_SIZE, BALLOON_SIZE)),
                        ..Default::default()
                    },
                );

                draw_circle_lines(
                    balloon.position.x,
                    screen_height() / 2.,
                    BALLOON_COLLIDER_SIZE,
                    2.,
                    RED,
                );
            }

            const TOWER_SIZE: f32 = 50.;

            if is_placing_tower {
                let mouse_position = mouse_position();
                draw_circle(mouse_position.0, mouse_position.1, TOWER_SIZE, BLUE);
                draw_line(
                    mouse_position.0,
                    mouse_position.1,
                    mouse_position.0 + new_tower_angle.cos() * TOWER_SIZE * 2.,
                    mouse_position.1 + new_tower_angle.sin() * TOWER_SIZE * 2.,
                    2.,
                    BLUE,
                );

                if (is_key_down(KeyCode::R)) {
                    new_tower_angle += 1. * get_frame_time();
                } else if (is_key_down(KeyCode::E)) {
                    new_tower_angle -= 1. * get_frame_time();
                }

                if is_mouse_button_down(MouseButton::Left) {
                    let new_tower: Tower = Tower {
                        position: Vec2::new(mouse_position.0, mouse_position.1),
                        angle: new_tower_angle,
                        shot_cooldown: 0.,
                    };

                    towers.push_back(new_tower);

                    is_placing_tower = false;
                    new_tower_angle = 0.;
                }
            }

            for tower in &mut towers {
                tower.shot_cooldown -= get_frame_time();
                draw_circle(tower.position.x, tower.position.y, TOWER_SIZE, BLUE);
                draw_line(
                    tower.position.x,
                    tower.position.y,
                    tower.position.x + tower.angle.cos() * TOWER_SIZE * 2.,
                    tower.position.y + tower.angle.sin() * TOWER_SIZE * 2.,
                    2.,
                    BLUE,
                );

                if (tower.shot_cooldown <= 0.) {
                    let new_projectile: Projectile = Projectile {
                        position: Vec2::new(tower.position.x, tower.position.y),
                        direction: Vec2::new(tower.angle.cos(), tower.angle.sin()),
                        is_alive: true,
                    };

                    projectiles.push(new_projectile);
                    tower.shot_cooldown = 1.;
                }
            }

            for projectile in &mut projectiles {
                projectile.position += projectile.direction * 500. * get_frame_time();

                let balloons_before = balloons.len();

                balloons = balloons
                    .into_iter()
                    .filter(|balloon| !check_collision(projectile, balloon))
                    .collect();

                if balloons.len() < balloons_before {
                    projectile.is_alive = false;
                }

                draw_circle(projectile.position.x, projectile.position.y, 15., ORANGE);
            }

            projectiles = projectiles
                .into_iter()
                .filter(|projectile| projectile.is_alive)
                .collect();

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
