use crate::geom::Axis;
use crate::ray::{Hittable, Ray};


#[derive(Clone)]
pub struct AABB
{
    minimum: glm::Vec3,
    maximum: glm::Vec3,
}


impl AABB
{
    /// 提供一个默认初始化的 AABB
    pub fn new_default() -> AABB
    {
        AABB {
            minimum: glm::vec3(f32::MAX, f32::MAX, f32::MAX),
            maximum: glm::vec3(f32::MIN, f32::MIN, f32::MIN),
        }
    }


    /// 提供 AABB 中两个边界点来创建 AABB
    pub fn new(a: glm::Vec3, b: glm::Vec3) -> AABB
    {
        debug_assert!(a.x <= b.x && a.y <= b.y && a.z <= b.z);

        AABB {
            minimum: a,
            maximum: b,
        }
    }

    pub fn min(&self) -> &glm::Vec3 { &self.minimum }
    pub fn max(&self) -> &glm::Vec3 { &self.maximum }


    pub fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> bool {
        for i in 0..3 {
            // FIXME 评估 NAN 的影响
            let inv_d = 1.0 / ray.dir()[i];

            let mut t0 = (self.minimum[i] - ray.orig()[i]) * inv_d;
            let mut t1 = (self.maximum[i] - ray.orig()[i]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let t_min = if t0 > t_range.0 { t0 } else { t_range.0 };
            let t_max = if t1 < t_range.1 { t1 } else { t_range.1 };

            if t_max <= t_min {
                return false;
            }
        }

        return true;
    }


    /// 将两个 AABB 合成一个更大的 AABB
    pub fn combine(box_a: &AABB, box_b: &AABB) -> AABB
    {
        let minimum = glm::min(box_a.minimum, box_b.minimum);
        let maximum = glm::max(box_a.maximum, box_b.maximum);
        AABB { minimum, maximum }
    }


    /// 从某个方向比较两个对象的 AABB，使用 minimum 来代表 AABB 进行比较
    #[inline(always)]
    fn compare(a: &dyn Hittable, b: &dyn Hittable, axis: Axis) -> std::cmp::Ordering
    {
        let box_a = a.bounding_box().unwrap();
        let box_b = b.bounding_box().unwrap();

        let res = match axis {
            Axis::X => { box_a.minimum.x < box_b.minimum.x }
            Axis::Y => { box_a.minimum.y < box_b.minimum.y }
            Axis::Z => { box_a.minimum.z < box_b.minimum.z }
        };

        if res { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }
    }

    pub fn compare_x(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering { Self::compare(a, b, Axis::X) }
    pub fn compare_y(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering { Self::compare(a, b, Axis::Y) }
    pub fn compare_z(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering { Self::compare(a, b, Axis::Z) }
}