use macroquad::prelude::*;

use crate::object_oriented::balloon::Balloon;

#[derive(Copy, Clone)]
pub enum ProjectileState {
    Alive,
    Dead,
    Hit,
}

#[derive(Copy, Clone)]
pub struct Projectile {
    position: Vec2,
    direction: Vec2,
    state: ProjectileState,
}

const PROJECTILE_SIZE: f32 = 15.;

impl Projectile {
    pub fn new(position: Vec2, direction: Vec2) -> Self {
        Self {
            position,
            direction,
            state: ProjectileState::Alive,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position += self.direction * 500. * delta_time;

        if self.position.x < 0.
            || self.position.x > screen_width()
            || self.position.y < 0.
            || self.position.y > screen_height()
        {
            self.state = ProjectileState::Dead;
        }
    }

    pub fn check_collision(&self, balloon: &Balloon) -> bool {
        let balloon_position = balloon.get_position();
        let balloon_collision_size = balloon.get_collision_size();

        let distance_x = (self.position.x - balloon_position.x).abs();
        let distance_y = (self.position.y - balloon_position.y).abs();

        if distance_x > balloon_collision_size + PROJECTILE_SIZE
            || distance_y > balloon_collision_size + PROJECTILE_SIZE
        {
            return false;
        }

        true
    }

    pub fn hit(&mut self) {
        self.state = ProjectileState::Hit;
    }

    pub fn is_alive(&self) -> bool {
        match self.state {
            ProjectileState::Alive => true,
            ProjectileState::Dead => false,
            ProjectileState::Hit => false,
        }
    }

    pub fn is_hit(&self) -> bool {
        match self.state {
            ProjectileState::Alive => false,
            ProjectileState::Dead => false,
            ProjectileState::Hit => true,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, PROJECTILE_SIZE, ORANGE);
    }
}
