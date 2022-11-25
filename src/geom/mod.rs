use rand::Rng;


pub enum Axis
{
    X,
    Y,
    Z,
}

impl Axis
{
    #[inline(always)]
    pub fn rand() -> Axis
    {
        match rand::thread_rng().gen_range(0_i32, 3_i32) {
            0 => Self::X,
            1 => Self::Y,
            2 => Self::Z,
            _ => panic!("wrong axis"),
        }
    }
}



mod sphere;
pub mod aabb;
pub mod bvh;
pub mod rect;


pub use sphere::Sphere;
pub mod cube;

