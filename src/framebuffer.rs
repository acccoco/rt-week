use geefr_ppm::Ppm as PPM;
use crate::utility::{gamma_correction};


#[derive(Clone, Copy)]
pub struct Grid {
    pub pos: (u32, u32),
    pub size: (u32, u32),
}


impl Grid {
    pub fn iter(&self) -> Vec<(u32, u32)>
    {
        let mut res = Vec::with_capacity((self.size.0 * self.size.1) as usize);
        let mut pos = self.pos;

        while pos.0 < self.pos.0 + self.size.0 {
            pos.1 = self.pos.1;
            while pos.1 < self.pos.1 + self.size.1 {
                res.push(pos);
                pos.1 += 1;
            }
            pos.0 += 1;
        }

        res
    }
}


// 原点位于左上角
pub struct FrameBuffer {
    width: u32,
    height: u32,
    ppm: PPM,
}


impl FrameBuffer
{
    pub fn new(width: u32, aspect: f32) -> FrameBuffer
    {
        let height = (width as f32 / aspect) as u32;
        FrameBuffer {
            width,
            height,
            ppm: PPM::new(width as usize, height as usize),
        }
    }

    /// 将颜色写入 ppm 中
    /// - 颜色是真实的颜色，范围可以超过 1，该函数会进行 Gamma 矫正
    pub fn write_color(&mut self, pos: (u32, u32), color: &glm::Vec3)
    {
        debug_assert!(color.x >= 0.0 && color.y >= 0.0 && color.z >= 0.0);

        let color = gamma_correction(*color);

        // 将颜色从浮点数截取到 [0, 1] 范围，并使用 unormal-u8 进行编码
        let to_ppm_color = |c: f32| (c.clamp(0.0, 0.999) * 256.0) as u8;

        self.ppm.set_pixel(pos.0 as usize, pos.1 as usize,
                           to_ppm_color(color.x),
                           to_ppm_color(color.y),
                           to_ppm_color(color.z));
    }


    /// 获得屏幕上某一点对应的 uv，如果超出范围，则返回 None
    pub fn get_uv(&self, pos: (u32, u32)) -> Option<(f32, f32)>
    {
        if pos.0 >= self.width || pos.1 >= self.height { return None; }
        Some((pos.0 as f32 / self.width as f32, pos.1 as f32 / self.height as f32))
    }


    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }


    /// 为某个像素生成多个 sample，并计算每个 sample 的 uv
    pub fn multi_sample(framebuffer_size: (u32, u32), pos: (u32, u32), samples: u32) -> Vec<(f32, f32)>
    {
        debug_assert!(pos.0 < framebuffer_size.0 && pos.1 < framebuffer_size.1);
        debug_assert!(samples > 0);

        let width_inv = 1.0 / framebuffer_size.0 as f32;
        let height_inv = 1.0 / framebuffer_size.1 as f32;

        let gen_uv = |_|
            ((pos.0 as f32 + rand::random::<f32>()) * width_inv,
             (pos.1 as f32 + rand::random::<f32>()) * height_inv);

        (0..samples).map(gen_uv).collect()
    }


    /// 将结果写入文件中
    pub fn save(&self, file_path: String) -> std::io::Result<()>
    {
        self.ppm.write(file_path)
    }


    pub fn pixel_iter(&self) -> impl Iterator<Item=(u32, u32)>
    {
        let w = self.width;
        let foo = move |v: u32| (v % w, v / w);

        (0..(self.width * self.height)).map(foo)
    }


    /// 将 framebuffer 切分为 grid，grid 的尺寸可能不完全相等
    pub fn split_to_tile(&self, tile_size: u32) -> Vec<Grid>
    {
        let mut tiles = Vec::with_capacity(
            ((self.width + tile_size - 1) / tile_size) as usize
                * ((self.height + tile_size - 1) / tile_size) as usize);

        let mut pos = (0_u32, 0_u32);
        while pos.0 < self.width {
            pos.1 = 0;
            while pos.1 < self.height {
                let tile_width = if pos.0 + tile_size < self.width {
                    tile_size
                } else {
                    self.width - pos.0
                };

                let tile_height = if pos.1 + tile_size < self.height {
                    tile_size
                } else {
                    self.height - pos.1
                };

                tiles.push(Grid { pos, size: (tile_width, tile_height) });
                pos.1 += tile_size;
            }
            pos.0 += tile_size;
        }

        tiles
    }
}