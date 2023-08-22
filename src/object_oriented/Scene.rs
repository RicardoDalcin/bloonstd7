use macroquad::prelude::*;
use std::collections::LinkedList;

use crate::object_oriented::Balloon::Balloon;
use crate::object_oriented::Balloon::BalloonState;
use crate::object_oriented::Tower::Tower;

struct Sprites {
    background: Texture2D,
    balloon: Texture2D,
}

pub struct Scene {
    coins: u32,
    lives: i32,
    game_over: bool,
    sprites: Sprites,
    balloons: Vec<Balloon>,
    towers: LinkedList<Tower>,
    spawn_timer: f32,
    is_placing_tower: bool,
    preview_tower: Option<Tower>,
}

const TOWER_COST: u32 = 15;

impl Scene {
    pub async fn new() -> Self {
        let background_sprite = load_texture("resources/sprites/background2.png")
            .await
            .unwrap();

        let balloon_sprite = load_texture("resources/sprites/balloon.png").await.unwrap();

        background_sprite.set_filter(FilterMode::Nearest);
        balloon_sprite.set_filter(FilterMode::Nearest);

        Self {
            coins: 30,
            lives: 3,
            game_over: false,
            sprites: Sprites {
                background: background_sprite,
                balloon: balloon_sprite,
            },
            balloons: Vec::new(),
            towers: LinkedList::new(),
            spawn_timer: 0.0,
            is_placing_tower: false,
            preview_tower: None,
        }
    }

    pub fn reset(&mut self) {
        self.coins = 30;
        self.lives = 3;
        self.game_over = false;
        self.spawn_timer = 0.0;
        self.is_placing_tower = false;

        self.balloons.clear();
        self.towers.clear();
    }

    fn draw_background(&self) {
        clear_background(LIGHTGRAY);

        let scale_factor = screen_width() / self.sprites.background.width();
        let adjusted_width = self.sprites.background.width() * scale_factor;
        let adjusted_height = self.sprites.background.height() * scale_factor;

        let x = (screen_width() - adjusted_width) / 2.0;
        let y = (screen_height() - adjusted_height) / 2.0;

        draw_texture_ex(
            &self.sprites.background,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(adjusted_width, adjusted_height)),
                ..Default::default()
            },
        );
    }

    fn draw_statistics(&self) {
        draw_text(
            format!("COINS: {}", self.coins).as_str(),
            10.,
            32.,
            32.,
            WHITE,
        );

        draw_text(
            format!("LIVES: {}", self.lives).as_str(),
            10.,
            64.,
            32.,
            WHITE,
        );
    }

    fn spawn_balloon(&mut self) {
        self.balloons.push(Balloon::new());
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.game_over {
            self.draw_background();

            self.spawn_timer += delta_time;

            if self.spawn_timer > 1. {
                self.spawn_timer -= 1.;
                self.spawn_balloon();
            }

            if is_key_down(KeyCode::T) {
                self.is_placing_tower = true;
            }

            if self.is_placing_tower {
                let mouse_position = mouse_position();

                if self.preview_tower.is_none() {
                    self.preview_tower =
                        Some(Tower::new(Vec2::new(mouse_position.0, mouse_position.1)));
                }

                if is_key_down(KeyCode::R) {
                    let new_tower_angle =
                        self.preview_tower.as_mut().unwrap().get_angle() + 5. * get_frame_time();

                    self.preview_tower
                        .as_mut()
                        .unwrap()
                        .set_angle(new_tower_angle);
                } else if is_key_down(KeyCode::E) {
                    let new_tower_angle =
                        self.preview_tower.as_mut().unwrap().get_angle() - 5. * get_frame_time();

                    self.preview_tower
                        .as_mut()
                        .unwrap()
                        .set_angle(new_tower_angle);
                }

                self.preview_tower
                    .as_mut()
                    .unwrap()
                    .set_position(Vec2::new(mouse_position.0, mouse_position.1));

                self.preview_tower
                    .as_mut()
                    .unwrap()
                    .draw(self.coins < TOWER_COST);

                if is_mouse_button_down(MouseButton::Left) && self.coins >= TOWER_COST {
                    self.towers
                        .push_back(self.preview_tower.as_mut().unwrap().clone());

                    self.is_placing_tower = false;
                    self.preview_tower = None;
                    self.coins -= TOWER_COST;
                }
            }

            self.update_balloons(delta_time);
            self.update_towers(delta_time);

            for balloon in &mut self.balloons {
                let position = balloon.get_position();
                let size = balloon.get_collision_size();

                if position.x > screen_width() + size || position.x < -size {
                    balloon.set_state(BalloonState::Escaped);
                    self.lives -= 1;

                    if self.lives <= 0 {
                        self.game_over = true;
                    }
                }
            }

            for tower in &mut self.towers {
                for projectile in &mut tower.get_projectiles().iter_mut() {
                    for balloon in &mut self.balloons {
                        if projectile.check_collision(balloon) {
                            projectile.hit();
                            balloon.set_state(BalloonState::Popped);

                            self.coins += 1;
                        }
                    }
                }

                tower.clean_projectiles();
            }

            self.balloons
                .retain(|balloon| balloon.get_state() == BalloonState::Alive);

            self.draw_statistics();
        } else {
            clear_background(WHITE);
            let text = "Game Over. Press [enter] to play again.";
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2.,
                font_size,
                DARKGRAY,
            );

            if is_key_down(KeyCode::Enter) {
                self.reset();
            }
        }
    }

    fn update_balloons(&mut self, delta_time: f32) {
        for balloon in &mut self.balloons {
            balloon.update(delta_time);
            balloon.draw(&self.sprites.balloon);
        }
    }

    fn update_towers(&mut self, delta_time: f32) {
        for tower in &mut self.towers {
            tower.update(delta_time);
            tower.draw(false);

            for projectile in &mut tower.get_projectiles().iter_mut() {
                projectile.update(delta_time);
                projectile.draw();
            }
        }
    }
}
