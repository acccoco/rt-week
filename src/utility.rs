use rand::{Rand, Rng};
use std::sync::mpsc::Sender;


/// 在单位球内随机取一点
pub fn rand_in_unit_sphere() -> glm::Vec3
{
    // 一个简单的实现方法是：
    // 在 [-1, 1]^3 的正方体中取一点，判断是否在球内
    // 如果在，就接受这个点
    // 否则，重复上面的操作

    let mut rng = rand::thread_rng();

    loop {
        // glm::rand 生成的是 [0, 1]^3 的点，需要手动将点映射到 [-1, 1]^3
        let p = glm::Vec3::rand(&mut rng) * 2.0 - 1.0;

        if glm::length(p) >= 1.0 { continue; }

        return p;
    }
}


/// 在单位圆中随机取一点
pub fn rand_in_unit_disk() -> glm::Vec2
{
    let mut rng = rand::thread_rng();
    loop {
        let p = glm::vec2(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.));
        if glm::length(p) >= 1. { continue; }
        return p;
    }
}


/// 随机方向的单位向量，等价于在单位球上随机取一点
pub fn rand_unit_vec() -> glm::Vec3
{
    glm::normalize(rand_in_unit_sphere())
}


/// 将真实的颜色值进行使用 Gamma 函数进行编码
pub fn gamma_correction(color: glm::Vec3) -> glm::Vec3
{
    // 最简单的 Gamma = 2.0
    glm::sqrt(color)
}


/// 向量是否很接近零
pub fn near_zero(vec: &glm::Vec3) -> bool
{
    const S: f32 = 1e-8;

    vec.x.abs() < S && vec.y.abs() < S && vec.z.abs() < S
}


/// 检查 vec 的所有分量是否都满足某个条件
#[inline(always)]
pub fn check_and<F: Fn(f32) -> bool>(vec: &glm::Vec3, check: F) -> bool
{
    check(vec.x) && check(vec.y) && check(vec.z)
}


/// 检查 vec 是否存在某个分量满足条件
#[inline(always)]
pub fn check_or<F: Fn(f32) -> bool>(vec: &glm::Vec3, check: F) -> bool
{
    check(vec.x) || check(vec.y) || check(vec.z)
}


/// 向量是否已经正规化
pub fn is_normalized(vec: &glm::Vec3) -> bool
{
    (glm::length(*vec) - 1.0).abs() < 0.001
}


#[cfg(test)]
mod test
{
    use super::*;
    use num::Zero;

    #[quickcheck]
    fn test_rand_in_unit_sphere() -> bool
    {
        glm::length(rand_in_unit_sphere()) < 1.0
    }

    #[test]
    fn test_temp()
    {
        let acc = glm::normalize(glm::Vec3::zero());
        println!("{:#?}", acc);
        assert!(f32::is_nan(acc.x));
    }
}


/// 对于生产消费模式 channel 中的 sender，只做多份拷贝
pub fn clone_sender<T>(sender: Sender<T>, num: usize) -> Vec<Sender<T>>
{
    debug_assert!(num > 0);
    let mut res = Vec::with_capacity(num);

    for _ in 1..num {
        res.push(sender.clone());
    }
    res.push(sender);
    res
}
