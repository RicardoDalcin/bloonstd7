use macroquad::prelude::*;

mod lib;

use lib::Scene::Scene;

#[macroquad::main("Balloons")]
async fn main() {
    let mut scene = Scene::new().await;

    loop {
        scene.update(get_frame_time());

        next_frame().await;
    }
}
