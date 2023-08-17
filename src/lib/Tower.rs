use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Tower {
    position: Vec2,
    angle: f32,
    shot_cooldown: f32,
}

const TOWER_SIZE: f32 = 50.;

impl Tower {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            angle: 0.,
            shot_cooldown: 0.,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.shot_cooldown -= delta_time;
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, TOWER_SIZE, BLUE);
        draw_line(
            self.position.x,
            self.position.y,
            self.position.x + self.angle.cos() * TOWER_SIZE * 2.,
            self.position.y + self.angle.sin() * TOWER_SIZE * 2.,
            2.,
            BLUE,
        );
    }
}
