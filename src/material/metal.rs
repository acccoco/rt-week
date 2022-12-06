use crate::hit::HitPayload;
use crate::material::{Material, Scatter};
use crate::ray::Ray;
use crate::utility::rand_in_unit_sphere;


/// 金属材质，并不是基于物理的
pub struct Metal
{
    albedo: glm::Vec3,
    fuzz: f32,
}


impl Metal
{
    pub fn new(albedo: glm::Vec3, fuzz: f32) -> Metal
    {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}


impl Material for Metal
{
    fn scatter(&self, ray_in: &Ray, hit_payload: &HitPayload) -> Option<Scatter>
    {
        let reflect_dir = glm::reflect(*ray_in.dir(), *hit_payload.normal());

        let specular_ray = Ray::new_d(*hit_payload.hit_point(),
                                      glm::normalize(reflect_dir + rand_in_unit_sphere() * self.fuzz));

        if glm::dot(*specular_ray.dir(), *hit_payload.normal()) <= 0.0 {
            return None;
        }

        Some(Scatter {
            specular_ray: Some(specular_ray),
            diffuse_pdf: None,
            attenuation: self.albedo,
        })
    }
}
