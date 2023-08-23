use macroquad::prelude::*;

use crate::functional::Balloon::Balloon;

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

pub fn new_projectile(position: Vec2, direction: Vec2) -> Projectile {
    Projectile {
        position,
        direction,
        state: ProjectileState::Alive,
    }
}

pub fn update_projectile(projectile: Projectile, delta_time: f32) -> Projectile {
    let mut projectile = projectile;
    projectile.position += projectile.direction * 500. * delta_time;

    if projectile.position.x < 0.
        || projectile.position.x > screen_width()
        || projectile.position.y < 0.
        || projectile.position.y > screen_height()
    {
        projectile.state = ProjectileState::Dead;
    }

    projectile
}

pub fn is_projectile_alive(projectile: &Projectile) -> bool {
    match projectile.state {
        ProjectileState::Alive => true,
        _ => false,
    }
}

pub fn is_projectile_hit(projectile: &Projectile) -> bool {
    match projectile.state {
        ProjectileState::Hit => true,
        _ => false,
    }
}

pub fn hit_projectile(projectile: Projectile) -> Projectile {
    Projectile {
        state: ProjectileState::Hit,
        ..projectile
    }
}

pub fn draw_projectile(projectile: Projectile) {
    draw_circle(
        projectile.position.x,
        projectile.position.y,
        PROJECTILE_SIZE,
        ORANGE,
    );
}
