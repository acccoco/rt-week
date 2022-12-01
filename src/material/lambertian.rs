use std::sync::Arc;
use num::traits::FloatConst;
use crate::hit::HitPayload;

use crate::material::{Material, Scatter};
use crate::ray::Ray;
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
        debug_assert!(albedo.x >= 0.0 && albedo.y >= 0.0 && albedo.z >= 0.0);

        Lambertian { albedo: Arc::new(SolidColor::new(albedo)) }
    }


    pub fn new_t(albedo: Arc<dyn Texture + Sync + Send>) -> Lambertian
    {
        Lambertian { albedo }
    }
}


impl Material for Lambertian
{
    fn scatter(&self, _: &Ray, hit_payload: &HitPayload) -> Option<Scatter>
    {
        // 用于 Monte Carlo 积分的 pdf = cos(theta) / pi，随意选择的
        let mut scatter_dir = *hit_payload.normal() + rand_unit_vec();

        // 如果随机生成的 target 距离 hit point 非常近，可能会导致除 0 错误
        if near_zero(&scatter_dir) {
            scatter_dir = *hit_payload.normal();
        }

        let ray_out = Ray::new(*hit_payload.hit_point(), *hit_payload.hit_point() + scatter_dir);

        Some(Scatter {
            monte_pdf: f32::max(f32::MIN_POSITIVE, glm::dot(*hit_payload.normal(), *ray_out.dir()) / f32::PI()),
            scatter_ray: ray_out,
            albedo: self.albedo.sample(hit_payload.uv(), hit_payload.hit_point()),
        })
    }


    /// 根据另一种形式的反射方程，朝某个方向散射的 pdf = cos(theta) / pi
    fn scatter_pdf(&self, _ray_in: &Ray, _hit_payload: &HitPayload, _ray_out: &Ray) -> f32 {
        f32::max(0.0,
                 glm::dot(*_hit_payload.normal(), *_ray_out.dir()) / f32::PI())
    }
}
