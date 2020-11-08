use crate::{PostProcessor, Scene, Vec3};
use image::{ImageBuffer, RgbImage};
use rand_distr::{Distribution, Normal};

pub struct Renderer<P: PostProcessor> {
    pub size: u32,
    pub samples: u32,
    pub num_bounces: u32,
    pub camera_size: f32,
    pub pixel_size: f32,
    pub post_process: P,
}

impl<P: PostProcessor> Renderer<P> {
    fn sample(&self, scene: &Scene, x: u32, y: u32) -> Vec3 {
        let mut rng = rand::thread_rng();
        let dist = Normal::new(0.0, self.pixel_size / 2.0).unwrap();
        let dx = dist.sample(&mut rng);
        let dy = dist.sample(&mut rng);
        let ray_dir_x = (((x as f32 + dx) / self.size as f32) - 0.5) * 2.0 * self.camera_size;
        let ray_dir_y = (((y as f32 + dy) / self.size as f32) - 0.5) * 2.0 * self.camera_size;
        let ray_dir_z = 1.0;
        let ray_dir = Vec3::new(ray_dir_x, ray_dir_y, ray_dir_z).normalized();
        scene.do_camera_ray(0.into(), ray_dir, self.num_bounces)
    }

    pub fn render(&self, scene: &Scene, filename: &str) {
        let mut buf: RgbImage = ImageBuffer::new(self.size, self.size);
        for x in 0..self.size {
            for y in 0..self.size {
                let mut color: Vec3 = 0.into();
                for _ in 0..self.samples {
                    color += self.sample(scene, x, y);
                }
                color /= self.samples as f32;
                color = self.post_process.process_pixel(color);
                let color = [
                    (color.x * 255.0) as u8,
                    (color.y * 255.0) as u8,
                    (color.z * 255.0) as u8,
                ];
                buf.put_pixel(x, y, color.into());
            }
        }
        buf.save(filename).unwrap();
    }
}
