use rand::Rand;

fn main()
{
    let a = f32::INFINITY;
    let b = f32::NEG_INFINITY;
    let c = f32::MAX;
    let d = f32::NAN;

    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let acc = glm::Vec3::rand(&mut rng);
        let acc = glm::normalize(acc);
        let diff = (glm::length(acc) - 1.0).abs();

        println!("{}", glm::log(0.0f32));
    }
}