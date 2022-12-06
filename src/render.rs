use std::ops::Deref;
use num::{One, Zero};

use crate::utility::clone_sender;
use crate::ray::Ray;
use crate::camera::Camera;
use crate::framebuffer::{FrameBuffer, Grid};
use std::sync::{Arc, mpsc};
use std::thread;
use crate::hit::Hittable;
use crate::render::Background::Sky;
use crate::material::Scatter;
use crate::pdf::{HittablePDF, MixPDF, PDF};


pub enum Background
{
    Sky,
    Color(glm::Vec3),
}

impl Background
{
    fn color(&self, ray: &Ray) -> glm::Vec3
    {
        match self {
            Sky => {
                let t = 0.5 * (ray.dir().y + 1.0);
                glm::mix_s(glm::Vec3::one(), glm::vec3(0.5, 0.7, 1.0), t)
            }
            Self::Color(c) => *c,
        }
    }
}


pub struct Renderer
{
    // 每个 pixel 采样数量
    samples: u32,

    // 每根光线最大迭代深度
    max_depth: i32,

    thread_num: u32,

    tile_size: u32,

    background: Background,
}


impl Renderer
{
    pub fn new() -> Renderer
    {
        Renderer {
            samples: 32,
            max_depth: 32,
            thread_num: 8,
            tile_size: 32,
            background: Sky,
        }
    }


    pub fn set_quality(&mut self, samples: u32, max_depth: u32)
    {
        self.samples = samples;
        self.max_depth = max_depth as i32;
    }

    pub fn set_backround(&mut self, background: Background) { self.background = background }

    pub fn set_performance(&mut self, thread_num: u32, tile_size: u32)
    {
        self.thread_num = thread_num;
        self.tile_size = tile_size;
    }

    /// iter_depth 表示剩余的迭代深度，为 0 时，超出范围
    ///
    /// # 光线投射的基本过程
    ///
    /// - 没有命中：返回背景颜色
    /// - 命中：采集发光颜色，并进行下一步的光线投射
    ///      - 光线没有后续：直接返回发光色
    ///      - 递归，返回发光色 + 递归的结果
    fn cast_ray(&self, scene: &dyn Hittable, ray_in: &Ray, iter_depth: i32, lights: Option<&dyn Hittable>) -> glm::Vec3
    {
        if iter_depth <= 0 { return glm::Vec3::zero(); }

        // 注：使用 near=0.001 可以避免自身反射
        match scene.hit(ray_in, (0.001, f32::INFINITY)) {

            // 情形 1：光线什么都没有击中，返回背景色
            None => self.background.color(ray_in),

            // 情形 2：光线击中了物体
            Some(payload) => {
                // 被击中物体的自发光色
                let emit_color = payload.material().emit(ray_in, &payload);

                match payload.material().scatter(ray_in, &payload) {

                    // 情形 2-1 ：光线击中了物体，但是不再有后续的散射，直接返回物体的发光色
                    None => return emit_color,

                    // 情形 2-2：击中了物体，且还有后续的散射
                    Some(Scatter { diffuse_pdf, attenuation, specular_ray }) => {

                        // 情形 2-2-1：specular 材质，scatter 方向是确定的
                        if let Some(specular_ray) = specular_ray {
                            let scatter_color = self.cast_ray(scene, &specular_ray, iter_depth - 1, lights);
                            return attenuation * scatter_color;
                        }


                        // 情形 2-2-2：diffuse 材质，scatter 方向由 pdf 决定
                        let diffuse_pdf =
                            if let Some(diffuse_pdf) = diffuse_pdf { diffuse_pdf } else { return emit_color; };

                        // 使用混合的 pdf，由以下部分得到：
                        // - 通过符合材质的重要性采样的 mat-pdf
                        // - 以及符合光源几何的 light-pdf
                        let scatter_res =
                            if lights.is_none() {
                                diffuse_pdf.generate()
                            } else {
                                let light_pdf = HittablePDF::new(lights.unwrap(), *payload.hit_point());
                                let mix_pdf = MixPDF::new(&light_pdf, diffuse_pdf.deref(), 0.5);
                                mix_pdf.generate()
                            };

                        let (scatter_dir, monte_pdf) =
                            if let Some(val) = scatter_res { val } else { return emit_color; };
                        debug_assert!(monte_pdf > 0.0);

                        let scatter_ray = Ray::new_d(*payload.hit_point(), scatter_dir);


                        // 朝某个方向散射的 pdf，是 BRDF 的一部分
                        let scatter_pdf = payload.material().scatter_pdf(ray_in, &payload, &scatter_ray);

                        // 这里的反射方程是另一种形式的，带有 scatter pdf 项的
                        // 使用 Monte Carlo 积分计算来自散射的光照，其 pdf 可以任意选择
                        let scatter_color = self.cast_ray(scene, &scatter_ray, iter_depth - 1, lights);
                        debug_assert!(scatter_color.x >= 0.0 && scatter_color.y >= 0.0 && scatter_color.z >= 0.0);


                        emit_color + attenuation * scatter_pdf * scatter_color / monte_pdf
                    }
                }
            }
        }
    }


    pub fn render_single_thread(&self, framebuffer: &mut FrameBuffer, scene: Arc<dyn Hittable + Sync + Send>, camera: &Camera, lights: Option<Arc<dyn Hittable + Sync + Send>>)
    {
        let framebuffer_size = (framebuffer.width(), framebuffer.height());

        let lights = lights.as_ref().and_then(|val| Some(val.as_ref() as &dyn Hittable));

        for pos in framebuffer.pixel_iter()
        {
            let mut pixel_color = glm::Vec3::zero();

            // NOTE 手动条件断点
            if pos.0 == 30 && pos.1 == 56 {
                println!("accc");
                println!("bbb");
            }


            // multi samlpe
            for uv in FrameBuffer::multi_sample(framebuffer_size, pos, self.samples)
            {
                let ray = camera.ray_from_uv(uv);

                pixel_color = pixel_color + self.cast_ray(scene.deref(), &ray, self.max_depth, lights);
            }

            pixel_color = pixel_color / self.samples as f32;

            framebuffer.write_color(pos, &pixel_color);
        }
    }


    fn generate_tasks(framebuffer: &FrameBuffer, thread_num: u32, tile_size: u32) -> Vec<Vec<Grid>>
    {
        let mut tasks: Vec<Vec<Grid>> = vec![Vec::new(); thread_num as usize];

        for (tile_id, tile) in framebuffer.split_to_tile(tile_size).into_iter().enumerate() {
            tasks[tile_id % thread_num as usize].push(tile);
        }

        tasks
    }


    pub fn render_multi_thread(renderer: Arc<Renderer>, framebuffer: &mut FrameBuffer, scene: Arc<dyn Hittable + Sync + Send>, camera: &Camera, lights: Option<Arc<dyn Hittable + Sync + Send>>)
    {
        let framebuffer_size = (framebuffer.width(), framebuffer.height());
        let mut tasks = Renderer::generate_tasks(framebuffer, renderer.thread_num, renderer.tile_size);

        let (sender, receiver) = mpsc::channel();
        let mut senders = clone_sender(sender, renderer.thread_num as usize);

        let mut threads = vec![];


        for _ in 0..renderer.thread_num {
            let renderer = renderer.clone();
            let sender = senders.pop().unwrap();
            let task = tasks.pop().unwrap();
            let scene = scene.clone();
            let lights = lights.clone();
            let camera = camera.clone();

            threads.push(thread::spawn(move || {
                let lights = lights.as_ref().and_then(|val| Some(val.deref() as &dyn Hittable));

                for tile in task {
                    let mut tile_res: Vec<((u32, u32), glm::Vec3)> = Vec::with_capacity((tile.pos.0 * tile.pos.1) as usize);

                    for pos in tile.iter() {
                        let mut color = glm::Vec3::zero();
                        for uv in FrameBuffer::multi_sample(framebuffer_size, pos, renderer.samples) {
                            let ray = camera.ray_from_uv(uv);
                            color = color + renderer.cast_ray(scene.deref(), &ray, renderer.max_depth, lights);
                        }
                        color = color / renderer.samples as f32;

                        tile_res.push((pos, color));
                    }

                    sender.send(tile_res).unwrap();
                }
            }));
        }

        // 不要剩下 transmitter，否则 channel 无法关闭，一直阻塞
        debug_assert!(senders.is_empty());
        debug_assert!(tasks.is_empty());

        // 进度条
        let pb = indicatif::ProgressBar::new(
            ((framebuffer.width() + renderer.tile_size - 1) / renderer.tile_size) as u64
                * ((framebuffer.height() + renderer.tile_size - 1) / renderer.tile_size) as u64);

        for tile_res in receiver {
            for (pos, color) in tile_res
            {
                framebuffer.write_color(pos, &color);
            }
            pb.inc(1);
        }

        pb.finish_with_message("done");

        for thread in threads {
            thread.join().unwrap();
        }
    }
}