use std::ops::Deref;
use std::sync::Arc;

use crate::geom::aabb::AABB;
use crate::geom::Axis;
use crate::ray::{HitPayload, Hittable, HittableList, Ray};


pub struct BVHNode
{
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    aabb: AABB,
}


impl BVHNode
{
    pub fn new(objects: &[Arc<dyn Hittable + Send + Sync>]) -> BVHNode
    {
        // 如果只有一个物体，那么将左右子节点都设为这个物体
        // 如果有多个物体，就随机选择一个轴进行排序，再对半分

        debug_assert!(!objects.is_empty());

        let left: Arc<dyn Hittable + Send + Sync>;
        let right: Arc<dyn Hittable + Send + Sync>;

        let comparator = match Axis::rand() {
            Axis::X => AABB::compare_x,
            Axis::Y => AABB::compare_y,
            Axis::Z => AABB::compare_z,
        };

        match objects.len() {
            1 => {
                left = objects[0].clone();
                right = objects[0].clone();
            }
            2 => {
                if comparator(objects[0].deref(), objects[1].deref()) == std::cmp::Ordering::Less {
                    left = objects[0].clone();
                    right = objects[1].clone();
                } else {
                    left = objects[1].clone();
                    right = objects[0].clone();
                }
            }
            _ => {
                let mut objects = objects.to_vec();
                objects.sort_by(|a, b| comparator(a.deref(), b.deref()));

                let mid = objects.len() / 2;
                left = Arc::new(Self::new(&mut objects[0..mid]));
                let len = objects.len();
                right = Arc::new(Self::new(&mut objects[mid..len]));
            }
        }

        let box_left = left.bounding_box().unwrap();
        let box_right = right.bounding_box().unwrap();

        BVHNode {
            left,
            right,
            aabb: AABB::combine(&box_left, &box_right),
        }
    }


    pub fn new_with_list(hittalbe_list: &HittableList) -> BVHNode
    {
        Self::new(hittalbe_list.objects())
    }
}


impl Hittable for BVHNode
{
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<HitPayload> {
        // 首先检查整体的包围盒是否击中，如果击中
        // 再依次判断左子节点和右子节点是否被击中，并选择 t 最小的作为结果

        if !self.aabb.hit(ray, t_range) {
            return None;
        }

        let left_payload = self.left.hit(ray, t_range);
        let new_range_max = if let Some(payload) = &left_payload { payload.t() } else { t_range.1 };
        let right_payload = self.right.hit(ray, (t_range.0, new_range_max));

        Option::or(right_payload, left_payload)
    }


    fn bounding_box(&self) -> Option<AABB> {
        Some(self.aabb.clone())
    }
}