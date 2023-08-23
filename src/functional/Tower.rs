use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Tower {
    pub position: Vec2,
    pub angle: f32,
    shot_cooldown: f32,
    // projectiles: Vec<Projectile>,
    pop_count: u32,
    level: u32,
}

const TOWER_SIZE: f32 = 50.;

pub fn new_tower(position: Vec2) -> Tower {
    Tower {
        position,
        angle: 0.,
        shot_cooldown: 0.,
        // projectiles: Vec::new(),
        pop_count: 0,
        level: 1,
    }
}

pub fn update_tower(tower: Tower, delta_time: f32) -> Tower {
    let mut next_tower = tower.clone();

    next_tower.shot_cooldown -= delta_time;

    if next_tower.shot_cooldown < 0. {
        // let new_projectile = Projectile::new(
        //   Vec2::new(self.position.x, self.position.y),
        //   Vec2::new(self.angle.cos(), self.angle.sin()),
        // );

        // self.projectiles.push(new_projectile);
        next_tower.shot_cooldown += 2. / next_tower.level as f32;
    }

    return next_tower;
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
