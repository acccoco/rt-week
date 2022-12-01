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
        Metal { albedo, fuzz }
    }
}


impl Material for Metal
{
    fn scatter(&self, ray_in: &Ray, hit_payload: &HitPayload) -> Option<Scatter>
    {
        let reflect_dir = glm::reflect(*ray_in.dir(), *hit_payload.normal());

        // 这样就不是镜面反射了，而是特定的反射波瓣
        let scattered_dir = reflect_dir + rand_in_unit_sphere() * self.fuzz;

        if glm::dot(scattered_dir, *hit_payload.normal()) <= 0.0 {
            None
        } else {
            Some(Scatter {
                monte_pdf: 1.0,     // TODO
                scatter_ray: Ray::new(*hit_payload.hit_point(), *hit_payload.hit_point() + scattered_dir),
                albedo: self.albedo,
            })
        }
    }


    fn scatter_pdf(&self, _ray_in: &Ray, _hit_payload: &HitPayload, _ray_out: &Ray) -> f32 {
        todo!()
    }
}
