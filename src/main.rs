use std::sync::Arc;

use rand::{Rand, Rng};
use num::Zero;

use rt_week::{camera::Camera,
              framebuffer::FrameBuffer,
              geom::Sphere,
              material::{Dielecric, Lambertian, Material, Metal},
              render::Renderer};
use rt_week::geom::Axis;
use rt_week::geom::bvh::BVHNode;
use rt_week::geom::cube::Cube;
use rt_week::geom::hittable_list::HittableList;
use rt_week::geom::rect::AxisRect;
use rt_week::geom::transform::{FlipFace, RotateY, Translate};
use rt_week::geom::volumn::ConstantMedium;
use rt_week::material::DiffuseEmit;
use rt_week::noise::NoiseTexture;
use rt_week::hit::Hittable;
use rt_week::render::Background;
use rt_week::texture::{CheckerTexture, ImageTexture};


fn main() {
    let mut renderer = Renderer::new();


    let (scene, camera) = match 5 {
        0 => random_scene(),
        1 => two_sphere(),
        2 => two_perlin_sphere(),
        3 => scene_earch(),
        4 => {
            renderer.set_backround(Background::Color(glm::Vec3::zero()));
            renderer.set_quality(64, 50);
            scene_light()
        }
        5 => {
            renderer.set_backround(Background::Color(glm::Vec3::zero()));
            renderer.set_quality(512, 50);
            cornel_box()
        }
        6 => {
            renderer.set_backround(Background::Color(glm::Vec3::zero()));
            renderer.set_quality(256, 32);
            cornel_smoke()
        }
        7 => {
            renderer.set_backround(Background::Color(glm::Vec3::zero()));
            renderer.set_quality(32, 32);
            final_scene()
        }
        _ => panic!(""),
    };

    let mut framebuffer = FrameBuffer::new(600, camera.aspect());

    let lights = Arc::new(AxisRect::new(glm::vec2(213.0, 227.0), glm::vec2(343.0, 332.0), 554.0, Arc::new(Lambertian::new(glm::Vec3::zero())), Axis::Y));


    // 开始渲染
    {
        let now = std::time::SystemTime::now();
        match 0 {
            0 => { Renderer::render_multi_thread(Arc::new(renderer), &mut framebuffer, scene, &camera, lights.clone()); }
            _ => { renderer.render_single_thread(&mut framebuffer, scene, &camera, lights.clone()); }
        }
        println!("{}", now.elapsed().unwrap().as_secs_f32());
    }

    framebuffer.save("image.ppm".to_string()).unwrap();
}


/// 创建一个随机的场景
fn random_scene() -> (Arc<dyn Hittable + Sync + Send>, Camera)
{
    let mut rng = rand::thread_rng();

    let mut scene = HittableList::default();

    // 地面
    let tex_checker = Arc::new(CheckerTexture::new_c(glm::vec3(0.2, 0.3, 0.1), glm::vec3(0.9, 0.9, 0.9)));
    let mat_ground = Arc::new(Lambertian::new_t(tex_checker));
    scene.add(Arc::new(Sphere::new(glm::vec3(0., -1000., 0.), 1000., mat_ground.clone())));


    // 随机生成一系列的小球
    for a in -11..11 {
        for b in -11..11 {
            let center = glm::vec3(a as f32 + 0.9 * rand::random::<f32>(), 0.2, b as f32 + 0.9 * rand::random::<f32>());

            let choose_mat: f32 = rand::random();
            if glm::length(center - glm::vec3(4., 0.2, 0.)) > 0.9 {
                let mat_sphere: Arc<dyn Material + Send + Sync> = match choose_mat {
                    x if x < 0.8 => {
                        // diffuse 材质
                        let albedo = glm::Vec3::rand(&mut rng) * glm::Vec3::rand(&mut rng);
                        Arc::new(Lambertian::new(albedo))
                    }
                    x if x < 0.95 => {
                        // metal 材质
                        let albedo = glm::vec3(rng.gen_range(0.5, 1.0), rng.gen_range(0.5, 1.0), rng.gen_range(0.5, 1.0));
                        let fuzz: f32 = rng.gen();
                        Arc::new(Metal::new(albedo, fuzz))
                    }
                    _ => {
                        // glass 材质
                        Arc::new(Dielecric::new(1.5))
                    }
                };

                scene.add(Arc::new(Sphere::new(center, 0.2, mat_sphere)));
            }
        }
    }


    // 生成三个大球
    let mat1 = Arc::new(Dielecric::new(1.5));
    scene.add(Arc::new(Sphere::new(glm::vec3(0., 1., 0.), 1., mat1)));

    let mat2 = Arc::new(Lambertian::new(glm::vec3(0.4, 0.2, 0.1)));
    scene.add(Arc::new(Sphere::new(glm::vec3(-4., 1., 0.), 1., mat2)));

    let mat3 = Arc::new(Metal::new(glm::vec3(0.7, 0.6, 0.5), 0.0));
    scene.add(Arc::new(Sphere::new(glm::vec3(4., 1., 0.), 1., mat3)));


    // 摄像机
    let camera =
        Camera::new(glm::vec3(13., 2., 3.), glm::vec3(0., 0., 0.),
                    glm::vec3(0., 1., 0.), 20.0, 16.0 / 9.0,
                    0.1, 10.0);

    (Arc::new(BVHNode::new_with_list(&scene)), camera)
}


/// 另一个场景
fn two_sphere() -> (Arc<dyn Hittable + Sync + Send>, Camera)
{
    let mut scene = HittableList::default();

    let tex_checker = Arc::new(CheckerTexture::new_c(glm::vec3(0.2, 0.3, 0.1), glm::vec3(0.9, 0.9, 0.9)));
    let mat_checker = Arc::new(Lambertian::new_t(tex_checker));

    scene.add(Arc::new(Sphere::new(glm::vec3(0.0, -10.0, 0.0), 10.0, mat_checker.clone())));
    scene.add(Arc::new(Sphere::new(glm::vec3(0.0, 10.0, 0.0), 10.0, mat_checker.clone())));


    // 摄像机
    let camera =
        Camera::new(glm::vec3(13., 2., 3.), glm::vec3(0., 0., 0.),
                    glm::vec3(0., 1., 0.), 20.0, 16.0 / 9.0,
                    0.0, 10.0);

    (Arc::new(BVHNode::new_with_list(&scene)), camera)
}


/// 有随机噪声纹理的球体
fn two_perlin_sphere() -> (Arc<dyn Hittable + Sync + Send>, Camera)
{
    let mut scene = HittableList::default();

    let tex_perlin = Arc::new(NoiseTexture::new(4.0));
    let mat_perlin = Arc::new(Lambertian::new_t(tex_perlin));

    scene.add(Arc::new(Sphere::new(glm::vec3(0.0, -1000.0, 0.0), 1000.0, mat_perlin.clone())));
    scene.add(Arc::new(Sphere::new(glm::vec3(0.0, 2.0, 0.0), 2.0, mat_perlin.clone())));


    // 摄像机
    let camera =
        Camera::new(glm::vec3(13., 2., 3.), glm::vec3(0., 0., 0.),
                    glm::vec3(0., 1., 0.), 20.0, 16.0 / 9.0,
                    0.0, 10.0);

    (Arc::new(BVHNode::new_with_list(&scene)), camera)
}


/// 地球的场景
fn scene_earch() -> (Arc<dyn Hittable + Sync + Send>, Camera)
{
    let mut scene = HittableList::default();

    let tex_earth = Arc::new(ImageTexture::new(&"earthmap.jpg".to_string()));
    let mat_earth = Arc::new(Lambertian::new_t(tex_earth));

    scene.add(Arc::new(Sphere::new(glm::vec3(0.0, 0.0, 0.0), 2.0, mat_earth)));

    // 摄像机
    let camera =
        Camera::new(glm::vec3(13., 2., 3.), glm::vec3(0., 0., 0.),
                    glm::vec3(0., 1., 0.), 20.0, 16.0 / 9.0,
                    0.0, 10.0);

    (Arc::new(scene), camera)
}


/// 具有灯光的场景
fn scene_light() -> (Arc<dyn Hittable + Sync + Send>, Camera)
{
    // 配置场景
    let mut scene = HittableList::default();

    let tex_noise = Arc::new(NoiseTexture::new(4.0));
    let mat_lam = Arc::new(Lambertian::new_t(tex_noise));

    let sphere1 = Sphere::new(glm::vec3(0.0, -1000.0, 0.0), 1000.0, mat_lam.clone());
    let sphere2 = Sphere::new(glm::vec3(0.0, 2.0, 0.0), 2.0, mat_lam.clone());
    scene.add(Arc::new(sphere1));
    scene.add(Arc::new(sphere2));

    let mat_diffuse_emit = Arc::new(DiffuseEmit::new_c(glm::vec3(4.0, 4.0, 4.0)));
    scene.add(Arc::new(AxisRect::new(glm::vec2(3.0, 1.0), glm::vec2(5.0, 3.0), -2.0, mat_diffuse_emit, Axis::Z)));


    // 摄像机
    let camera =
        Camera::new(glm::vec3(26., 3., 6.), glm::vec3(0., 2., 0.),
                    glm::vec3(0., 1., 0.), 20.0, 16.0 / 9.0,
                    0.0, 10.0);

    (Arc::new(scene), camera)
}


fn cornel_box() -> (Arc<dyn Hittable + Sync + Send>, Camera)
{
    // 配置场景
    let mut scene = HittableList::default();

    let mat_red = Arc::new(Lambertian::new(glm::vec3(0.65, 0.05, 0.05)));
    let mat_white = Arc::new(Lambertian::new(glm::vec3(0.73, 0.73, 0.73)));
    let mat_green = Arc::new(Lambertian::new(glm::vec3(0.12, 0.45, 0.15)));
    let mat_light = Arc::new(DiffuseEmit::new_c(glm::vec3(15.0, 15.0, 15.0)));

    // 右
    scene.add(Arc::new(AxisRect::new(glm::vec2(0.0, 0.0), glm::vec2(555.0, 555.0), 555.0, mat_green.clone(), Axis::X)));
    // 左
    scene.add(Arc::new(AxisRect::new(glm::vec2(0.0, 0.0), glm::vec2(555.0, 555.0), 0.0, mat_red.clone(), Axis::X)));
    // 灯
    let light = AxisRect::new(glm::vec2(213.0, 227.0), glm::vec2(343.0, 332.0), 554.0, mat_light.clone(), Axis::Y);
    scene.add(Arc::new(FlipFace::new(Arc::new(light))));
    // 地板
    scene.add(Arc::new(AxisRect::new(glm::vec2(0.0, 0.0), glm::vec2(555.0, 555.0), 0.0, mat_white.clone(), Axis::Y)));
    // 天花板
    scene.add(Arc::new(AxisRect::new(glm::vec2(0.0, 0.0), glm::vec2(555.0, 555.0), 555.0, mat_white.clone(), Axis::Y)));
    // 背景墙
    scene.add(Arc::new(AxisRect::new(glm::vec2(0.0, 0.0), glm::vec2(555.0, 555.0), 555.0, mat_white.clone(), Axis::Z)));


    // 两个立方体
    let box1 = Cube::new(glm::vec3(0.0, 0.0, 0.0), glm::vec3(165.0, 330.0, 165.0), mat_white.clone());
    let box1 = RotateY::new(Arc::new(box1), 15.0);
    let box1 = Translate::new(Arc::new(box1), glm::vec3(265.0, 0.0, 295.0));
    // let box1 = Cube::new(glm::vec3(265.0, 0.0, 295.0), glm::vec3(430.0, 330.0, 460.0), mat_white.clone());
    scene.add(Arc::new(box1));

    let box2 = Cube::new(glm::vec3(0.0, 0.0, 0.0), glm::vec3(165.0, 165.0, 165.0), mat_white.clone());
    let box2 = RotateY::new(Arc::new(box2), -18.0);
    let box2 = Translate::new(Arc::new(box2), glm::vec3(130.0, 0.0, 65.0));
    // let box2 = Cube::new(glm::vec3(130.0, 0.0, 65.0), glm::vec3(295.0, 165.0, 230.0), mat_white.clone());
    scene.add(Arc::new(box2));


    // 摄像机
    let camera =
        Camera::new(glm::vec3(278.0, 278.0, -800.0), glm::vec3(278.0, 278.0, 0.0),
                    glm::vec3(0., 1., 0.), 40.0, 1.0,
                    0.0, 10.0);

    (Arc::new(scene), camera)
}


fn cornel_smoke() -> (Arc<dyn Hittable + Sync + Send>, Camera)
{
    // 配置场景
    let mut scene = HittableList::default();

    let mat_red = Arc::new(Lambertian::new(glm::vec3(0.65, 0.05, 0.05)));
    let mat_white = Arc::new(Lambertian::new(glm::vec3(0.73, 0.73, 0.73)));
    let mat_green = Arc::new(Lambertian::new(glm::vec3(0.12, 0.45, 0.15)));
    let mat_light = Arc::new(DiffuseEmit::new_c(glm::vec3(7.0, 7.0, 7.0)));

    // 右
    scene.add(Arc::new(AxisRect::new(glm::vec2(0.0, 0.0), glm::vec2(555.0, 555.0), 555.0, mat_green.clone(), Axis::X)));
    // 左
    scene.add(Arc::new(AxisRect::new(glm::vec2(0.0, 0.0), glm::vec2(555.0, 555.0), 0.0, mat_red.clone(), Axis::X)));
    // 灯
    scene.add(Arc::new(AxisRect::new(glm::vec2(113.0, 127.0), glm::vec2(443.0, 432.0), 554.0, mat_light.clone(), Axis::Y)));
    // 地板
    scene.add(Arc::new(AxisRect::new(glm::vec2(0.0, 0.0), glm::vec2(555.0, 555.0), 0.0, mat_white.clone(), Axis::Y)));
    // 天花板
    scene.add(Arc::new(AxisRect::new(glm::vec2(0.0, 0.0), glm::vec2(555.0, 555.0), 555.0, mat_white.clone(), Axis::Y)));
    // 背景墙
    scene.add(Arc::new(AxisRect::new(glm::vec2(0.0, 0.0), glm::vec2(555.0, 555.0), 555.0, mat_white.clone(), Axis::Z)));


    // 两个立方体
    let box1 = Cube::new(glm::vec3(0.0, 0.0, 0.0), glm::vec3(165.0, 330.0, 165.0), mat_white.clone());
    let box1 = RotateY::new(Arc::new(box1), 15.0);
    let box1 = Translate::new(Arc::new(box1), glm::vec3(265.0, 0.0, 295.0));
    scene.add(Arc::new(ConstantMedium::new_c(Arc::new(box1), 0.01, glm::vec3(0.0, 0.0, 0.0))));


    let box2 = Cube::new(glm::vec3(0.0, 0.0, 0.0), glm::vec3(165.0, 165.0, 165.0), mat_white.clone());
    let box2 = RotateY::new(Arc::new(box2), -18.0);
    let box2 = Translate::new(Arc::new(box2), glm::vec3(130.0, 0.0, 65.0));
    scene.add(Arc::new(ConstantMedium::new_c(Arc::new(box2), 0.01, glm::vec3(1.0, 1.0, 1.0))));


    // 摄像机
    let camera =
        Camera::new(glm::vec3(278.0, 278.0, -800.0), glm::vec3(278.0, 278.0, 0.0),
                    glm::vec3(0., 1., 0.), 40.0, 1.0,
                    0.0, 10.0);

    (Arc::new(scene), camera)
}


/// 融合了各种技术的最终场景
fn final_scene() -> (Arc<dyn Hittable + Sync + Send>, Camera)
{
    let mut rng = rand::thread_rng();
    let mut scene = HittableList::default();

    // 地面由多个盒子组成
    let mut boxes1 = HittableList::default();
    let mat_ground = Arc::new(Lambertian::new(glm::vec3(0.48, 0.83, 0.53)));
    let boxes_per_sides = 20;
    for i in 0..boxes_per_sides {
        for j in 0..boxes_per_sides {
            let w = 100.0;
            let x0 = -1000.0 + w * i as f32;
            let z0 = -1000.0 + w * j as f32;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range::<f32>(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(Cube::new(glm::vec3(x0, y0, z0), glm::vec3(x1, y1, z1), mat_ground.clone())));
        }
    }
    scene.add(Arc::new(BVHNode::new_with_list(&boxes1)));


    // 光源：灯光
    let mat_light = Arc::new(DiffuseEmit::new_c(glm::vec3(7.0, 7.0, 7.0)));
    scene.add(Arc::new(AxisRect::new(glm::vec2(123.0, 147.0), glm::vec2(423.0, 412.0), 554.0, mat_light.clone(), Axis::Y)));


    // 运动模糊的球体，可惜没有运动
    let center1 = glm::vec3(400.0, 400.0, 200.0);
    let mat_sphere = Arc::new(Lambertian::new(glm::vec3(0.7, 0.3, 0.1)));
    scene.add(Arc::new(Sphere::new(center1, 50.0, mat_sphere.clone())));

    // 玻璃球
    scene.add(Arc::new(Sphere::new(glm::vec3(260.0, 150.0, 45.0), 50.0,
                                   Arc::new(Dielecric::new(1.5)))));

    // 金属球
    scene.add(Arc::new(Sphere::new(glm::vec3(0.0, 150.0, 145.0), 50.0,
                                   Arc::new(Metal::new(glm::vec3(0.8, 0.8, 0.9), 1.0)))));


    // 玻璃球内部的体积雾
    let boundary = Arc::new(Sphere::new(glm::vec3(360.0, 150.0, 145.0), 70.0,
                                        Arc::new(Dielecric::new(1.5))));
    scene.add(boundary.clone());
    scene.add(Arc::new(ConstantMedium::new_c(boundary.clone(), 0.2, glm::vec3(0.2, 0.4, 0.9))));

    // 笼罩在整个环境中的体积雾
    let boundary = Arc::new(Sphere::new(glm::vec3(0.0, 0.0, 0.0), 5000.0,
                                        Arc::new(Dielecric::new(1.5))));
    scene.add(Arc::new(ConstantMedium::new_c(boundary.clone(), 0.0001, glm::vec3(1.0, 1.0, 1.0))));

    // 地球
    let earth_mat = Arc::new(Lambertian::new_t(Arc::new(
        ImageTexture::new(&"earthmap.jpg".to_string()))));
    scene.add(Arc::new(Sphere::new(glm::vec3(400.0, 200.0, 400.0), 100.0, earth_mat.clone())));

    // 噪声纹理的球
    let tex_perlin = Arc::new(NoiseTexture::new(0.1));
    scene.add(Arc::new(Sphere::new(glm::vec3(220.0, 280.0, 300.0), 80.0,
                                   Arc::new(Lambertian::new_t(tex_perlin)))));

    // 一个立方体，里面塞满了球
    let mut boxes2 = HittableList::default();
    let mat_white = Arc::new(Lambertian::new(glm::vec3(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(glm::Vec3::rand(&mut rng) * 165.0, 10.0, mat_white.clone())));
    }
    let boxes2 = BVHNode::new_with_list(&boxes2);
    let boxes2 = Translate::new(Arc::new(boxes2), glm::vec3(-100.0, 270.0, 395.0));
    scene.add(Arc::new(boxes2));


    // 摄像机
    let camera =
        Camera::new(glm::vec3(478.0, 278.0, -600.0), glm::vec3(278.0, 278.0, 0.0),
                    glm::vec3(0., 1., 0.), 40.0, 1.0,
                    0.0, 10.0);

    (Arc::new(scene), camera)
}