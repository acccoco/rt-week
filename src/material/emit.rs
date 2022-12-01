use std::sync::Arc;
use crate::hit::HitPayload;
use crate::material::{Material, Scatter};
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};


pub struct DiffuseEmit
{
    emit: Arc<dyn Texture + Sync + Send>,
}


impl DiffuseEmit
{
    pub fn new(emit: Arc<dyn Texture + Sync + Send>) -> DiffuseEmit
    {
        DiffuseEmit { emit }
    }

    pub fn new_c(color: glm::Vec3) -> DiffuseEmit
    {
        DiffuseEmit {
            emit: Arc::new(SolidColor::new(color)),
        }
    }
}


impl Material for DiffuseEmit {
    /// 发光材料，因此不反射或者折射光线
    fn scatter(&self, _ray_in: &Ray, _hit_payload: &HitPayload) -> Option<Scatter> {
        None
    }


    fn emit(&self, uv: &glm::Vec2, p: &glm::Vec3) -> glm::Vec3 {
        self.emit.sample(uv, p)
    }
}
