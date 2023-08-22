use macroquad::prelude::*;

mod functional;
mod object_oriented;

use functional::Scene::init_scene;
use functional::Scene::new_scene;
use functional::Scene::update_scene;

use object_oriented::Scene::Scene;

#[macroquad::main("Balloons")]
async fn main() {
    // let mut scene = Scene::new().await;

    let mut scene = new_scene();

    loop {
        // scene.update(get_frame_time());

        update_scene(get_frame_time(), &mut scene);

        next_frame().await;
    }
}
