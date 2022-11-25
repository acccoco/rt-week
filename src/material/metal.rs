use crate::material::Material;
use crate::ray::{Ray, HitPayload};
use crate::utility::{rand_in_unit_sphere};


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
        Metal { albedo, fuzz }
    }
}


impl Material for Metal
{
    fn scatter(&self, ray_in: &Ray, hit_payload: &HitPayload) -> Option<(Ray, glm::Vec3)>
    {
        let reflect_dir = glm::reflect(*ray_in.dir(), *hit_payload.normal());

        // 这样就不是镜面反射了，而是特定的反射波瓣
        let scattered_dir = reflect_dir + rand_in_unit_sphere() * self.fuzz;

        if glm::dot(scattered_dir, *hit_payload.normal()) <= 0.0 {
            None
        } else {
            let ray_out = Ray::new(*hit_payload.hit_point(), *hit_payload.hit_point() + scattered_dir);
            Some((ray_out, self.albedo))
        }
    }
}
