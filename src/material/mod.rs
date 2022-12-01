use num::Zero;
use crate::ray::Ray;


/// 散射结果的各种信息
pub struct Scatter {
    pub scatter_ray: Ray,
    pub albedo: glm::Vec3,

    /// 该 pdf 是随机选择的，根据该 pdf 确定散射方向，是 Monte Carlo 积分方法中的一部分
    pub monte_pdf: f32,
}


pub trait Material
{
    /// 使用 Monte Carlo 方法计算反射方程
    ///
    /// 选择合适的概率密度函数，使得结果等于：Lo = 1/n \sigma {Li * attenuation}
    ///
    /// attenuation 的表达式应该是：= fr * cos(theta) / fW，其中 fW 是关于半球方向的概率密度函数
    fn scatter(&self, _ray_in: &Ray, _hit_payload: &HitPayload) -> Option<Scatter> { None }


    /// 计算朝某个方向散射的 pdf
    ///
    /// 根据反射方程的另一种形式，可知 scatter_pdf = BRDF * cos(theta) / albedo
    fn scatter_pdf(&self, _ray_in: &Ray, _hit_payload: &HitPayload, _ray_out: &Ray) -> f32 { 0.0 }


    /// 返回发光颜色
    fn emit(&self, _uv: &glm::Vec2, _p: &glm::Vec3) -> glm::Vec3
    {
        glm::Vec3::zero()
    }
}


mod lambertian;
mod metal;
mod dielecric;
mod emit;


pub use lambertian::Lambertian;
pub use metal::Metal;
pub use dielecric::Dielecric;
pub use emit::DiffuseEmit;
use crate::hit::HitPayload;


