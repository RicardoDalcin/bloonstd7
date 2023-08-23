use macroquad::prelude::*;

mod functional;
mod object_oriented;

use functional::scene::init_scene;
use functional::scene::new_scene;
use functional::scene::update_scene;
use functional::scene::Keys;

// use object_oriented::scene::Scene;

#[macroquad::main("Balloons")]
async fn main() {
    let mut scene = new_scene();
    scene = init_scene(scene).await;

    loop {
        let keys = Keys {
            tower_placement: is_key_pressed(KeyCode::T),
            cancel_tower_placement: is_key_pressed(KeyCode::Escape),
            place_tower: is_mouse_button_down(MouseButton::Left),
            rotate_tower_clockwise: is_key_down(KeyCode::R),
            rotate_tower_counter_clockwise: is_key_down(KeyCode::E),
            reset: is_key_pressed(KeyCode::Enter),
        };

        scene = update_scene(get_frame_time(), keys, scene);
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
