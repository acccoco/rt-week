use std::sync::Arc;
use crate::texture::Texture;
use crate::texture::solidcolor::SolidColor;


/// 棋盘格样式的纹理
pub struct CheckerTexture
{
    odd: Arc<dyn Texture + Send + Sync>,
    even: Arc<dyn Texture + Send + Sync>,
}


impl CheckerTexture
{
    pub fn new(odd: Arc<dyn Texture + Send + Sync>, even: Arc<dyn Texture + Send + Sync>) -> CheckerTexture
    {
        CheckerTexture { odd, even }
    }

    pub fn new_c(color1: glm::Vec3, color2: glm::Vec3) -> CheckerTexture
    {
        CheckerTexture {
            odd: Arc::new(SolidColor::new(color1)),
            even: Arc::new(SolidColor::new(color2)),
        }
    }
}


impl Texture for CheckerTexture
{
    fn sample(&self, uv: &glm::Vec2, p: &glm::Vec3) -> glm::Vec3 {
        // 在 xyz 三个方向都会存在纹理交替
        let sines = f32::sin(10.0 * p.x) * f32::sin(10.0 * p.y) * f32::sin(10.0 * p.z);
        if sines < 0.0 {
            self.odd.sample(uv, p)
        } else {
            self.even.sample(uv, p)
        }
    }
}
