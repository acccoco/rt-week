use std::ops::Deref;
use num::{One, Zero};

use crate::utility::{clone_sender};
use crate::ray::{Ray, Hittable};
use crate::camera::{Camera};
use crate::framebuffer::{FrameBuffer, Grid};
use std::sync::{Arc, mpsc};
use std::thread;
use crate::render::Background::Sky;


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
            samples: 8,
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
    fn cast_ray(&self, scene: &dyn Hittable, ray: &Ray, iter_depth: i32) -> glm::Vec3
    {
        if iter_depth <= 0 { return glm::Vec3::zero(); }

        // 注：使用 near=0.001 可以避免自身反射
        match scene.hit(ray, (0.001, f32::INFINITY)) {
            // 光线什么都没有击中，返回背景色
            None => self.background.color(ray),


            // 根据表面材质得到后续光线
            Some(payload) => {

                // 采集发光颜色
                let emit_color = payload.material().emit(payload.uv(), payload.hit_point());

                match payload.material().scatter(ray, &payload) {

                    // 光线击中了物体，但是不再有后续的散射，直接返回物体的发光色
                    None => emit_color,

                    // 还有后续的散射
                    Some((ray_out, attenuation)) => {
                        emit_color + self.cast_ray(scene, &ray_out, iter_depth - 1) * attenuation
                    }
                }
            }
        }
    }


    pub fn render_single_thread(&self, framebuffer: &mut FrameBuffer, scene: Arc<dyn Hittable + Sync + Send>, camera: &Camera)
    {
        let framebuffer_size = (framebuffer.width(), framebuffer.height());
        for pos in framebuffer.pixel_iter()
        {
            let mut pixel_color = glm::Vec3::zero();

            // multi samlpe
            for uv in FrameBuffer::multi_sample(framebuffer_size, pos, self.samples)
            {
                let ray = camera.ray_from_uv(uv);

                pixel_color = pixel_color + self.cast_ray(scene.deref(), &ray, self.max_depth);
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


    pub fn render_multi_thread(renderer: Arc<Renderer>, framebuffer: &mut FrameBuffer, scene: Arc<dyn Hittable + Sync + Send>, camera: &Camera)
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
            let camera = camera.clone();

            threads.push(thread::spawn(move || {
                for tile in task {
                    let mut tile_res: Vec<((u32, u32), glm::Vec3)> = Vec::with_capacity((tile.pos.0 * tile.pos.1) as usize);

                    for pos in tile.iter() {
                        let mut color = glm::Vec3::zero();
                        for uv in FrameBuffer::multi_sample(framebuffer_size, pos, renderer.samples) {
                            let ray = camera.ray_from_uv(uv);
                            color = color + renderer.cast_ray(scene.deref(), &ray, renderer.max_depth);
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

        for tile_res in receiver {
            for (pos, color) in tile_res
            {
                framebuffer.write_color(pos, &color);
            }
        }

        for thread in threads {
            thread.join().unwrap();
        }
    }
}