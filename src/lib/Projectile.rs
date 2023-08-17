use macroquad::prelude::*;

pub struct Projectile {
    position: Vec2,
    direction: Vec2,
    is_alive: bool,
}

const PROJECTILE_SIZE: f32 = 15.;

impl Projectile {
    pub fn new(position: Vec2, direction: Vec2) -> Self {
        Self {
            position,
            direction,
            is_alive: true,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position += self.direction * 500. * delta_time;

        if self.position.x < 0.
            || self.position.x > screen_width()
            || self.position.y < 0.
            || self.position.y > screen_height()
        {
            self.is_alive = false;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, PROJECTILE_SIZE, ORANGE);
    }
}
