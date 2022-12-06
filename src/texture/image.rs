use num::Zero;
use crate::texture::Texture;
use stb_image::image as stbi;


/// 图形纹理
pub struct ImageTexture
{
    img: stbi::Image<u8>,
}


impl ImageTexture
{
    pub fn new(filename: &String) -> ImageTexture
    {
        let img = match stbi::load(filename) {
            stbi::LoadResult::Error(msg) => {
                panic!("error load image({}): {}", filename, msg);
            }
            stbi::LoadResult::ImageF32(_) => {
                panic!("currently not support f32 image: {}", filename);
            }
            stbi::LoadResult::ImageU8(img) => img
        };

        ImageTexture { img }
    }


    fn get_color(&self, pos: (usize, usize)) -> glm::Vec4
    {
        debug_assert!(pos.0 < self.img.width && pos.1 < self.img.height);

        let idx = pos.1 * (self.img.width * self.img.depth) + pos.0 * self.img.depth;

        let mut color = glm::Vec4::zero();
        let color_scale = 1.0 / 255.0;
        for i in 0..self.img.depth {
            color[i] = self.img.data[idx + i] as f32 * color_scale;
        }

        color
    }
}


impl Texture for ImageTexture
{
    fn sample(&self, uv: &glm::Vec2, _p: &glm::Vec3) -> glm::Vec3 {
        // 将 uv 的范围限制在 [0, 1]，并翻转 v（这个和 stbi 的读取有关）
        let u = uv.x.clamp(0.0, 1.0);
        let v = 1.0 - uv.y.clamp(0.0, 1.0);

        let i = ((self.img.width as f32 * u) as usize).min(self.img.width - 1);
        let j = ((self.img.height as f32 * v) as usize).min(self.img.height - 1);

        let color = self.get_color((i, j));
        glm::vec3(color.x, color.y, color.z)
    }
}