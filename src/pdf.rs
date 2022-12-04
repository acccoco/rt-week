use std::ops::Deref;
use std::sync::Arc;
use num::traits::FloatConst;
use crate::geom::onb::ONB;
use crate::hit::Hittable;
use crate::ray::Ray;
use crate::utility::rand_cos_dir;


/// 封装一种随机采样的策略
pub trait PDF
{
    /// 根据采样的方法，随机变量 dir 的概率密度是多少
    fn value(&self, dir: &glm::Vec3) -> f32;


    /// 根据采样策略，生成一个随机变量 dir
    fn generate(&self) -> Option<(glm::Vec3, f32)>;
}


/// 在半球空间中采样，使得立体角的概率密度等于 cos(theta)/pi
pub struct CosPDF
{
    uvw: ONB,
}


impl CosPDF
{
    pub fn new(n: glm::Vec3) -> CosPDF
    {
        CosPDF { uvw: ONB::new(n) }
    }
}


impl PDF for CosPDF
{
    fn value(&self, dir: &glm::Vec3) -> f32 {
        let cosine = glm::dot(glm::normalize(*dir), *self.uvw.w());
        debug_assert!(cosine.is_finite());
        f32::max(0.0, cosine / f32::PI())
    }

    fn generate(&self) -> Option<(glm::Vec3, f32)> {
        loop {
            let dir = self.uvw.local(&rand_cos_dir());
            let cosine = glm::dot(dir, *self.uvw.w());
            if cosine > 0.0 {
                return Some((dir, cosine / f32::PI()));
            }
        }
    }
}


/// 在物体物体上随机取一点，确保光线可以击中物体
pub struct HittablePDF<'a>
{
    obj: &'a dyn Hittable,
    p: glm::Vec3,
}


impl<'a> HittablePDF<'a>
{
    pub fn new(obj: &'a dyn Hittable, p: glm::Vec3) -> HittablePDF<'a>
    {
        HittablePDF { obj, p }
    }
}


impl<'a> PDF for HittablePDF<'a>
{
    fn value(&self, dir: &glm::Vec3) -> f32 {
        let ray = Ray::new(self.p, self.p + *dir);
        self.obj.pdf(&ray)
    }

    fn generate(&self) -> Option<(glm::Vec3, f32)> {
        self.obj.rand_dir(&self.p)
    }
}


/// 混合两个 pdf
pub struct MixPDF<'a>
{
    // 混合系数，0 表示完全采用 pdf-a，1 表示完全采用 pdf-b
    t: f32,
    pdf_a: &'a dyn PDF,
    pdf_b: &'a dyn PDF,
}


impl<'a> MixPDF<'a>
{
    pub fn new(pdf_a: &'a dyn PDF, pdf_b: &'a dyn PDF, t: f32) -> MixPDF<'a>
    {
        debug_assert!(t >= 0.0 && t <= 1.0);

        Self { t, pdf_a, pdf_b }
    }
}


impl<'a> PDF for MixPDF<'a>
{
    fn value(&self, dir: &glm::Vec3) -> f32 {
        self.pdf_a.value(dir) * self.t + self.pdf_b.value(dir) * (1.0 - self.t)
    }

    fn generate(&self) -> Option<(glm::Vec3, f32)> {
        if rand::random::<f32>() >= self.t {
            self.pdf_a.generate().and_then(|(dir, pdf)| {
                let mix_pdf = (1.0 - self.t) * pdf + self.t * self.pdf_b.value(&dir);
                Some((dir, mix_pdf))
            })
        } else {
            self.pdf_b.generate().and_then(|(dir, pdf)| {
                let mix_pdf = (1.0 - self.t) * self.pdf_a.value(&dir) + self.t * pdf;
                Some((dir, mix_pdf))
            })
        }
    }
}


