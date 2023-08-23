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
use super::Projectile::update_projectile;

#[derive(Clone)]
pub struct Keys {
    pub tower_placement: bool,
    pub cancel_tower_placement: bool,
    pub place_tower: bool,
    pub rotate_tower_clockwise: bool,
    pub rotate_tower_counter_clockwise: bool,
    pub reset: bool,
}

#[derive(Clone)]
pub struct GameState {
    delta_time: f32,
    keys: Keys,
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
    keys: Keys {
        tower_placement: false,
        cancel_tower_placement: false,
        place_tower: false,
        rotate_tower_clockwise: false,
        rotate_tower_counter_clockwise: false,
        reset: false,
    },
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

fn reset(state: GameState) -> GameState {
    GameState {
        delta_time: 0.0,
        keys: Keys {
            tower_placement: false,
            cancel_tower_placement: false,
            place_tower: false,
            rotate_tower_clockwise: false,
            rotate_tower_counter_clockwise: false,
            reset: false,
        },
        balloon_sprite: state.balloon_sprite,
        background_sprite: state.background_sprite,
        coins: 30,
        lives: 3,
        game_over: false,
        spawn_timer: 0.0,
        is_placing_tower: false,
        preview_tower: None,
        balloons: Vec::new(),
        towers: Vec::new(),
    }
}

fn draw_game_over(state: GameState) -> GameState {
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

    if state.keys.reset {
        return reset(state);
    }

    state
}

fn draw_background(state: GameState) -> GameState {
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

    state
}

fn draw_statistics(state: GameState) -> GameState {
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

    state
}

fn spawn_balloon(state: GameState) -> GameState {
    GameState {
        balloons: state
            .balloons
            .iter()
            .chain(std::iter::once(&new_balloon()))
            .cloned()
            .collect(),
        ..state
    }
}

fn handle_spawn_timer(state: GameState) -> GameState {
    let new_state = GameState {
        spawn_timer: state.spawn_timer + state.delta_time,
        ..state
    };

    if new_state.spawn_timer > 1. {
        return spawn_balloon(GameState {
            spawn_timer: new_state.spawn_timer - 1.,
            ..new_state
        });
    }

    return new_state;
}

fn update_balloons(state: GameState) -> GameState {
    GameState {
        balloons: state
            .balloons
            .iter()
            .map(|balloon| update_balloon(balloon.clone(), state.delta_time))
            .collect(),
        ..state
    }
}

fn draw_balloons(state: GameState) -> GameState {
    for balloon in state.balloons.iter() {
        draw_balloon(
            balloon.clone(),
            state.balloon_sprite.as_ref().unwrap().clone(),
        );
    }

    state
}

fn clear_balloons(state: GameState) -> GameState {
    let mut new_state = GameState {
        balloons: state
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
            .collect(),
        ..state
    };

    let escaped_balloons = new_state
        .balloons
        .iter()
        .filter(|balloon| balloon.state == BalloonState::Escaped)
        .count();

    new_state = GameState {
        lives: new_state.lives - escaped_balloons as i32,
        ..new_state
    };

    if new_state.lives <= 0 {
        new_state = GameState {
            game_over: true,
            ..new_state
        };
    }

    GameState {
        balloons: new_state
            .balloons
            .iter()
            .filter(|balloon| balloon.state == BalloonState::Alive)
            .cloned()
            .collect(),
        ..new_state
    }
}

fn handle_tower_placement(state: GameState) -> GameState {
    let mut next_state = state.clone();

    if state.keys.cancel_tower_placement {
        next_state.is_placing_tower = false;
        next_state.preview_tower = None;
    }

    if state.keys.tower_placement {
        let mouse_position = mouse_position();

        next_state.is_placing_tower = true;
        next_state.preview_tower = Some(new_tower(Vec2::new(mouse_position.0, mouse_position.1)));
    }

    if next_state.is_placing_tower {
        let mut new_preview_tower = next_state.preview_tower.unwrap().clone();

        if state.keys.rotate_tower_clockwise {
            let new_tower_angle = new_preview_tower.angle + 5. * state.delta_time;
            new_preview_tower.angle = new_tower_angle;
        } else if state.keys.rotate_tower_counter_clockwise {
            let new_tower_angle = new_preview_tower.angle - 5. * state.delta_time;
            new_preview_tower.angle = new_tower_angle;
        }

        new_preview_tower.position = Vec2::new(mouse_position().0, mouse_position().1);

        draw_tower(new_preview_tower.clone(), next_state.coins < TOWER_COST);

        if state.keys.place_tower && next_state.coins >= TOWER_COST {
            next_state.towers.push(new_preview_tower.clone());

            next_state.is_placing_tower = false;
            next_state.preview_tower = None;
            next_state.coins -= TOWER_COST;
        }

        next_state.preview_tower = Some(new_preview_tower);
    }

    return next_state;
}

fn update_towers(state: GameState) -> GameState {
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

    return next_state;
}

fn handle_popping(state: GameState) -> GameState {
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

    return next_state;
}

fn clean_projectiles(state: GameState) -> GameState {
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

    return next_state;
}

fn update_stateful(
    state: GameState,
    delta_time: f32,
    keys: Keys,
    next_fn: fn(GameState) -> GameState,
) -> GameState {
    next_fn(GameState {
        delta_time,
        keys,
        ..state
    })
}

pub fn pipe(actions: Vec<fn(GameState) -> GameState>, initial_state: GameState) -> GameState {
    actions
        .into_iter()
        .fold(initial_state, |state, action| action(state))
}

pub fn update_scene(delta_time: f32, keys: Keys, state: GameState) -> GameState {
    match state.game_over {
        true => update_stateful(state, delta_time, keys, |state| draw_game_over(state)),
        false => update_stateful(state, delta_time, keys, |state| {
            pipe(
                vec![
                    draw_background,
                    handle_spawn_timer,
                    handle_tower_placement,
                    update_balloons,
                    update_towers,
                    draw_balloons,
                    handle_popping,
                    clean_projectiles,
                    clear_balloons,
                    draw_statistics,
                ],
                state,
            )
        }),
    }
}
