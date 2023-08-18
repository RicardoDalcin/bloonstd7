use macroquad::prelude::*;
use std::collections::LinkedList;

use crate::lib::Balloon::Balloon;
use crate::lib::Projectile::Projectile;
use crate::lib::Tower::Tower;

struct Sprites {
    background: Texture2D,
    balloon: Texture2D,
}

pub struct Scene {
    coins: u32,
    lives: u32,
    game_over: bool,
    sprites: Sprites,
    balloons: Vec<Balloon>,
    towers: LinkedList<Tower>,
    projectiles: Vec<Projectile>,
    spawn_timer: f32,
    is_placing_tower: bool,
    preview_tower: Option<Tower>,
}

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
            projectiles: Vec::new(),
            spawn_timer: 0.0,
            is_placing_tower: false,
            preview_tower: None,
        }
    }

    pub fn reset(&mut self) {
        self.coins = 0;
        self.lives = 3;
        self.game_over = false;
        self.spawn_timer = 0.0;
        self.is_placing_tower = false;

        self.balloons.clear();
        self.towers.clear();
        self.projectiles.clear();
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

    fn spawn_balloon(&mut self) {
        self.balloons.push(Balloon::new());
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.game_over {
            self.draw_background();

            self.spawn_timer += delta_time;

            if self.spawn_timer > 1.0 {
                self.spawn_timer -= 1.0;
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

                self.preview_tower.as_mut().unwrap().draw();

                if is_mouse_button_down(MouseButton::Left) {
                    self.towers
                        .push_back(self.preview_tower.as_mut().unwrap().clone());

                    self.is_placing_tower = false;
                    self.preview_tower = None;
                }
            }

            self.update_balloons(delta_time);
            self.update_towers(delta_time);
            self.update_projectiles(delta_time);

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
            )
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
            tower.draw();
        }
    }

    fn update_projectiles(&mut self, delta_time: f32) {
        for projectile in &mut self.projectiles {
            projectile.update(delta_time);
            projectile.draw();
        }
    }
}
