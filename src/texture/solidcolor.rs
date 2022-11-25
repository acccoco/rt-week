use crate::texture::Texture;


pub struct SolidColor
{
    color: glm::Vec3,
}


impl SolidColor
{
    pub fn new(color: glm::Vec3) -> Self
    {
        SolidColor { color }
    }
}


impl Default for SolidColor {
    fn default() -> Self {
        SolidColor { color: glm::vec3(0., 0., 0.) }
    }
}


impl Texture for SolidColor
{
    fn sample(&self, _uv: &glm::Vec2, _p: &glm::Vec3) -> glm::Vec3 {
        self.color
    }
}
