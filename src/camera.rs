use crate::utility::{rand_in_unit_disk};
use crate::ray::{Ray};


#[derive(Debug, Clone)]
pub struct Camera
{
    viewport_aspect: f32,

    // viewport 左上角的坐标，用于 uv 的原点
    upper_left_corner: glm::Vec3,

    // 摄像机的位置
    pos: glm::Vec3,

    // viewport 的 uv 方向向量
    // 大小与 viewport 尺寸相同
    viewport_u: glm::Vec3,
    viewport_v: glm::Vec3,

    // 摄像机坐标系基向量，是单位向量
    camera_u: glm::Vec3,
    camera_v: glm::Vec3,
    camera_w: glm::Vec3,

    // 光圈大小，用于景深
    lens_radius: f32,
}


impl Camera {
    /// fov 是垂直方向的，单位是 degree
    /// aperture 是光圈的大小
    /// focus_dist 是景深，从相机到场景中成像清晰的位置的距离
    pub fn new(pos: glm::Vec3, lookat: glm::Vec3, up: glm::Vec3, vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Camera
    {
        // viewport 平面到摄像机的距离默认为 1.0

        // viewport 的尺寸
        let viewport_height = 2.0 * glm::tan(glm::radians(vfov / 2.0));
        let viewport_width = viewport_height * aspect_ratio;

        // 摄像机局部坐标系（右手系）的基向量
        // u：摄像机的右；w：与摄像机的朝向相反；v：上，head
        let camera_w = glm::normalize(pos - lookat);
        let camera_u = glm::normalize(glm::cross(up, camera_w));
        let camera_v = glm::cross(camera_w, camera_u);

        // viewport 基本方向对应的向量
        let viewport_u = camera_u * viewport_width * focus_dist;
        let viewport_v = -camera_v * viewport_height * focus_dist;

        // viewport 在场景中（成像清晰的位置）的投影的左上角
        let upper_left_corner = pos - viewport_u * 0.5 - viewport_v * 0.5 - camera_w * focus_dist;

        let lens_radius = aperture * 0.5;


        Camera {
            viewport_aspect: aspect_ratio,
            upper_left_corner,
            pos,
            viewport_u,
            viewport_v,
            camera_u,
            camera_v,
            camera_w,
            lens_radius,
        }
    }


    pub fn aspect(&self) -> f32 { self.viewport_aspect }

    pub fn camera_w(&self) -> &glm::Vec3 { &self.camera_w }

    /// 根据 uv 坐标，创建出对应的射线；
    /// uv 原点位于左上角，范围是 (0, 1)^2
    pub fn ray_from_uv(&self, uv: (f32, f32)) -> Ray
    {
        // 基于光圈大小，随机生成一个点
        let rd = rand_in_unit_disk() * self.lens_radius;
        let offset = self.camera_u * rd.x + self.camera_v * rd.y;

        let target = self.upper_left_corner + self.viewport_u * uv.0 + self.viewport_v * uv.1;

        Ray::new(self.pos + offset, target)
    }
}
