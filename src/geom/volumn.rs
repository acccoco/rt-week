use std::sync::Arc;
use glm::ext::Consts;
use crate::geom::aabb::AABB;
use crate::hit::{HitPayload, Hittable};
use crate::material::{Material, Scatter};
use crate::pdf::RandSpherePDF;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::utility::rand_unit_vec;


/// 密度是常数的介质
pub struct ConstantMedium
{
    /// 假定包围体是凸的
    boundary: Arc<dyn Hittable + Sync + Send>,
    phase_function: Arc<dyn Material + Sync + Send>,

    neg_inv_density: f32,
}


impl ConstantMedium
{
    pub fn new(boundary: Arc<dyn Hittable + Sync + Send>, density: f32, albedo: Arc<dyn Texture + Sync + Send>) -> ConstantMedium
    {
        debug_assert!(density.is_finite() && density != 0.0);

        ConstantMedium { boundary, neg_inv_density: -1.0 / density, phase_function: Arc::new(Isotropic::new(albedo)) }
    }

    pub fn new_c(boundary: Arc<dyn Hittable + Sync + Send>, density: f32, color: glm::Vec3) -> ConstantMedium
    {
        debug_assert!(density.is_finite() && density != 0.0);

        ConstantMedium { boundary, neg_inv_density: -1.0 / density, phase_function: Arc::new(Isotropic::new_c(color)) }
    }
}


impl Hittable for ConstantMedium
{
    /// 在介质内，每前进单位距离，就有固定的概率发生散射，概率与介质密度有关
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<HitPayload> {
        // 得到光线关于 boundary 的两个交点，其中 hit1 是符合条件的 t 最小的交点
        // 需要包围体是凸的
        let hit_payload1 = match self.boundary.hit(ray, (f32::NEG_INFINITY, f32::INFINITY)) {
            None => { return None; }
            Some(payload) => payload
        };

        let hit_payload2 = match self.boundary.hit(ray, (hit_payload1.t() + 0.0001, f32::INFINITY)) {
            None => { return None; }
            Some(payload) => payload
        };

        // clamp 两个交点的范围
        let t1 = f32::max(hit_payload1.t(), t_range.0);
        let t2 = f32::min(hit_payload2.t(), t_range.1);
        if t1 >= t2 { return None; }
        let t1 = f32::max(0.0, t1);


        // 光线在介质内可以走的最大距离
        let distance_inside_boundary = t2 - t1;

        // 注：对数函数在 (0, 1) 的运算结果是负数
        // 在雾中发生散射是一个泊松过程，lambda = density（单位距离发生散射的概率/次数）
        // 「散射距离」符合「爱尔兰」分布，根据分布变换，可以从 uniform 分布的随机数得到「散射距离」这个随机变量。
        // hit_distance 的取值范围是 [0, +inf]，不会发生意外错误
        let hit_distance = self.neg_inv_density * glm::log(rand::random::<f32>());

        // 光线能够直接穿过介质而不发生散射
        if hit_distance > distance_inside_boundary { return None; }

        let t = t1 + hit_distance;
        let normal = glm::vec3(1.0, 0.0, 0.0);
        // NOTE 说是需要保证始终是 front_face，目前没有看出什么影响
        // https://raytracing.github.io/books/RayTracingTheNextWeek.html#volumes


        Some(HitPayload::new(ray, t, normal, self.phase_function.clone(), glm::vec2(0.0, 0.0)))
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.boundary.bounding_box()
    }
}


/// 各向同性材质
pub struct Isotropic
{
    albedo: Arc<dyn Texture + Sync + Send>,
}


impl Isotropic
{
    pub fn new(albedo: Arc<dyn Texture + Sync + Send>) -> Isotropic
    {
        Isotropic { albedo }
    }

    pub fn new_c(albedo: glm::Vec3) -> Isotropic
    {
        Isotropic { albedo: Arc::new(SolidColor::new(albedo)) }
    }
}


impl Material for Isotropic
{
    /// ios 介质会让散射方向随机
    fn scatter(&self, _ray_in: &Ray, hit_payload: &HitPayload) -> Option<Scatter> {
        Some(Scatter {
            attenuation: self.albedo.sample(hit_payload.uv(), hit_payload.hit_point()),
            diffuse_pdf: None,
            specular_ray: Some(Ray::new_d(*hit_payload.hit_point(), rand_unit_vec())),
        })
    }
}