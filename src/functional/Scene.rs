// this is the functional version of the Scene

use macroquad::prelude::*;

struct GameState {
    coins: u32,
    lives: i32,
    game_over: bool,
}

const INITIAL_STATE: GameState = GameState {
    coins: 30,
    lives: 3,
    game_over: false,
};

pub fn new_scene() -> GameState {
    INITIAL_STATE
}

pub fn init_scene() {
    let background_sprite = load_texture("resources/sprites/background2.png")
        .await
        .unwrap();

    let balloon_sprite = load_texture("resources/sprites/balloon.png").await.unwrap();

    background_sprite.set_filter(FilterMode::Nearest);
    balloon_sprite.set_filter(FilterMode::Nearest);
}

pub fn update_scene(delta_time: f32, state: &mut GameState) {}
