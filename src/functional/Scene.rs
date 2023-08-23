// this is the functional version of the Scene

use macroquad::prelude::*;

use crate::functional::Balloon::draw_balloon;
use crate::functional::Balloon::has_escaped;
use crate::functional::Balloon::new_balloon;
use crate::functional::Balloon::update_balloon;
use crate::functional::Balloon::Balloon;
use crate::functional::Balloon::BalloonState;

use crate::functional::Tower::draw_tower;
use crate::functional::Tower::increase_tower_pop_count;
use crate::functional::Tower::new_tower;
use crate::functional::Tower::update_tower;
use crate::functional::Tower::Tower;

use super::Projectile::check_collision;
use super::Projectile::draw_projectile;
use super::Projectile::hit_projectile;
use super::Projectile::is_projectile_alive;
use super::Projectile::is_projectile_hit;
use super::Projectile::new_projectile;
use super::Projectile::update_projectile;

#[derive(Clone)]
pub struct GameState {
    delta_time: f32,
    balloon_sprite: Option<Texture2D>,
    background_sprite: Option<Texture2D>,
    coins: u32,
    lives: i32,
    game_over: bool,
    spawn_timer: f32,
    is_placing_tower: bool,
    preview_tower: Option<Tower>,
    balloons: Vec<Balloon>,
    towers: Vec<Tower>,
}

const INITIAL_STATE: GameState = GameState {
    delta_time: 0.0,
    balloon_sprite: None,
    background_sprite: None,
    coins: 30,
    lives: 3,
    game_over: false,
    spawn_timer: 0.0,
    is_placing_tower: false,
    preview_tower: None,
    balloons: Vec::new(),
    towers: Vec::new(),
};

const TOWER_COST: u32 = 15;

pub fn new_scene() -> GameState {
    INITIAL_STATE
}

pub async fn init_scene(state: GameState) -> GameState {
    let mut next_state = state.clone();

    next_state.background_sprite = Some(
        load_texture("resources/sprites/background2.png")
            .await
            .unwrap(),
    );
    next_state.balloon_sprite = Some(load_texture("resources/sprites/balloon.png").await.unwrap());

    if next_state.background_sprite.is_none() || next_state.balloon_sprite.is_none() {
        panic!("Failed to load sprites");
    }

    next_state
        .background_sprite
        .as_mut()
        .unwrap()
        .set_filter(FilterMode::Nearest);

    next_state
        .balloon_sprite
        .as_mut()
        .unwrap()
        .set_filter(FilterMode::Nearest);

    return next_state;
}

// fn run_loop(
//     state: GameState,
//     next_fn: Option<fn(GameState) -> GameState>,
//     produceFn: fn(GameState) -> GameState,
// ) {
//     let mut next_state = produceFn(state.clone());

//     return match next_fn {
//         Some(next) => next(next_state),
//         None => next_state,
//     };
// }

fn reset(state: GameState, next_fn: impl Fn(GameState) -> GameState) -> GameState {
    let mut next_state = state.clone();

    next_state.coins = 30;
    next_state.lives = 3;
    next_state.game_over = false;
    next_state.spawn_timer = 0.0;
    next_state.is_placing_tower = false;

    next_state.balloons.clear();
    next_state.towers.clear();

    return next_fn(next_state);
}

fn draw_game_over(state: GameState, next_fn: impl Fn(GameState) -> GameState) -> GameState {
    clear_background(LIGHTGRAY);

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
        return reset(state, next_fn);
    }

    return next_fn(state);
}

fn draw_background(state: GameState, next_fn: impl Fn(GameState) -> GameState) -> GameState {
    clear_background(LIGHTGRAY);

    if state.background_sprite.is_none() {
        panic!("Background sprite is not loaded");
    }

    let sprite = state.background_sprite.as_ref().unwrap();

    let scale_factor = screen_width() / sprite.width();
    let adjusted_width = sprite.width() * scale_factor;
    let adjusted_height = sprite.height() * scale_factor;

    let x = (screen_width() - adjusted_width) / 2.0;
    let y = (screen_height() - adjusted_height) / 2.0;

    draw_texture_ex(
        &sprite,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(adjusted_width, adjusted_height)),
            ..Default::default()
        },
    );

    return next_fn(state);
}

fn draw_statistics(state: GameState, next_fn: impl Fn(GameState) -> GameState) -> GameState {
    draw_text(
        format!("COINS: {}", state.coins).as_str(),
        10.,
        32.,
        32.,
        WHITE,
    );

    draw_text(
        format!("LIVES: {}", state.lives).as_str(),
        10.,
        64.,
        32.,
        WHITE,
    );

    return next_fn(state);
}

fn spawn_balloon(state: GameState, next_fn: impl Fn(GameState) -> GameState) -> GameState {
    let mut next_state = state.clone();

    next_state.balloons.push(new_balloon());

    return next_fn(next_state);
}

fn handle_spawn_timer(state: GameState, next_fn: impl Fn(GameState) -> GameState) -> GameState {
    let mut next_state = state.clone();

    next_state.spawn_timer += state.delta_time;

    if next_state.spawn_timer > 1. {
        next_state.spawn_timer -= 1.;
        return spawn_balloon(next_state, next_fn);
    }

    return next_fn(next_state);
}

fn update_balloons(state: GameState, next_fn: impl Fn(GameState) -> GameState) -> GameState {
    let mut next_state = state.clone();

    next_state.balloons = next_state
        .balloons
        .iter()
        .map(|balloon| update_balloon(balloon.clone(), state.delta_time))
        .collect();

    return next_fn(next_state);
}

fn draw_balloons(state: GameState, next_fn: impl Fn(GameState) -> GameState) -> GameState {
    let next_state = state.clone();

    for balloon in next_state.balloons.iter() {
        draw_balloon(
            balloon.clone(),
            state.balloon_sprite.as_ref().unwrap().clone(),
        );
    }

    return next_fn(next_state);
}

fn clear_balloons(state: GameState, next_fn: impl Fn(GameState) -> GameState) -> GameState {
    let mut next_state = state.clone();

    next_state.balloons = next_state
        .balloons
        .iter()
        .map(|balloon| {
            if has_escaped(balloon.clone()) {
                let mut next_balloon = balloon.clone();

                next_balloon.state = BalloonState::Escaped;

                return next_balloon;
            }

            return balloon.clone();
        })
        .collect();

    let escaped_balloons = next_state
        .balloons
        .iter()
        .filter(|balloon| balloon.state == BalloonState::Escaped)
        .count();

    next_state.lives -= escaped_balloons as i32;

    if next_state.lives <= 0 {
        next_state.game_over = true;
    }

    next_state
        .balloons
        .retain(|balloon| balloon.state == BalloonState::Alive);

    return next_fn(next_state);
}

fn handle_tower_placement(state: GameState, next_fn: impl Fn(GameState) -> GameState) -> GameState {
    let mut next_state = state.clone();

    if is_key_pressed(KeyCode::Escape) {
        next_state.is_placing_tower = false;
        next_state.preview_tower = None;
    }

    if is_key_pressed(KeyCode::T) {
        let mouse_position = mouse_position();

        next_state.is_placing_tower = true;
        next_state.preview_tower = Some(new_tower(Vec2::new(mouse_position.0, mouse_position.1)));
    }

    if next_state.is_placing_tower {
        let mut new_preview_tower = next_state.preview_tower.unwrap().clone();

        if is_key_down(KeyCode::R) {
            let new_tower_angle = new_preview_tower.angle + 5. * state.delta_time;
            new_preview_tower.angle = new_tower_angle;
        } else if is_key_down(KeyCode::E) {
            let new_tower_angle = new_preview_tower.angle - 5. * state.delta_time;
            new_preview_tower.angle = new_tower_angle;
        }

        new_preview_tower.position = Vec2::new(mouse_position().0, mouse_position().1);

        draw_tower(new_preview_tower.clone(), next_state.coins < TOWER_COST);

        if is_mouse_button_down(MouseButton::Left) && next_state.coins >= TOWER_COST {
            next_state.towers.push(new_preview_tower.clone());

            next_state.is_placing_tower = false;
            next_state.preview_tower = None;
            next_state.coins -= TOWER_COST;
        }

        next_state.preview_tower = Some(new_preview_tower);
    }

    return next_fn(next_state);
}

fn update_towers(state: GameState, next_fn: impl Fn(GameState) -> GameState) -> GameState {
    let mut next_state = state.clone();

    next_state.towers = next_state
        .towers
        .iter_mut()
        .map(|tower| {
            let mut new_tower = update_tower(tower.clone(), state.delta_time);

            new_tower.projectiles = new_tower
                .projectiles
                .iter_mut()
                .map(|projectile| update_projectile(projectile.clone(), state.delta_time))
                .collect();

            return new_tower.clone();
        })
        .collect();

    next_state.towers.iter().for_each(|tower| {
        draw_tower(tower.clone(), false);

        tower
            .projectiles
            .iter()
            .for_each(|projectile| draw_projectile(projectile.clone()));
    });

    return next_fn(next_state);
}

fn handle_popping(state: GameState, next_fn: impl Fn(GameState) -> GameState) -> GameState {
    let mut next_state = state.clone();

    next_state.towers = next_state
        .towers
        .iter()
        .map(|tower| {
            let mut new_tower = tower.clone();

            new_tower.projectiles = new_tower
                .projectiles
                .iter()
                .map(|projectile| {
                    let mut new_projectile = projectile.clone();

                    next_state.balloons.iter_mut().for_each(|balloon| {
                        if check_collision(new_projectile.clone(), balloon.clone()) {
                            new_projectile = hit_projectile(new_projectile.clone());
                            balloon.state = BalloonState::Popped;
                            next_state.coins += 1;
                        }
                    });

                    return new_projectile;
                })
                .collect();

            return new_tower;
        })
        .collect();

    return next_fn(next_state);
}

fn clean_projectiles(state: GameState, next_fn: impl Fn(GameState) -> GameState) -> GameState {
    let mut next_state = state.clone();

    next_state.towers = next_state
        .towers
        .iter_mut()
        .map(|tower| {
            let mut new_tower = tower.clone();

            let projectiles_count = new_tower.projectiles.len();

            new_tower
                .projectiles
                .retain(|projectile| !is_projectile_hit(projectile));

            let projectiles_hit = projectiles_count - new_tower.projectiles.len();

            new_tower
                .projectiles
                .retain(|projectile| is_projectile_alive(projectile));

            if projectiles_hit > 0 {
                new_tower = increase_tower_pop_count(new_tower, projectiles_hit as u32);
            }

            return new_tower;
        })
        .collect();

    return next_fn(next_state);
}

fn update_delta_time(
    state: GameState,
    delta_time: f32,
    next_fn: fn(GameState) -> GameState,
) -> GameState {
    let mut next_state = state.clone();

    next_state.delta_time = delta_time;

    return next_fn(next_state);
}

pub fn update_scene(delta_time: f32, state: GameState) -> GameState {
    match state.game_over {
        true => update_delta_time(state, delta_time, |state| {
            draw_game_over(state, |state| state)
        }),
        false => update_delta_time(state, delta_time, |state| {
            draw_background(state, |state| {
                handle_spawn_timer(state, |state| {
                    handle_tower_placement(state, |state| {
                        update_balloons(state, |state| {
                            update_towers(state, |state| {
                                draw_balloons(state, |state| {
                                    handle_popping(state, |state| {
                                        clean_projectiles(state, |state| {
                                            clear_balloons(state, |state| {
                                                draw_statistics(state, |state| state)
                                            })
                                        })
                                    })
                                })
                            })
                        })
                    })
                })
            })
        }),
    }
}
