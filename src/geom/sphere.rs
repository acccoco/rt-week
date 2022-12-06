use std::sync::Arc;
use glm::ext::Consts;
use num::traits::FloatConst;
use crate::geom::aabb::AABB;
use crate::geom::onb::ONB;
use crate::hit::{HitPayload, Hittable};
use crate::ray::Ray;
use crate::material::Material;
use crate::utility::{check_and, is_normalized, rand_in_cone};


pub struct Sphere
{
    center: glm::Vec3,
    radius: f32,
    mat: Arc<dyn Material + Send + Sync>,
}


impl Sphere
{
    /// 创建球体，允许负数半径
    pub fn new(center: glm::Vec3, radius: f32, mat: Arc<dyn Material + Send + Sync>) -> Sphere
    {
        debug_assert!(check_and(&center, f32::is_finite));
        debug_assert!(radius.is_finite() && radius != 0.0);

        Sphere { center, radius, mat }
    }

    /// 通过等距柱状投影得到球体的纹理坐标
    /// u = phi / (2 * pi), v = theta / pi
    /// p 是单位球上的一个点，确保到原点的距离为 1
    pub fn get_uv(p: &glm::Vec3) -> glm::Vec2
    {
        // 确保 p 位于单位球上
        debug_assert!(is_normalized(p));

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
            if root > t_range.0 && root < t_range.1 { break; }
            root = (-half_b + sqrtd) / a;
            if root > t_range.0 && root < t_range.1 { break; }

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
        Some(AABB::new(
            self.center - self.radius,
            self.center + self.radius,
        ))
    }


    fn pdf(&self, _ray: &Ray) -> f32 {
        // 均匀采样，因此 pdf 是常数
        match self.hit(_ray, (0.001, f32::INFINITY)) {
            None => 0.0,
            Some(hit_payload) => {
                if !hit_payload.front_face() {
                    return 0.0;
                }
                let distance = glm::length(self.center - *_ray.orig());
                let cos_theta_max = f32::sqrt(1.0 - self.radius * self.radius / (distance * distance));
                1.0 / (2.0 * f32::PI() * (1.0 - cos_theta_max))
            }
        }
    }


    /// 以 origin 为顶点，构成一个与球体相切的圆锥，在圆锥范围内随机采样一个方向
    fn rand_dir(&self, _origin: &glm::Vec3) -> Option<(glm::Vec3, f32)> {
        // 圆锥轴线的方向
        let cone_axis = self.center - *_origin;
        let distance = glm::length(cone_axis);

        if !distance.is_finite() || distance <= self.radius {
            // origin 位于球内，无法构成圆锥，无法采样
            return None;
        }

        let sin_theta_max = self.radius / distance;
        let cos_theta_max = f32::sqrt(1.0 - sin_theta_max * sin_theta_max);

        let local_coord = ONB::new(cone_axis);
        let res_dir = local_coord.local(&rand_in_cone(cos_theta_max));

        // 因为是均匀采样，因此 pdf 是常数，和那片区域的立体角有关
        let pdf = 1.0 / (2.0 * f32::PI() * (1.0 - cos_theta_max));

        Some((res_dir, pdf))
    }
}


#[cfg(test)]
mod test
{
    use crate::material::Lambertian;
    use super::*;
    use num::Zero;

    #[test]
    fn test_sphere_rand()
    {
        let p = glm::vec3(3.0, 4.0, 5.0);
        let sphere = Sphere::new(glm::Vec3::zero(), 4.0, Arc::new(Lambertian::new(glm::Vec3::zero())));
        let distance = glm::length(p - sphere.center);
        let axis_dir = glm::normalize(sphere.center - p);

        for _ in 0..100 {
            let (dir, pdf)= sphere.rand_dir(&p).unwrap();
            let ray = Ray::new_d(p, dir);
            assert!((pdf - sphere.pdf(&ray)).abs() < 0.001);

            let sin_theta_max = sphere.radius / distance;
            let cos_theta = glm::dot(axis_dir, dir);
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
            assert!(sin_theta <= sin_theta_max);

            println!("sin: {}, sin max: {}", sin_theta, sin_theta_max);
        }
    }
}
