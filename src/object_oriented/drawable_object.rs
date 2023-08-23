use macroquad::texture::Texture2D;

pub trait DrawableObject {
    fn draw(&self, sprite: Option<&Texture2D>, is_disabled: Option<bool>);
}
