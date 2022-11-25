pub trait Texture
{
    fn sample(&self, uv: &glm::Vec2, p: &glm::Vec3) -> glm::Vec3;
}


mod checker;
mod solidcolor;
mod image;


pub use checker::CheckerTexture;
pub use solidcolor::SolidColor;
pub use image::ImageTexture;


