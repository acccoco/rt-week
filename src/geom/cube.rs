use std::sync::Arc;
use crate::geom::aabb::AABB;
use crate::geom::Axis;
use crate::geom::rect::AxisRect;
use crate::material::Material;
use crate::ray::{HitPayload, Hittable, HittableList, Ray};

pub struct Cube
{
    box_min: glm::Vec3,
    box_max: glm::Vec3,
    sides: HittableList,
}


impl Cube
{
    pub fn new(p0: glm::Vec3, p1: glm::Vec3, mat: Arc<dyn Material + Sync + Send>) -> Cube
    {
        debug_assert!(p0.x < p1.x && p0.y < p1.y && p0.z < p1.z);

        let front = AxisRect::new(glm::vec2(p0.x, p0.y), glm::vec2(p1.x, p1.y), p1.z, mat.clone(), Axis::Z);
        let back = AxisRect::new(glm::vec2(p0.x, p0.y), glm::vec2(p1.x, p1.y), p0.z, mat.clone(), Axis::Z);

        let up = AxisRect::new(glm::vec2(p0.x, p0.z), glm::vec2(p1.x, p1.z), p1.y, mat.clone(), Axis::Y);
        let down = AxisRect::new(glm::vec2(p0.x, p0.z), glm::vec2(p1.x, p1.z), p0.y, mat.clone(), Axis::Y);

        let right = AxisRect::new(glm::vec2(p0.y, p0.z), glm::vec2(p1.y, p1.z), p1.x, mat.clone(), Axis::X);
        let left = AxisRect::new(glm::vec2(p0.y, p0.z), glm::vec2(p1.y, p1.z), p0.x, mat.clone(), Axis::X);

        let mut sides = HittableList::default();
        sides.add(Arc::new(front));
        sides.add(Arc::new(back));
        sides.add(Arc::new(up));
        sides.add(Arc::new(down));
        sides.add(Arc::new(left));
        sides.add(Arc::new(right));

        Cube { box_min: p0, box_max: p1, sides }
    }
}


impl Hittable for Cube
{
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<HitPayload> {
        self.sides.hit(ray, t_range)
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }
}