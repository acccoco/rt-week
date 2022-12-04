use crate::material::{Material, Scatter};
use crate::ray::Ray;
use num::One;
use num::pow::Pow;
use crate::hit::HitPayload;


pub struct Dielecric
{
    ir: f32,    // 材质的折射系数 index of refraction
}


impl Dielecric
{
    pub fn new(ir: f32) -> Dielecric
    {
        Dielecric { ir }
    }
}


/// snell 折射定律
/// i 是入射方向，n 是法线，两者都是单位向量
/// rate 表示两个界面的相对折射率
fn refract(i: glm::Vec3, n: glm::Vec3, refraction_ratio: f32) -> glm::Vec3
{
    let cos_theta = f32::min(glm::dot(-i, n), 1.0);

    let r_out_perp = (i + n * cos_theta) * refraction_ratio;
    let r_out_parallel = -n * f32::sqrt(f32::abs(1.0 - glm::dot(r_out_perp, r_out_perp)));

    r_out_perp + r_out_parallel
}


/// 使用 Schlick 近似来计算菲涅尔方程，得到反射能量占比
fn reflectance(cos_theta: f32, ref_idx: f32) -> f32
{
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * f32::pow(1. - cos_theta, 5_u8)
}


impl Material for Dielecric
{
    fn scatter(&self, ray_in: &Ray, hit_payload: &HitPayload) -> Option<Scatter> {
        let refraction_ratio = if hit_payload.front_face() { 1.0 / self.ir } else { self.ir };

        let cos_theta = f32::min(glm::dot(-*ray_in.dir(), *hit_payload.normal()), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let scatter_dir =
            // 从 snell 和 fresnell 两个角度来判断是否发生全反射
            if refraction_ratio * sin_theta > 1.0 || reflectance(cos_theta, refraction_ratio) > rand::random() {
                // 全反射
                glm::reflect(*ray_in.dir(), *hit_payload.normal())
            } else {
                // 既有折射，又有反射
                refract(*ray_in.dir(), *hit_payload.normal(), refraction_ratio)
            };


        todo!()
        // Some(Scatter {
        //     monte_pdf: 1.0,         // FIXME
        //     albedo: glm::Vec3::one(),
        //     scatter_ray: Ray::new(*hit_payload.hit_point(), *hit_payload.hit_point() + scatter_dir),
        // })
    }


    fn scatter_pdf(&self, _ray_in: &Ray, _hit_payload: &HitPayload, _ray_out: &Ray) -> f32 {
        todo!()
    }
}
