use std::sync::Arc;
use crate::geom::aabb::AABB;
use crate::hit::{HitPayload, Hittable};
use crate::ray::Ray;
use crate::utility::check_and;


/// 在原 Hittable 物体的基础上，沿着世界坐标系的 Y 轴旋转
pub struct RotateY
{
    obj: Arc<dyn Hittable + Sync + Send>,
    sin_theta: f32,
    cos_theta: f32,
    aabb: Option<AABB>,
}


/// https://raytracing.github.io/books/RayTracingTheNextWeek.html#instances/instancerotation
impl RotateY
{
    pub fn new(obj: Arc<dyn Hittable + Sync + Send>, rotate_degree: f32) -> RotateY
    {
        debug_assert!(rotate_degree.is_finite());

        let rotate_radians = glm::radians(rotate_degree);
        let sin_theta = f32::sin(rotate_radians);
        let cos_theta = f32::cos(rotate_radians);

        let mut aabb = obj.bounding_box();

        // 如果内部的物体有 AABB，就生成新的 AABB
        if let Some(aabb) = &mut aabb {
            let mut min = glm::vec3(f32::INFINITY, f32::INFINITY, f32::INFINITY);
            let mut max = glm::vec3(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

            // 通过三重循环变量 AABB 的 8 个顶点，并分别对每个顶点进行选择变换
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = aabb.max().x * i as f32 + aabb.min().x * (1 - i) as f32;
                        let y = aabb.max().y * j as f32 + aabb.min().y * (1 - j) as f32;
                        let z = aabb.max().z * k as f32 + aabb.min().z * (1 - k) as f32;

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = glm::vec3(newx, y, newz);

                        for c in 0..3 {
                            min[c] = f32::min(min[c], tester[c]);
                            max[c] = f32::max(max[c], tester[c]);
                        }
                    }
                }
            }

            *aabb = AABB::new(min, max);
        }

        RotateY {
            obj,
            sin_theta,
            cos_theta,
            aabb,
        }
    }
}


impl Hittable for RotateY
{
    /// 先将 ray 变换到 obj 所在的坐标系中
    /// 计算 hit 后，再将 normal 等变换到世界坐标系中
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<HitPayload> {
        let mut origin = *ray.orig();
        let mut direction = *ray.dir();

        origin[0] = self.cos_theta * ray.orig()[0] - self.sin_theta * ray.orig()[2];
        origin[2] = self.sin_theta * ray.orig()[0] + self.cos_theta * ray.orig()[2];

        direction[0] = self.cos_theta * ray.dir()[0] - self.sin_theta * ray.dir()[2];
        direction[2] = self.sin_theta * ray.dir()[0] + self.cos_theta * ray.dir()[2];

        let rotated_ray = Ray::new_d(origin, direction);

        self.obj.hit(&rotated_ray, t_range).and_then(|payload| {
            let mut normal = *payload.normal();

            normal[0] = self.cos_theta * payload.normal()[0] + self.sin_theta * payload.normal()[2];
            normal[2] = -self.sin_theta * payload.normal()[0] + self.cos_theta * payload.normal()[2];
            if !payload.front_face() {
                normal = -normal;
            }

            Some(HitPayload::new(&ray, payload.t(), normal, payload.material().clone(), *payload.uv()))
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.aabb.clone()
    }
}


/// 在原 Hittable 物体的基础上，进行平移变换
pub struct Translate
{
    obj: Arc<dyn Hittable + Sync + Send>,
    offset: glm::Vec3,
}


impl Translate
{
    pub fn new(obj: Arc<dyn Hittable + Sync + Send>, offset: glm::Vec3) -> Translate
    {
        debug_assert!(check_and(&offset, f32::is_finite));

        Translate { obj, offset }
    }
}


impl Hittable for Translate
{
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<HitPayload> {
        let moved_ray = Ray::new_d(*ray.orig() - self.offset, *ray.dir());

        self.obj.hit(&moved_ray, t_range).and_then(|payload| {
            Some(HitPayload::new(&ray, payload.t(), payload.obj_normal(), payload.material().clone(), *payload.uv()))
        })
    }

    fn bounding_box(&self) -> Option<AABB>
    {
        self.obj.bounding_box().and_then(|aabb| {
            Some(AABB::new(*aabb.min() + self.offset,
                           *aabb.max() + self.offset))
        })
    }
}


/// 翻转面法线
pub struct FlipFace
{
    obj: Arc<dyn Hittable + Sync + Send>,
}

impl FlipFace
{
    pub fn new(obj: Arc<dyn Hittable + Sync + Send>) -> FlipFace
    {
        FlipFace { obj }
    }
}

impl Hittable for FlipFace
{
    fn hit(&self, ray: &Ray, t_range: (f32, f32)) -> Option<HitPayload> {
        self.obj.hit(ray, t_range).and_then(|mut payload| {
            payload.set_normal(*payload.normal(), !payload.front_face());
            Some(payload)
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.obj.bounding_box()
    }
}