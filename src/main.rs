use macroquad::prelude::*;

mod functional;
mod object_oriented;

use functional::Scene::init_scene;
use functional::Scene::new_scene;
use functional::Scene::update_scene;

// use object_oriented::Scene::Scene;

#[macroquad::main("Balloons")]
async fn main() {
    let mut scene = new_scene();
    scene = init_scene(scene).await;

    loop {
        scene = update_scene(get_frame_time(), scene);
        next_frame().await;
    }
}

// #[macroquad::main("Balloons")]
// async fn main() {
//     let mut scene = Scene::new().await;

//     loop {
//         scene.update(get_frame_time());

//         next_frame().await;
//     }
// }
