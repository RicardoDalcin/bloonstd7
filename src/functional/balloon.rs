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
        matches!(
            (self, other),
            (Self::Alive, Self::Alive)
                | (Self::Popped, Self::Popped)
                | (Self::Escaped, Self::Escaped)
        )
    }
}

const BALLOON_SPRITE_SIZE: f32 = 48.;
const BALLOON_SIZE: f32 = BALLOON_SPRITE_SIZE * 3.;
pub const BALLOON_COLLIDER_SIZE: f32 = BALLOON_SIZE / 2.;
const BALLOON_SPEED: f32 = 150.;

#[derive(Copy, Clone)]
pub struct Balloon {
    pub position: Vec2,
    direction: Direction,
    pub state: BalloonState,
}

pub fn new_balloon() -> Balloon {
    Balloon {
        position: Vec2::new(BALLOON_SIZE, screen_height() / 2.),
        direction: Direction::Right,
        state: BalloonState::Alive,
    }
}

pub fn update_balloon(balloon: Balloon, delta_time: f32) -> Balloon {
    let current_position_x = balloon.position.x;

    let new_position_x = match balloon.direction {
        Direction::Right => current_position_x + BALLOON_SPEED * delta_time,
        Direction::Left => current_position_x - BALLOON_SPEED * delta_time,
    };

    Balloon {
        position: Vec2 {
            x: new_position_x,
            y: balloon.position.y,
        },
        ..balloon
    }
}

pub fn has_escaped(balloon: Balloon) -> bool {
    let position = balloon.position;
    let size = BALLOON_COLLIDER_SIZE;

    position.x > screen_width() + size || position.x < -size
}

pub fn draw_balloon(balloon: Balloon, balloon_texture: Texture2D) {
    draw_texture_ex(
        &balloon_texture,
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
        1.,
        RED,
    );
}
