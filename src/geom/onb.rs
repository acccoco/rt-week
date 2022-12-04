use std::ops::Index;
use crate::utility::check_and;

/// 单位正交基
pub struct ONB {
    axis: [glm::Vec3; 3],
}


impl ONB
{
    /// 基于法向量建立局部坐标系
    pub fn new(n: glm::Vec3) -> ONB
    {
        let w = glm::normalize(n);
        debug_assert!(check_and(&w, f32::is_finite));

        let a = if w.x.abs() > 0.9 { glm::vec3(0.0, 1.0, 0.0) } else { glm::vec3(1.0, 0.0, 0.0) };
        let v = glm::normalize(glm::cross(w, a));
        let u = glm::cross(w, v);

        ONB { axis: [u, v, w] }
    }

    pub fn u(&self) -> &glm::Vec3 { &self.axis[0] }
    pub fn v(&self) -> &glm::Vec3 { &self.axis[1] }
    pub fn w(&self) -> &glm::Vec3 { &self.axis[2] }

    pub fn local(&self, v: &glm::Vec3) -> glm::Vec3
    {
        self.axis[0] * v.x + self.axis[1] * v.y + self.axis[2] * v.z
    }
}


impl Index<usize> for ONB
{
    type Output = glm::Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 3);
        &self.axis[index]
    }
}