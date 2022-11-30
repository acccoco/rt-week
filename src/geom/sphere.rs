use std::sync::Arc;
use glm::ext::Consts;
use crate::geom::aabb::AABB;
use crate::ray::HitPayload;
use crate::ray::{Ray, Hittable};
use crate::material::{Material};


pub struct Sphere
{
    center: glm::Vec3,
    radius: f32,
    mat: Arc<dyn Material + Send + Sync>,
}


impl Sphere
{
    pub fn new(center: glm::Vec3, radius: f32, mat: Arc<dyn Material + Send + Sync>) -> Sphere
    {
        Sphere { center, radius, mat }
    }

    /// 通过等距柱状投影得到球体的纹理坐标
    /// u = phi / (2 * pi), v = theta / pi
    /// p 是单位球上的一个点，确保到原点的距离为 1
    pub fn get_uv(p: &glm::Vec3) -> glm::Vec2
    {
        debug_assert!(glm::is_close_to(&glm::length(*p), &1.0, 0.01));

        let theta = f32::acos(-p.y);
        let phi = f32::atan2(-p.z, p.x) + f32::pi();

        glm::vec2(phi / (2.0 * f32::pi()), theta / f32::pi())
    }
}


impl Hittable for Sphere
{
    /// 光线是否和球相交
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<HitPayload>
    {

        // 从光线起点指向球心的向量
        let oc = *ray.orig() - self.center;

        let a = glm::dot(*ray.dir(), *ray.dir());
        let half_b = glm::dot(oc, *ray.dir());
        let c = glm::dot(oc, oc) - self.radius * self.radius;

        // 这就是一元二次方程的判别式和求根公式
        let discriminant = half_b * half_b - a * c;

        // 没有根，当然没有交点
        if discriminant < 0.0 { return None; }

        // 找到最近的符合条件的交点
        let sqrtd = glm::sqrt(discriminant);

        let mut root: f32;
        loop {
            // 优先选择 t 更小的那一个交点
            root = (-half_b - sqrtd) / a;
            if root >= t_range.0 && root < t_range.1 { break; }
            root = (-half_b + sqrtd) / a;
            if root >= t_range.0 && root < t_range.1 { break; }

            // 两个根都不在合适的范围内
            return None;
        };


        let p = ray.at(root);

        // 注：使用 (p - self.center) / self.radius 表示法线，可以将球的半径设为负数，对应的法线指向内侧
        let obj_normal = glm::normalize((p - self.center) / self.radius);

        Some(HitPayload::new(&ray, root, obj_normal, self.mat.clone(),
                             Sphere::get_uv(&obj_normal)))
    }


    fn bounding_box(&self) -> Option<AABB> {
        let aabb = AABB::new(
            self.center - glm::vec3(self.radius, self.radius, self.radius),
            self.center + glm::vec3(self.radius, self.radius, self.radius),
        );
        Some(aabb)
    }
}
