use std::sync::Arc;

use crate::material::{Material};
use crate::ray::{Ray, HitPayload};
use crate::utility::{near_zero, rand_unit_vec};
use crate::texture::{SolidColor, Texture};

/// Lambert 材质，fr = albedo / pi
pub struct Lambertian
{
    albedo: Arc<dyn Texture + Sync + Send>,
}


impl Lambertian
{
    pub fn new(albedo: glm::Vec3) -> Lambertian
    {
        Lambertian { albedo: Arc::new(SolidColor::new(albedo)) }
    }


    pub fn new_t(albedo: Arc<dyn Texture + Sync + Send>) -> Lambertian
    {
        Lambertian { albedo }
    }
}


impl Material for Lambertian
{
    /// 对于 Lambert 材质，选择的概率密度函数是 fW = cos\theta / pi
    /// 可以得到 attenuation = albedo
    fn scatter(&self, _: &Ray, hit_payload: &HitPayload) -> Option<(Ray, glm::Vec3)>
    {
        let mut scatter_dir = *hit_payload.normal() + rand_unit_vec();

        // 如果随机生成的 target 距离 hit point 非常近，可能会导致除 0 错误
        if near_zero(&scatter_dir) {
            scatter_dir = *hit_payload.normal();
        }

        let ray_out = Ray::new(*hit_payload.hit_point(), *hit_payload.hit_point() + scatter_dir);
        Some((ray_out, self.albedo.sample(hit_payload.uv(), hit_payload.hit_point())))
    }
}
