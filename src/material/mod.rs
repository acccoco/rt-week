use num::Zero;
use crate::ray::{Ray, HitPayload};


pub trait Material
{
    /// 使用 Monte Carlo 方法计算反射方程
    /// 选择合适的概率密度函数，使得结果等于：Lo = 1/n \sigma {Li * attenuation}
    /// attenuation 的表达式应该是：= fr * cos(theta) / fW，其中 fW 是关于半球方向的概率密度函数
    fn scatter(&self, ray_in: &Ray, hit_payload: &HitPayload) -> Option<(Ray, glm::Vec3)>;


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


