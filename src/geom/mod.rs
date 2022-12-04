use rand::Rng;


#[derive(Debug)]
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
pub mod volumn;


pub use sphere::Sphere;


pub mod cube;
pub mod transform;
pub mod hittable_list;
pub mod onb;


#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_axis()
    {
        let mut x_cnt = 0;
        let mut y_cnt = 1;
        let mut z_cnt = 2;

        for _ in 0..100 {
            match Axis::rand() {
                Axis::X => { x_cnt += 1; }
                Axis::Y => { y_cnt += 1; }
                Axis::Z => { z_cnt += 1; }
            }
        }

        assert!(x_cnt > 0 && y_cnt > 0 && z_cnt > 0);
    }
}