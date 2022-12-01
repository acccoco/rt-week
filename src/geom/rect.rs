use std::sync::Arc;
use num::Zero;
use crate::geom::aabb::AABB;
use crate::geom::Axis;
use crate::hit::{HitPayload, Hittable};
use crate::material::Material;
use crate::ray::Ray;


/// 轴对齐的矩形
pub struct AxisRect
{
    mat: Arc<dyn Material + Send + Sync>,

    /// 矩形的两个顶点
    p0: glm::Vec2,
    p1: glm::Vec2,

    /// 矩形在垂直方向上的位置
    k: f32,

    normal: glm::Vec3,

    /// 矩形平面的两个方向
    idx0: usize,
    idx1: usize,

    /// 矩形的垂直方向
    idx_axis: usize,
}


impl AxisRect
{
    pub fn new(p0: glm::Vec2, p1: glm::Vec2, k: f32, mat: Arc<dyn Material + Send + Sync>, dir: Axis) -> AxisRect
    {
        debug_assert!(p0.x.is_finite() && p0.y.is_finite());
        debug_assert!(p1.x.is_finite() && p1.y.is_finite());
        debug_assert!(p0[0] < p1[0] && p0[1] < p1[1]);


        let normal;
        let idx0;
        let idx1;
        let idx_axis;

        match dir {
            Axis::X => {
                normal = glm::vec3(1.0, 0.0, 0.0);
                idx0 = 1;
                idx1 = 2;
                idx_axis = 0;
            }
            Axis::Y => {
                normal = glm::vec3(0.0, 1.0, 0.0);
                idx0 = 0;
                idx1 = 2;
                idx_axis = 1;
            }
            Axis::Z => {
                normal = glm::vec3(0.0, 0.0, 1.0);
                idx0 = 0;
                idx1 = 1;
                idx_axis = 2;
            }
        }

        AxisRect {
            mat,
            p0,
            p1,
            k,
            normal,
            idx0,
            idx1,
            idx_axis,
        }
    }
}


impl Hittable for AxisRect
{
    /// 是否命中轴对齐矩形
    ///
    /// uv 的起点是 minimum 点
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<HitPayload> {
        debug_assert!(t_range.0 < t_range.1);

        let t = (self.k - ray.orig()[self.idx_axis]) / ray.dir()[self.idx_axis];
        if t <= t_range.0 || t >= t_range.1 || !t.is_finite() {
            return None;
        }

        let a = ray.orig()[self.idx0] + t * ray.dir()[self.idx0];
        let b = ray.orig()[self.idx1] + t * ray.dir()[self.idx1];
        if a < self.p0[0] || a > self.p1[0] || b < self.p0[1] || b > self.p1[1] {
            return None;
        }

        let uv = (glm::vec2(a, b) - self.p0) / (self.p1 - self.p0);
        debug_assert!(uv.x >= 0.0 && uv.y >= 0.0);

        Some(HitPayload::new(ray, t, self.normal, self.mat.clone(), uv))
    }

    fn bounding_box(&self) -> Option<AABB> {
        // 确保 AABB 是有体积的
        let mut min = glm::Vec3::zero() + (self.k - 0.0001);
        let mut max = glm::Vec3::zero() + (self.k + 0.0001);

        min[self.idx0] = self.p0[0];
        min[self.idx1] = self.p0[1];
        max[self.idx0] = self.p1[0];
        max[self.idx1] = self.p1[1];

        Some(AABB::new(min, max))
    }
}
