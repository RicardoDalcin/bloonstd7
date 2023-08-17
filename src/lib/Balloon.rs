use macroquad::prelude::*;

enum Direction {
    Right,
    Left,
}

const BALLOON_SPRITE_SIZE: f32 = 32.;
const BALLOON_SIZE: f32 = BALLOON_SPRITE_SIZE * 3.;
const BALLOON_COLLIDER_SIZE: f32 = BALLOON_SIZE / 2.;
const BALLOON_SPEED: f32 = 150.;

pub struct Balloon {
    position: Vec2,
    direction: Direction,
}

impl Balloon {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(BALLOON_SIZE, screen_height() / 2.),
            direction: Direction::Right,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        match self.direction {
            Direction::Right => self.position.x += BALLOON_SPEED * delta_time,
            Direction::Left => self.position.x -= BALLOON_SPEED * delta_time,
        }

        if self.position.x > screen_width() + BALLOON_SIZE {
            self.position.x = -BALLOON_SIZE;
        } else if self.position.x < -BALLOON_SIZE {
            self.position.x = screen_width() + BALLOON_SIZE;
        }
    }

    pub fn draw(&self, balloon_sprite: &Texture2D) {
        draw_texture_ex(
            &balloon_sprite,
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
