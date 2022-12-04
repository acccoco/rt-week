use std::sync::Arc;
use num::traits::FloatConst;

use crate::hit::HitPayload;

use crate::material::{Material, Scatter};
use crate::pdf::CosPDF;
use crate::ray::Ray;

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
        let pdf = CosPDF::new(*hit_payload.normal());

        Some(Scatter {
            pdf: Box::new(pdf),
            albedo: self.albedo.sample(hit_payload.uv(), hit_payload.hit_point()),
        })
    }


    /// 根据另一种形式的反射方程，朝某个方向散射的 pdf = cos(theta) / pi
    fn scatter_pdf(&self, _ray_in: &Ray, _hit_payload: &HitPayload, _ray_out: &Ray) -> f32 {
        f32::max(0.0,
                 glm::dot(*_hit_payload.normal(), *_ray_out.dir()) / f32::PI())
    }
}
