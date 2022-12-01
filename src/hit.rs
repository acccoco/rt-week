use std::sync::Arc;
use crate::geom::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::utility::{is_normalized};


/// 射线交点需要带有的信息
pub struct HitPayload
{
    /// 交点到射线起点的距离，根据射线的单位向量方向来计算
    t: f32,

    /// 交点位置几何的法线，一定是单位向量；与光线方向相对的，并不是物体的实际法线
    normal: glm::Vec3,

    /// 击中的交点
    p: glm::Vec3,

    /// 光线是否是从外部击中物体表面
    front_face: bool,

    mat: Arc<dyn Material + Send + Sync>,

    /// 交点的纹理坐标
    uv: glm::Vec2,
}


impl HitPayload
{
    /// obj_normal 是物体的实际法线，保证已经正规化
    pub fn new(ray: &Ray, t: f32, obj_normal: glm::Vec3, mat: Arc<dyn Material + Send + Sync>, uv: glm::Vec2) -> HitPayload
    {
        debug_assert!(is_normalized(&obj_normal));
        debug_assert!(t.is_finite());

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
    /// - `t_range` 表示射线的有效范围，是一个开区间
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<HitPayload>;


    /// 获取物体的 AABB
    fn bounding_box(&self) -> Option<AABB>;
}
