use crate::{Vec3, MAX_SDF_DISTANCE};

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
    pub size: f32,
    pub color: Vec3,
}

impl ImmediateLight for DirectionalLight {
    fn sample(&self, from: Vec3) -> LightSample {
        // TODO: Size.
        LightSample {
            shadow_ray_target: from - self.direction * MAX_SDF_DISTANCE,
            color: self.color,
        }
    }
}
