use std::sync::Arc;
use crate::geom::aabb::AABB;
use crate::hit::{HitPayload, Hittable};
use crate::ray::Ray;


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
                closest_so_far = payload.t();
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
