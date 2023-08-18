use macroquad::prelude::*;

mod lib;

use lib::Scene::Scene;

#[macroquad::main("Balloons")]
async fn main() {
    let mut scene = Scene::new().await;

    loop {
        scene.update(get_frame_time());

        // if get_time() - last_update > 1. / 60. {
        //     last_update = get_time();
        //     score += 1;
        // }

        // for tower in &mut towers {
        //     tower.shot_cooldown -= get_frame_time();

        //     if (tower.shot_cooldown <= 0.) {
        //         let new_projectile: Projectile = Projectile {
        //             position: Vec2::new(tower.position.x, tower.position.y),
        //             direction: Vec2::new(tower.angle.cos(), tower.angle.sin()),
        //             is_alive: true,
        //         };

        //         projectiles.push(new_projectile);
        //         tower.shot_cooldown = 1.;
        //     }
        // }

        // for projectile in &mut projectiles {
        //     let balloons_before = balloons.len();

        //     balloons = balloons
        //         .into_iter()
        //         .filter(|balloon| !check_collision(projectile, balloon))
        //         .collect();

        //     if balloons.len() < balloons_before {
        //         projectile.is_alive = false;
        //     }

        // }

        // projectiles = projectiles
        //     .into_iter()
        //     .filter(|projectile| projectile.is_alive)
        //     .collect();

        next_frame().await;
    }
}
