use crate::object_oriented::DrawableObject::DrawableObject;
use macroquad::prelude::*;

#[derive(Copy, Clone)]
enum Direction {
    Right,
    Left,
}

#[derive(Copy, Clone)]
pub enum BalloonState {
    Alive,
    Popped,
    Escaped,
}

impl PartialEq for BalloonState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Alive, Self::Alive) => true,
            (Self::Popped, Self::Popped) => true,
            (Self::Escaped, Self::Escaped) => true,
            _ => false,
        }
    }
}

const BALLOON_SPRITE_SIZE: f32 = 48.;
const BALLOON_SIZE: f32 = BALLOON_SPRITE_SIZE * 3.;
const BALLOON_COLLIDER_SIZE: f32 = BALLOON_SIZE / 2.;
const BALLOON_SPEED: f32 = 150.;

#[derive(Copy, Clone)]
pub struct Balloon {
    position: Vec2,
    direction: Direction,
    state: BalloonState,
}

impl Balloon {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(BALLOON_SIZE, screen_height() / 2.),
            direction: Direction::Right,
            state: BalloonState::Alive,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        match self.direction {
            Direction::Right => self.position.x += BALLOON_SPEED * delta_time,
            Direction::Left => self.position.x -= BALLOON_SPEED * delta_time,
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_collision_size(&self) -> f32 {
        BALLOON_COLLIDER_SIZE
    }

    pub fn get_state(&self) -> BalloonState {
        self.state
    }

    pub fn set_state(&mut self, state: BalloonState) {
        self.state = state;
    }
}

impl DrawableObject for Balloon {
    fn draw(&self, sprite: Option<&Texture2D>, is_disabled: Option<bool>) {
        draw_texture_ex(
            sprite.unwrap(),
            self.position.x - BALLOON_SIZE / 2.,
            screen_height() / 2. - (BALLOON_SIZE / 2.),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(BALLOON_SIZE, BALLOON_SIZE)),
                ..Default::default()
            },
        );

        draw_circle_lines(
            self.position.x,
            screen_height() / 2.,
            BALLOON_COLLIDER_SIZE,
            2.,
            RED,
        );
    }
}
