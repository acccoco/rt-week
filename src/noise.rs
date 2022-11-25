use num::{One, Zero};
use rand::{Rng, Rand};
use crate::texture::Texture;


const PERLIN_POINT: usize = 256;


pub struct Perlin
{
    perm_x: [i32; PERLIN_POINT],
    perm_y: [i32; PERLIN_POINT],
    perm_z: [i32; PERLIN_POINT],

    ranvec: [glm::Vec3; PERLIN_POINT],
}


impl Perlin
{
    pub fn new() -> Self
    {
        let mut rng = rand::thread_rng();
        let mut ranvec = [glm::Vec3::zero(); PERLIN_POINT];
        for v in &mut ranvec {
            *v = glm::normalize(glm::Vec3::rand(&mut rng) * 2.0 - 1.0);
        }

        Perlin {
            ranvec,
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }


    fn generate_perm() -> [i32; PERLIN_POINT]
    {
        let mut p = [0 as i32; PERLIN_POINT];
        for i in 0..PERLIN_POINT {
            p[i] = i as i32;
        }

        Self::permute(p, PERLIN_POINT as i32)
    }

    /// permute: vt. 变换
    fn permute(mut p: [i32; PERLIN_POINT], n: i32) -> [i32; PERLIN_POINT]
    {
        let mut rng = rand::thread_rng();
        for i in (1..n as usize).rev() {
            let target = rng.gen_range(0, i + 1);
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
        p
    }


    pub fn noise(&self, p: &glm::Vec3) -> f32
    {
        // 确保 uvw 的范围是 [0, 1] 用作插值系数
        let u = p.x - f32::floor(p.x);
        let v = p.y - f32::floor(p.y);
        let w = p.z - f32::floor(p.z);

        // 使用 i32，可以保留负数。可以确保按位与的结果是正数
        let i = f32::floor(p.x) as i32;
        let j = f32::floor(p.y) as i32;
        let k = f32::floor(p.z) as i32;

        let mut c = [[[glm::Vec3::zero(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let temp = self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize];

                    c[di][dj][dk] = self.ranvec[temp as usize];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }


    /// 三线性插值，c 是锚点，uvw 是插值系数
    fn trilinear_interp(c: [[[glm::Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32
    {
        // 使用 Hermit 对插值系数进行平滑处理
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);


        let mut accum = 0.0_f32;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = glm::vec3(u - i as f32, v - j as f32, w - k as f32);
                    accum += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                        * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                        * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                        * glm::dot(c[i][j][k], weight_v);
                }
            }
        }

        accum
    }


    /// 湍流：将多个噪声复合
    pub fn turb(&self, p: &glm::Vec3, depth: Option<i32>) -> f32
    {
        // depth 的默认值是 7
        let depth = depth.unwrap_or(7);

        let mut accum = 0.0_f32;
        let mut temp_p = *p;
        let mut weight = 1.0_f32;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        f32::abs(accum)
    }
}


pub struct NoiseTexture
{
    noise: Perlin,
    scale: f32, // 可以改变噪声的频率
}


impl NoiseTexture
{
    pub fn new(scale: f32) -> NoiseTexture { NoiseTexture { noise: Perlin::new(), scale } }
}


impl Texture for NoiseTexture
{
    fn sample(&self, _uv: &glm::Vec2, p: &glm::Vec3) -> glm::Vec3 {
        // 模糊的 Perlin 噪声
        // glm::Vec3::one() * 0.5 * (self.noise.noise(&(*p * self.scale)) + 1.0)

        // 网状的纹理
        // glm::Vec3::one() * self.noise.turb(&(*p * self.scale), None)

        // 大理石纹理
        glm::Vec3::one() * 0.5 * (1.0 + f32::sin(self.scale * p.z + 10.0 * self.noise.turb(p, None)))
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use num::Zero;

    #[test]
    fn t1() {
        let perlin = NoiseTexture::new(2.0);

        println!("{:?}", perlin.sample(&glm::Vec2::zero(), &glm::vec3(-50.0, -50.0, -50.0)));
        println!("{:?}", perlin.sample(&glm::Vec2::zero(), &glm::vec3(-51.0, -51.0, -51.0)));
    }
}