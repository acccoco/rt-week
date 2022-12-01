use crate::utility::{check_and, is_normalized};


#[derive(Debug)]
pub struct Ray
{
    orig: glm::Vec3,
    /// 确保该向量一定是单位向量
    dir: glm::Vec3,
}


impl Ray {
    /// 通过起点和目标点的方式来创建射线
    pub fn new(orig: glm::Vec3, target: glm::Vec3) -> Ray
    {
        let dir = glm::normalize(target - orig);
        debug_assert!(check_and(&dir, f32::is_finite));

        Ray { orig, dir }
    }

    /// dir 是方向，确保是单位向量
    pub fn new_d(orig: glm::Vec3, dir: glm::Vec3) -> Ray
    {
        debug_assert!(is_normalized(&dir));

        Ray { orig, dir }
    }


    pub fn orig(&self) -> &glm::Vec3 { &self.orig }
    pub fn dir(&self) -> &glm::Vec3 { &self.dir }


    // 射线方向上，距离原点 t 的点的坐标
    pub fn at(&self, t: f32) -> glm::Vec3
    {
        debug_assert!(t.is_finite());

        self.orig + self.dir * t
    }
}


#[cfg(test)]
mod test
{
    use super::*;
    use num::{One, Zero};


    #[test]
    fn t1() {
        let _ray = Ray::new(glm::Vec3::zero(), glm::Vec3::one());
        // println!("{:?}", ray.at(f32::NAN));
        println!("{}, {}", 123.0 > f32::NAN, 213.0 < f32::NAN);
    }
}