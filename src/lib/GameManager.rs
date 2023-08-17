use crate::lib::Scene::Scene;

pub struct GameManager {
    pub score: u32,
    pub lives: u32,
    pub game_over: bool,
    pub scene: Scene,
}

impl GameManager {
    pub async fn new() -> Self {
        Self {
            score: 0,
            lives: 3,
            game_over: false,
            scene: Scene::new().await,
        }
    }

    pub fn reset(&mut self) {
        self.score = 0;
        self.lives = 3;
        self.game_over = false;
        self.scene.reset();
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.game_over {
            self.scene.update(delta_time);
        }
    }
}
