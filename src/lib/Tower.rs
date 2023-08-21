use macroquad::prelude::*;

use crate::lib::Projectile::Projectile;

#[derive(Clone)]
pub struct Tower {
    position: Vec2,
    angle: f32,
    shot_cooldown: f32,
    projectiles: Vec<Projectile>,
    pop_count: u32,
    level: u32,
}

const TOWER_SIZE: f32 = 50.;

impl Tower {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            angle: 0.,
            shot_cooldown: 0.,
            projectiles: Vec::new(),
            pop_count: 0,
            level: 1,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.shot_cooldown -= delta_time;

        if self.shot_cooldown < 0. {
            let new_projectile = Projectile::new(
                Vec2::new(self.position.x, self.position.y),
                Vec2::new(self.angle.cos(), self.angle.sin()),
            );

            self.projectiles.push(new_projectile);
            self.shot_cooldown += 2. / self.level as f32;
        }
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

    pub fn get_projectiles(&mut self) -> &mut Vec<Projectile> {
        &mut self.projectiles
    }

    pub fn clean_projectiles(&mut self) {
        let projectiles_count = self.projectiles.len();

        self.projectiles.retain(|projectile| !projectile.is_hit());

        let projectiles_hit = projectiles_count - self.projectiles.len();

        self.projectiles.retain(|projectile| projectile.is_alive());

        if (projectiles_hit > 0) {
            self.increase_pop_count(Some(projectiles_hit as u32));
        }
    }

    fn increase_pop_count(&mut self, pop_count: Option<u32>) {
        self.pop_count += match pop_count {
            Some(count) => count,
            None => 1,
        };

        if self.pop_count % 10 == 0 {
            self.level += 1;
        }
    }

    pub fn draw(&self, is_disabled: bool) {
        let color = if is_disabled {
            GRAY
        } else {
            match self.level {
                1 => BLUE,
                2 => GREEN,
                3 => YELLOW,
                4 => ORANGE,
                5 => RED,
                _ => RED,
            }
        };

        draw_circle(self.position.x, self.position.y, TOWER_SIZE, color);
        draw_line(
            self.position.x,
            self.position.y,
            self.position.x + self.angle.cos() * TOWER_SIZE * 2.,
            self.position.y + self.angle.sin() * TOWER_SIZE * 2.,
            2.,
            color,
        );
    }
}
