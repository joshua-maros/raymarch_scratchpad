use crate::{Vec3, MAX_SDF_DISTANCE};
use rand::Rng;
use std::f32::consts::PI;

pub trait ImmediateLight {
    fn sample(&self, from: Vec3) -> LightSample;
}

pub struct LightSample {
    pub shadow_ray_target: Vec3,
    pub color: Vec3,
}

#[derive(Clone, Debug)]
pub struct DirectionalLight {
    pub direction: Vec3,
    /// Controls how big the light source appears in the sky, which changes how soft its shadows
    /// will appear.
    pub percent_size: f32,
    pub color: Vec3,
}

impl ImmediateLight for DirectionalLight {
    fn sample(&self, from: Vec3) -> LightSample {
        let mut rng = rand::thread_rng();
        // Since direction is normalized, vx and vy should be normalized as well.
        let (vx, vy) = self.direction.make_two_perpendicular();
        // Randomly offset the direction we are sampling in based on how big the light source is.
        let angle = rng.gen_range(0.0, 2.0 * PI);
        let offset = rng.gen_range(0.0, self.percent_size);
        let real_direction = self.direction + vx * offset * angle.cos() + vy * offset * angle.sin();
        LightSample {
            shadow_ray_target: from - real_direction * MAX_SDF_DISTANCE,
            color: self.color,
        }
    }
}
