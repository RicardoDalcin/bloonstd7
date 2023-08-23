use macroquad::prelude::*;

use crate::functional::projectile::new_projectile;
use crate::functional::projectile::Projectile;

#[derive(Clone)]
pub struct Tower {
    pub position: Vec2,
    pub angle: f32,
    shot_cooldown: f32,
    pub projectiles: Vec<Projectile>,
    pop_count: u32,
    level: u32,
}

const TOWER_SIZE: f32 = 50.;

pub fn new_tower(position: Vec2) -> Tower {
    Tower {
        position,
        angle: 0.,
        shot_cooldown: 0.,
        projectiles: Vec::new(),
        pop_count: 0,
        level: 1,
    }
}

pub fn update_tower(tower: Tower, delta_time: f32) -> Tower {
    let new_shot_cooldown = tower.shot_cooldown - delta_time;

    let (new_shot_cooldown, new_projectiles) = if new_shot_cooldown < 0. {
        let new_projectile = new_projectile(
            Vec2::new(tower.position.x, tower.position.y),
            Vec2::new(tower.angle.cos(), tower.angle.sin()),
        );

        (
            new_shot_cooldown + 2. / tower.level as f32,
            vec![new_projectile],
        )
    } else {
        (new_shot_cooldown, vec![])
    };

    let new_projectiles = tower
        .projectiles
        .into_iter()
        .chain(new_projectiles.into_iter())
        .collect();

    Tower {
        shot_cooldown: new_shot_cooldown,
        projectiles: new_projectiles,
        ..tower
    }
}

pub fn increase_tower_pop_count(tower: Tower, pop_count: u32) -> Tower {
    let new_pop_count = tower.pop_count + pop_count;

    let new_level = if new_pop_count % 10 == 0 {
        tower.level + 1
    } else {
        tower.level
    };

    Tower {
        pop_count: new_pop_count,
        level: new_level,
        ..tower
    }
}

pub fn draw_tower(tower: Tower, is_disabled: bool) {
    let color = if is_disabled {
        GRAY
    } else {
        match tower.level {
            1 => BLUE,
            2 => GREEN,
            3 => YELLOW,
            4 => ORANGE,
            5 => RED,
            _ => RED,
        }
    };

    draw_circle(tower.position.x, tower.position.y, TOWER_SIZE, color);
    draw_line(
        tower.position.x,
        tower.position.y,
        tower.position.x + tower.angle.cos() * TOWER_SIZE * 2.,
        tower.position.y + tower.angle.sin() * TOWER_SIZE * 2.,
        2.,
        color,
    );
}
