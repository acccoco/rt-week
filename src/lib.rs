#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;


pub mod ray;
pub mod framebuffer;
pub mod utility;
pub mod camera;
pub mod geom;
pub mod render;
pub mod material;
pub mod texture;
pub mod noise;

