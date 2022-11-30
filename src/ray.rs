use std::sync::Arc;

use crate::geom::aabb::{AABB};
use crate::utility::{check_or};
use crate::material::{Material};


#[derive(Debug)]
pub struct Ray
{
    orig: glm::Vec3,
    // 请确保该向量一定是单位向量
    dir: glm::Vec3,
}


impl Ray {
    /// 通过起点和目标点的方式来创建射线
    pub fn new(orig: glm::Vec3, target: glm::Vec3) -> Ray
    {
        let dir = glm::normalize(target - orig);
        debug_assert!(!check_or(&dir, f32::is_nan), "ray dir NaN");

        Ray { orig, dir }
    }

    /// dir 是方向，确保是单位向量
    pub fn new_d(orig: glm::Vec3, dir: glm::Vec3) -> Ray
    {
        debug_assert!(glm::is_close_to(&glm::length(dir), &1.0_f32, 0.01_f32));

        Ray { orig, dir }
    }


    pub fn orig(&self) -> &glm::Vec3 { &self.orig }
    pub fn dir(&self) -> &glm::Vec3 { &self.dir }


    // 射线方向上，距离原点 t 的点的坐标
    pub fn at(&self, t: f32) -> glm::Vec3
    {
        debug_assert!(!t.is_nan());

        self.orig + self.dir * t
    }

    // 将光源方向映射到特定的颜色
    pub fn debug_color(&self) -> glm::Vec3
    {
        // dir 是单位向量，因此 t 的范围是 [0, 1]
        let t = 0.5 * (self.dir[1] + 1.0);
        glm::mix_s(glm::vec3(1.0, 1.0, 1.0),
                   glm::vec3(0.5, 0.7, 1.0),
                   t)
    }
}


/// 射线交点需要带有的信息
pub struct HitPayload
{
    // 交点到射线起点的距离，根据射线的单位向量方向来计算
    t: f32,

    // 交点位置几何的法线，一定是单位向量；与光线方向相对的，并不是物体的实际法线
    normal: glm::Vec3,

    // 击中的交点
    p: glm::Vec3,

    // 光线是否是从外部击中物体表面
    front_face: bool,

    mat: Arc<dyn Material + Send + Sync>,

    // 交点的纹理坐标
    uv: glm::Vec2,
}


impl HitPayload
{
    /// obj_normal 是物体的实际法线，保证已经正规化
    pub fn new(ray: &Ray, t: f32, obj_normal: glm::Vec3, mat: Arc<dyn Material + Send + Sync>, uv: glm::Vec2) -> HitPayload
    {
        debug_assert!(glm::is_close_to(&glm::length(obj_normal), &1.0, 0.01));
        debug_assert!(!t.is_nan());

        if check_or(&obj_normal, f32::is_nan) {
            panic!("invalida normal");
        }

        // 调整表面法线，让法线始终与光线方向相反，使用 front_face 来记录是否是从外部击中表面
        let front_face = glm::dot(*ray.dir(), obj_normal) < 0.0;
        let normal = if front_face { obj_normal } else { -obj_normal };

        HitPayload { t, normal, p: ray.at(t), front_face, mat, uv }
    }

    /// 和光线相对的法线方向，并不是物体本身的法线方向
    pub fn normal(&self) -> &glm::Vec3 { &self.normal }

    /// 物体本身的法线方向
    pub fn obj_normal(&self) -> glm::Vec3 { if self.front_face { self.normal } else { -self.normal } }
    pub fn front_face(&self) -> bool { self.front_face }
    pub fn t(&self) -> f32 { self.t }
    pub fn material(&self) -> &Arc<dyn Material + Send + Sync> { &self.mat }
    pub fn hit_point(&self) -> &glm::Vec3 { &self.p }
    pub fn uv(&self) -> &glm::Vec2 { &self.uv }
}


/// 支持与光线求交
pub trait Hittable
{
    /// 判断是否相交，并返回相交的数据
    /// - `t_range` 表示射线的有效范围
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<HitPayload>;


    /// 获取物体的 AABB
    fn bounding_box(&self) -> Option<AABB>;
}


pub struct HittableList
{
    objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}


impl Default for HittableList
{
    fn default() -> Self {
        HittableList { objects: Vec::new() }
    }
}


impl HittableList
{
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>)
    {
        self.objects.push(object);
    }

    pub fn objects(&self) -> &[Arc<dyn Hittable + Send + Sync>] { &self.objects[..] }
}


impl Hittable for HittableList
{
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<HitPayload> {
        // 目前最近的交点
        let mut closest_so_far = t_range.1;
        let mut res: Option<HitPayload> = None;

        for object in &self.objects
        {
            if let Some(payload) = object.hit(ray, (t_range.0, closest_so_far))
            {
                closest_so_far = payload.t;
                res = Some(payload);
            }
        }

        res
    }


    fn bounding_box(&self) -> Option<AABB> {
        if self.objects.is_empty() { return None; }

        let mut aabb: AABB = AABB::new_default();

        let first_box = true;
        for obj in &self.objects {
            if let Some(obj_box) = obj.bounding_box() {
                aabb = if first_box { obj_box } else { AABB::combine(&obj_box, &aabb) };
            } else {
                return None;
            }
        }
        Some(aabb)
    }
}


#[cfg(test)]
mod test
{
    use super::*;
    use num::{Zero, One};


    #[test]
    fn t1() {
        let ray = Ray::new(glm::Vec3::zero(), glm::Vec3::one());
        // println!("{:?}", ray.at(f32::NAN));
        println!("{}, {}", 123.0 > f32::NAN, 213.0 < f32::NAN);
    }
}