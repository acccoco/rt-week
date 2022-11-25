use std::sync::Arc;
use num::Zero;
use crate::geom::aabb::AABB;
use crate::geom::Axis;
use crate::material::Material;
use crate::ray::{HitPayload, Hittable, Ray};


/// 轴对齐的矩形
pub struct AxisRect
{
    mat: Arc<dyn Material + Send + Sync>,
    p0: glm::Vec2,
    p1: glm::Vec2,
    k: f32,
    normal: glm::Vec3,
    idx0: usize,
    idx1: usize,
    idx_axis: usize,
}


impl AxisRect
{
    pub fn new(p0: glm::Vec2, p1: glm::Vec2, k: f32, mat: Arc<dyn Material + Send + Sync>, dir: Axis) -> AxisRect
    {
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
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<HitPayload> {
        let t = (self.k - ray.orig()[self.idx_axis]) / ray.dir()[self.idx_axis];
        if t < t_range.0 || t > t_range.1 || t.is_nan() {
            return None;
        }

        let a = ray.orig()[self.idx0] + t * ray.dir()[self.idx0];
        let b = ray.orig()[self.idx1] + t * ray.dir()[self.idx1];
        if a < self.p0[0] || a > self.p1[0] || b < self.p0[1] || b > self.p1[1] {
            return None;
        }

        let u = (a - self.p0[0]) / (self.p1[0] - self.p0[0]);
        let v = (b - self.p0[1]) / (self.p1[1] - self.p1[1]);


        Some(HitPayload::new(ray, t, ray.at(t), self.normal, self.mat.clone(), glm::vec2(u, v)))
    }

    fn bounding_box(&self) -> Option<AABB> {
        let mut min = glm::Vec3::zero() + (self.k - 0.0001);
        let mut max = glm::Vec3::zero() + (self.k + 0.0001);

        min[self.idx0] = self.p0[0];
        min[self.idx1] = self.p0[1];
        max[self.idx0] = self.p1[0];
        max[self.idx1] = self.p1[1];

        Some(AABB::new(min, max))
    }
}
