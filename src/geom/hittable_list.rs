use std::sync::Arc;
use glm::Vec3;
use rand::Rng;
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


    fn pdf(&self, _ray: &Ray) -> f32 {
        let weight = 1.0 / self.objects.len() as f32;  // 结果是 NaN 也不影响
        let mut sum = 0.0;

        for obj in &self.objects {
            sum += obj.pdf(_ray) * weight;
        }

        sum
    }

    fn rand_dir(&self, _origin: &Vec3) -> Option<(Vec3, f32)> {
        if self.objects.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        self.objects[rng.gen_range(0, self.objects.len()) as usize].rand_dir(_origin)
    }
}
